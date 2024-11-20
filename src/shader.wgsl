struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.world_position = model.position;
    out.world_normal = model.normal;
    out.color = model.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_direction = normalize(vec3<f32>(1.0, -1.0, 0.5));
    let light_color = vec3<f32>(1.0, 1.0, 0.9);  // Slightly warm light
    
    // Ambient light
    let ambient_strength = 0.1;
    let ambient = ambient_strength * light_color;

    // Diffuse light
    let normal = normalize(in.world_normal);
    let diff = max(dot(normal, -light_direction), 0.0);
    let diffuse = diff * light_color;

    // Combine lighting
    let result = (ambient + diffuse) * in.color;
    
    return vec4<f32>(result, 1.0);
}