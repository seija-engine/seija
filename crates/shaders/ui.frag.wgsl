struct VertexOutput {
    @location(0) uv: vec2<f32>,
    @location(1) index: u32,
    @builtin(position) position: vec4<f32>,
}

struct FragmentOutput {
    @location(0) _outColor: vec4<f32>,
}

@group(2) @binding(0)
var texture_array: binding_array<texture_2d<f32>,2>;
@group(2) @binding(1)
var Sampler: sampler;

@fragment
fn main(input: VertexOutput) -> FragmentOutput {
    var color = textureSample(texture_array[input.index], Sampler, input.uv);
    return FragmentOutput(color);
}