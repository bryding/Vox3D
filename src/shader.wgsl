struct CameraUniform {
    view_proj: mat4x4<f32>,
    camera_pos: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct LightData {
    direction: vec3<f32>,
    color: vec3<f32>,
    ambient_strength: f32,
    specular_strength: f32,
};

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
    @location(3) view_position: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_position = vec4<f32>(model.position, 1.0);
    out.clip_position = camera.view_proj * world_position;
    out.world_position = model.position;
    out.world_normal = model.normal;
    out.color = model.color;
    
    // Calculate view space position for fog
    out.view_position = (camera.view_proj * world_position).xyz;
    
    return out;
}

fn fresnel(normal: vec3<f32>, view_dir: vec3<f32>, power: f32) -> f32 {
    return pow((1.0 - saturate(dot(normal, view_dir))), power);
}

fn calculate_fog(distance: f32, fog_start: f32, fog_end: f32) -> f32 {
    return smoothstep(fog_start, fog_end, distance);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Initialize light data here instead of as a constant
    let light = LightData(
        normalize(vec3<f32>(1.0, -1.0, 0.5)),  // direction
        vec3<f32>(1.0, 1.0, 0.9),              // color (slightly warm)
        0.15,                                   // ambient_strength
        0.5                                     // specular_strength
    );

    let normal = normalize(in.world_normal);
    let view_dir = normalize(camera.camera_pos - in.world_position);
    
    // Ambient light
    let ambient = light.ambient_strength * light.color;
    
    // Diffuse light
    let diff = max(dot(normal, -light.direction), 0.0);
    let diffuse = diff * light.color;
    
    // Specular light (Blinn-Phong)
    let halfway_dir = normalize(-light.direction + view_dir);
    let spec = pow(max(dot(normal, halfway_dir), 0.0), 32.0);
    let specular = light.specular_strength * spec * light.color;
    
    // Rim lighting
    let rim_power = 3.0;
    let rim_strength = 0.3;
    let rim = rim_strength * fresnel(normal, view_dir, rim_power);
    let rim_color = vec3<f32>(0.3, 0.4, 0.5);
    
    // Combine base lighting
    var result = (ambient + diffuse + specular) * in.color;
    
    // Add rim lighting
    result = result + (rim * rim_color);
    
    // Calculate fog
    let fog_color = vec3<f32>(0.6, 0.7, 0.8);
    let fog_start = 30.0;
    let fog_end = 200.0;
    let fog_amount = calculate_fog(length(in.view_position), fog_start, fog_end);
    
    // Mix final color with fog
    result = mix(result, fog_color, fog_amount);
    
    return vec4<f32>(result, 1.0);
}