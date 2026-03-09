// Camera uniform buffer
struct Camera {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;


// --------------------
// Vertex shader
// --------------------

struct VertexInput {
    @location(0) position: vec3<f32>,
	@location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
	@location(1) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.color = model.color;

    // Apply camera transform
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);

	out.normal = model.normal;

    return out;
}


// --------------------
// Fragment shader
// --------------------

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let n = normalize(in.normal);
    let color = n * 0.5 + vec3<f32>(0.5); // [-1,1] -> [0,1] for coloring
    return vec4<f32>(color, 1.0);
}
