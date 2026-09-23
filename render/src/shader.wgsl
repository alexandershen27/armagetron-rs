struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    // out.clip_position = vec4<f32>(model.position, 1.0);

    // AI GENERATED, DELETE LATER
    out.clip_position = vec4<f32>(
        (model.position.x - model.position.y) / 50.0,
        ((model.position.x + model.position.y) + model.position.z * 0.5) / 50.0,
        0.5,
        1.0,
    );

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}