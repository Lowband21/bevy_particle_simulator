// Vertex Input Structure
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

// Vertex Output Structure
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

//// Uniforms for the particle's transform matrix
//@group(0) @binding(0)
//var<uniform> u_Transform: mat4x4<f32>;

// Uniforms for particle properties such as color and size
@group(1) @binding(0)
var<uniform> u_ParticleColor: vec4<f32>;
@group(1) @binding(1)
var<uniform> u_ParticleSize: vec2<f32>;

//// Vertex Shader Function
//@vertex
//fn vertex(input: VertexInput) -> VertexOutput {
//    var output: VertexOutput;
//    output.clip_position = u_Transform * vec4<f32>(input.position, 1.0);
//    output.uv = input.uv;
//    return output;
//}

// Fragment Shader Function
@fragment
fn fragment(
    @builtin(position) coord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) normals: vec3<f32>,
    @location(2) uv: vec2<f32>
) -> @location(0) vec4<f32> {
    // Apply the particle's color to the texture color and modulate it by size
    let size_modulation: f32 = length(u_ParticleSize) / 10.0; // Example size modulation, adjust as necessary
    let color: vec4<f32> = u_ParticleColor;// * size_modulation;

    // Apply alpha blending or other particle-specific color modifications here
    // This example will just return the color as is
    return color;
}
