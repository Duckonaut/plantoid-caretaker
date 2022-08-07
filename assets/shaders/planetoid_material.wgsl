#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

@group(1) @binding(0)
var color_map: texture_2d<f32>;
@group(1) @binding(1)
var color_map_sampler: sampler;
@group(1) @binding(2)
var planetoid_heightmap: texture_2d<f32>;
@group(1) @binding(3)
var planetoid_heightmap_sampler: sampler;
@group(1) @binding(4)
var<uniform> planetoid: vec4<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec4<f32>,
    @location(2) position: vec4<f32>,
    @location(3) normal: vec4<f32>,
    @location(4) displacement: vec4<f32>,
};

fn rand2(n: vec2<f32>) -> f32 {
    return fract(sin(dot(n, vec2<f32>(12.9898, 4.1414))) * 43758.5453);
}

fn noise2(n: vec2<f32>) -> f32 {
    let d = vec2<f32>(0., 1.);
    let b = floor(n);
    let f = smoothstep(vec2<f32>(0.), vec2<f32>(1.), fract(n));
    return mix(mix(rand2(b), rand2(b + d.yx), f.x), mix(rand2(b + d.xy), rand2(b + d.yy), f.x), f.y);
}

@vertex
fn vertex(
    @location(0) position: vec4<f32>,
    @location(1) normal: vec4<f32>,
) -> VertexOutput {
    var polar_pos = vec2<f32>(
        atan2(position.z, position.x),
        atan2(
            sqrt(position.x * position.x + position.y * position.y),
            position.z
        )
    );
    var height = noise2(polar_pos * 2.0) * 2.0 - 1.0;
    var height = height * 2.0;
    let displacement = vec4<f32>(normalize(position.xyz) * height, height);

    var out: VertexOutput;
    out.displacement = displacement;
    out.position = position + vec4<f32>(displacement.xyz * 0.02, 0.0);
    out.normal = normal;
    out.world_normal = vec4<f32>(mesh_normal_local_to_world(normal.xyz), 1.0);
    out.world_position = mesh_position_local_to_world(mesh.model, out.position);

    out.clip_position = mesh_position_world_to_clip(out.world_position);
    return out;
}

@fragment
fn fragment(
    vertex: VertexOutput
) -> @location(0) vec4<f32> {
    let sun_pos = planetoid.xyz;
    let sun_intensity = planetoid.w;
    let light_dir = normalize(sun_pos - vertex.world_position.xyz);
    let light_intensity = sun_intensity * dot(light_dir, vertex.world_normal.xyz);
    let final_color = textureSample(color_map, color_map_sampler, vec2<f32>((light_intensity + 1.0) / 2.0, 0.5));
    let position = vertex.world_position;
//    return vec4<f32>(
//        0.5 + atan2(position.z, position.x) / 6.14,
//        acos(
//            position.y / sqrt(position.x * position.x + position.y * position.y + position.z * position.z)
//        ) / 3.14,
//        0.0,
//        1.0
//    );
    return final_color;
}
