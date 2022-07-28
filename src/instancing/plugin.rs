use bevy::{
    asset::load_internal_asset,
    prelude::{AddAsset, App, Assets, Handle, HandleUntyped, Plugin, Shader},
    reflect::TypeUuid,
    render::{render_component::ExtractComponentPlugin, RenderApp},
};

use crate::prelude::{
    BasicMaterial, InstanceBlock, InstancedMaterialPlugin, InstancedMeshPipeline, MeshInstanceColor,
};

use bevy::asset as bevy_asset;

pub const INSTANCED_MESH_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 7051817732463169032);

pub const INSTANCE_STRUCT_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 14563515845427599203);

pub const INDIRECT_STRUCT_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 7281773422344927676);

/// Plugin encapsulating instanced mesh rendering
pub struct IndirectRenderingPlugin;

impl Plugin for IndirectRenderingPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            INSTANCED_MESH_SHADER_HANDLE,
            "render/shaders/instanced_mesh.wgsl",
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            INSTANCE_STRUCT_HANDLE,
            "render/shaders/instance_struct.wgsl",
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            INDIRECT_STRUCT_HANDLE,
            "render/shaders/indirect_struct.wgsl",
            Shader::from_wgsl
        );

        app.register_type::<MeshInstanceColor>()
            .register_type::<InstanceBlock>();

        app.add_plugin(ExtractComponentPlugin::<InstanceBlock>::default());

        app.sub_app_mut(RenderApp)
            .init_resource::<InstancedMeshPipeline>();

        // Material
        app.add_asset::<BasicMaterial>()
            .add_plugin(InstancedMaterialPlugin::<BasicMaterial>::default());

        app.world
            .resource_mut::<Assets<BasicMaterial>>()
            .set_untracked(Handle::<BasicMaterial>::default(), BasicMaterial::default());
    }
}
