#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct
#import indirect_instancing::instance_struct

[[group(2), binding(0)]]
var<storage> instances: Instances;

struct Vertex {
    [[builtin(instance_index)]] instance: u32;
    [[location(0)]] vertex: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] vertex: vec3<f32>;
    [[location(2)]] normal: vec3<f32>;
};

[[stage(vertex)]]
fn vertex(in: Vertex) -> VertexOutput {
    let instance = instances.instances[in.instance];

    var out: VertexOutput;
    out.world_position = instance.transform * vec4<f32>(in.vertex, 1.0);
    out.clip_position = view.view_proj * out.world_position;
    out.vertex = in.vertex;
    out.normal = in.normal;
    return out;
}

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 0.0, 1.0, 1.0);
}
