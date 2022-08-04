#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

@group(1) @binding(0)
var render_texture: texture_2d<f32>;
@group(1) @binding(1)
var render_texture_sampler: sampler;
@group(1) @binding(2)
var noise_texture: texture_2d<f32>;
@group(1) @binding(3)
var noise_texture_sampler: sampler;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let color = textureSample(render_texture, render_texture_sampler, uv);
    let noise = textureSample(noise_texture, noise_texture_sampler, floor(uv * 64.0) / 64.0);
    if (color.r == 0.0 && color.g == 0.0 && color.b == 0.0) {
        return vec4(0.0, 0.0, 0.0, 1.0);
    }
    return color;
}
