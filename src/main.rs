use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin};
use bevy::time::Timer;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng}; // Import SeedableRng trait // Make sure to import StdRng

// Define the Particle component without color as it will be part of the material
#[derive(Component)]
struct Particle {
    velocity: Vec3,
    acceleration: Vec3,
    lifespan: f32, // in seconds
}

// A material component for rendering particles, now includes size
#[derive(AsBindGroup, TypeUuid, Clone, Component, Asset, TypePath)]
#[uuid = "d7b9f456-5eab-4b3c-91cc-7b9a3f8f2a6b"]
struct ParticleMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(1)]
    size: Vec2, // added size field
}

// Implement the plugin for the ParticleMaterial.
impl Material2d for ParticleMaterial {
    fn fragment_shader() -> ShaderRef {
        // Here you would return a custom shader that knows how to render your particles, or use a default one.
        "shaders/particle.wgsl".into()
    }
}

#[derive(Resource)]
struct RngResource(StdRng);

// Define the Bevy app main entry point.
fn main() {
    App::new()
        .insert_resource(RngResource(StdRng::from_entropy())) // Add RNG resource
        .add_plugins(DefaultPlugins)
        .add_plugins(Material2dPlugin::<ParticleMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, particle_spawner_system)
        .add_systems(Update, particle_update_system)
        .add_systems(Update, particle_render_system)
        .run();
}

// Setup function that adds a camera and other resources.
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;

// Your existing Particle and ParticleMaterial definitions...

// Utility function to create a quad mesh for the particle
fn create_particle_mesh(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> Mesh2dHandle {
    let mesh = Mesh::from(shape::Quad {
        size: Vec2::new(10.0, 10.0), // Size of the particle
        flip: false,
    });
    Mesh2dHandle(meshes.add(mesh))
}

// Utility function to create a particle material now also includes size
fn create_particle_material(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ParticleMaterial>>,
    color: Color, // now we pass color as a parameter
    size: Vec2,   // now we pass size as a parameter
) -> Handle<ParticleMaterial> {
    materials.add(ParticleMaterial { color, size })
}

// Constants for particle spawning
const NUM_PARTICLES_TO_SPAWN: usize = 10; // Number of particles to spawn each time
const PARTICLE_SIZE: Vec2 = Vec2::new(10.0, 10.0); // Size of the particles
const PARTICLE_LIFESPAN: f32 = 5.0; // Lifespan of particles in seconds

fn particle_spawner_system(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: Local<Timer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParticleMaterial>>,
    mut rng: ResMut<RngResource>, // Access StdRng as a mutable resource
) {
    if spawn_timer.tick(time.delta()).just_finished() {
        let particle_mesh = create_particle_mesh(&mut commands, &mut meshes);

        for _ in 0..NUM_PARTICLES_TO_SPAWN {
            let random_velocity = Vec3::new(
                rng.0.gen_range(-1.0..1.0), // Random x velocity
                rng.0.gen_range(1.0..3.0),  // Random y velocity, always upwards
                0.0,
            );
            let random_color_offset = rng.0.gen_range(0.0..0.2); // Slight color variation
            let particle_color =
                Color::rgb(1.0 - random_color_offset, 1.0 - random_color_offset, 1.0);

            let particle_material = create_particle_material(
                &mut commands,
                &mut materials,
                particle_color,
                PARTICLE_SIZE,
            );

            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: particle_mesh.clone(), // Use clone to use the same mesh handle for all particles
                    material: particle_material,
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                    ..Default::default()
                })
                .insert(Particle {
                    velocity: random_velocity,
                    acceleration: Vec3::new(0.0, -9.8, 0.0),
                    lifespan: PARTICLE_LIFESPAN,
                });
        }
    }
}

// Particle update system to update particle state each frame.
fn particle_update_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Transform), Without<Despawned>>,
) {
    for (entity, mut particle, mut transform) in query.iter_mut() {
        // Update particle position and decrease lifespan.
        let acc = particle.acceleration;
        transform.translation += particle.velocity.clone() * time.delta_seconds();
        particle.velocity += acc * time.delta_seconds();
        particle.lifespan -= time.delta_seconds();

        // Despawn if lifespan is over.
        if particle.lifespan <= 0.0 {
            // The entity can be despawned here
            commands.entity(entity).insert(Despawned);
        }
    }
}

// Marker component for despawned entities to avoid despawning twice
#[derive(Component)]
struct Despawned;

// System to actually remove despawned entities
fn despawn_system(mut commands: Commands, query: Query<Entity, With<Despawned>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// Particle render system to render particles. In this simplified version, we're just using Bevy's sprite rendering.
fn particle_render_system(mut query: Query<(&Particle, &mut ParticleMaterial)>) {
    for (particle, mut material) in query.iter_mut() {
        // Update the particle's material color or other properties for rendering if needed.
    }
}
