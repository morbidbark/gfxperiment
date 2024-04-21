struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct Instance {
    @location(4) pos: vec2<f32>,
    @location(5) size: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> aspect_ratio: vec2<u32>;

@vertex
fn vs_main(
    @location(0) vin: vec2<f32>,
    @location(1) uv: vec2<f32>,
    instance: Instance,
) -> VertexOutput {
    var scale: vec2<f32>;
    var pos: vec2<f32>;
    var out: VertexOutput;
    scale = instance.size / vec2<f32>(aspect_ratio);
    pos = (instance.pos / vec2<f32>(aspect_ratio) * vec2<f32>(2.0, -2.0)) + vec2<f32>(-1.0, 1.0);
    out.pos = vec4<f32>(vin, 0.0, 1.0) * vec4<f32>(scale, 0.0, 1.0) + vec4<f32>(pos, 0.0, 0.0);
    out.uv = uv;
    return out;
}

@group(1) @binding(0)
var tex: texture_2d<f32>;

@group(1) @binding(1)
var samp: sampler;

@fragment
fn fs_main(vin: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(tex, samp, vin.uv);
}
