#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

@group(1) @binding(0)
var color_map_texture: texture_2d<f32>;
@group(1) @binding(1)
var color_map_sampler: sampler;
@group(1) @binding(2)
var planetoid_heightmap: texture_2d<f32>;
@group(1) @binding(3)
var planetoid_heightmap_sampler: sampler;
@group(1) @binding(4)
var<uniform> sun_pos: vec3<f32>;
@group(1) @binding(5)
var<uniform> sun_intensity: f32;

struct Vertex {
    @align(16) @location(0) position: vec3<f32>,
    @align(16) @location(1) normal: vec3<f32>,
#ifdef VERTEX_UVS
    @align(16) @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @align(16) @location(3) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @align(16) @location(4) color: vec4<f32>,
#endif
#ifdef SKINNED
    @align(16) @location(5) joint_indices: vec4<u32>,
    @align(16) @location(6) joint_weights: vec4<f32>,
#endif
};

struct VertexOutput {
    @align(16) @builtin(position) clip_position: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
    @align(16) @location(5) position: vec3<f32>,
    @align(16) @location(6) normal: vec3<f32>,
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
fn vertex(vertex: Vertex) -> VertexOutput {
    var position = vertex.position;
    var polar_pos = vec2<f32>(
        atan2(position.z, position.x),
        atan2(
            sqrt(position.x * position.x + position.y * position.y), 
            position.z
        )
    );
    var height = noise2(polar_pos * 2.0) * 2.0 - 1.0;
    height *= 0.25;

    //position = position + height * normalize(position);
    var out: VertexOutput;
    out.position = vertex.position;
    out.normal = vertex.normal;
#ifdef SKINNED
    var model = skin_model(vertex.joint_indices, vertex.joint_weights);
    out.world_normal = skin_normals(model, vertex.normal);
#else
    var model = mesh.model;
    out.world_normal = mesh_normal_local_to_world(vertex.normal);
#endif
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif
#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_tangent_local_to_world(model, vertex.tangent);
#endif
#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif

    out.clip_position = mesh_position_world_to_clip(out.world_position);
    return out;
}

@fragment
fn fragment(
    vertex: VertexOutput
) -> @location(0) vec4<f32> {
    let height = length(vertex.position - normalize(vertex.position)) * 2.0;
    let light_dir = normalize(sun_pos - vertex.world_position.xyz);
    let light_intensity = sun_intensity * dot(light_dir, vertex.world_normal);
    let final_color = textureSample(color_map_texture, color_map_sampler, vec2<f32>((light_intensity + 1.0) / 2.0, 0.5 + height));
    return final_color;
}
