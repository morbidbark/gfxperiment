struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct Instance {
    @location(4) pos: vec2<f32>,
    @location(5) scale: vec2<f32>,
    @location(6) color: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) vin: vec2<f32>,
    instance: Instance,
) -> VertexOutput {
    var out: VertexOutput;
    out.pos = vec4<f32>(vin, 0.0, 1.0) * vec4<f32>(instance.scale, 0.0, 1.0) + vec4<f32>(instance.pos, 0.0, 0.0);
    out.color = instance.color;
    return out;
}

@fragment
fn fs_main(vin: VertexOutput) -> @location(0) vec4<f32> {
    return vin.color;
}
