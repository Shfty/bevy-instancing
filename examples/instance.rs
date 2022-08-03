use bevy::{
    core::Name,
    math::{Quat, Vec3},
    pbr::{AlphaMode, DirectionalLight, DirectionalLightBundle},
    prelude::{
        default,
        shape::{Cube, Icosphere, Quad, Torus, UVSphere},
        App, AssetServer, Assets, Camera, Camera3dBundle, Color, Commands, EventWriter, Handle,
        Mesh, PerspectiveProjection, Res, ResMut, SpatialBundle, Transform,
    },
    render::{
        camera::{Projection, RenderTarget},
        render_resource::Face,
    },
    window::{CreateWindow, PresentMode, WindowDescriptor, WindowId},
    DefaultPlugins,
};

use bevy_instancing::prelude::{
    BasicMaterial, BasicMaterialPlugin, ColorInstanceBundle, CustomMaterial, CustomMaterialPlugin,
    IndirectRenderingPlugin, MeshInstanceBundle, TextureMaterial, TextureMaterialPlugin,
};
const USE_SECOND_CAMERA: bool = false;

// Test indirect rendering
fn main() {
    let mut app = App::default();

    app.insert_resource(bevy::render::settings::WgpuSettings {
        disabled_features: Some(wgpu::Features::INDIRECT_FIRST_INSTANCE),
        constrained_limits: Some(wgpu::Limits {
            max_storage_buffers_per_shader_stage: 0,
            max_uniform_buffer_binding_size: 16 << 10,
            ..default()
        }),
        ..default()
    });

    app.add_plugins(DefaultPlugins)
        .add_plugin(IndirectRenderingPlugin)
        .add_plugin(BasicMaterialPlugin)
        .add_plugin(CustomMaterialPlugin)
        .add_plugin(TextureMaterialPlugin);

    app.add_startup_system(setup_instancing);

    app.run()
}

fn setup_instancing(
    asset_server: Res<AssetServer>,
    mut create_window_events: EventWriter<CreateWindow>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut board_materials: ResMut<Assets<CustomMaterial>>,
    mut texture_materials: ResMut<Assets<TextureMaterial>>,
    mut commands: Commands,
) {
    // Directional Light
    commands.spawn().insert_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 4000.,
            ..default()
        },
        transform: Transform {
            // Workaround: Pointing straight up or down prevents directional shadow from rendering
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2 * 0.6),
            ..default()
        },
        ..default()
    });

    // main camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-50.0, 50.0, 50.0)
            .looking_at(Vec3::new(0.0, 12.0, 0.0), Vec3::Y),
        projection: Projection::Perspective(PerspectiveProjection {
            fov: 15.0f32.to_radians(),
            ..default()
        }),
        ..default()
    });

    if USE_SECOND_CAMERA {
        let window_id = WindowId::new();

        // sends out a "CreateWindow" event, which will be received by the windowing backend
        create_window_events.send(CreateWindow {
            id: window_id,
            descriptor: WindowDescriptor {
                width: 800.,
                height: 600.,
                present_mode: PresentMode::AutoNoVsync,
                title: "Second window".to_string(),
                ..default()
            },
        });

        // second window camera
        commands.spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(50.0, 0.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                target: RenderTarget::Window(window_id),
                ..default()
            },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 15.0f32.to_radians(),
                ..default()
            }),
            ..default()
        });
    }

    // Populate scene
    let mesh_cube = meshes.add(Cube::default().into());
    let mesh_quad = meshes.add(Quad::default().into());

    let mesh_icosphere = meshes.add(
        Icosphere {
            radius: 0.5,
            ..default()
        }
        .into(),
    );

    let mesh_uv_sphere = meshes.add(
        UVSphere {
            radius: 0.5,
            ..default()
        }
        .into(),
    );

    let mesh_torus = meshes.add(
        Torus {
            radius: 0.25 + 0.125,
            ring_radius: 0.125,
            ..default()
        }
        .into(),
    );

    let meshes = [
        mesh_cube,
        mesh_quad,
        mesh_icosphere,
        mesh_uv_sphere,
        mesh_torus,
    ];

    let material_basic = Handle::<BasicMaterial>::default();

    let material_opaque_no_cull = board_materials.add(CustomMaterial {
        alpha_mode: AlphaMode::Opaque,
        cull_mode: None,
    });

    let material_mask_no_cull = board_materials.add(CustomMaterial {
        alpha_mode: AlphaMode::Mask(0.5),
        cull_mode: None,
    });

    let material_blend_no_cull = board_materials.add(CustomMaterial {
        alpha_mode: AlphaMode::Blend,
        cull_mode: None,
    });

    let material_opaque_cull_front = board_materials.add(CustomMaterial {
        alpha_mode: AlphaMode::Opaque,
        cull_mode: Some(Face::Front),
    });

    let material_mask_cull_front = board_materials.add(CustomMaterial {
        alpha_mode: AlphaMode::Mask(0.5),
        cull_mode: Some(Face::Front),
    });

    let material_blend_cull_front = board_materials.add(CustomMaterial {
        alpha_mode: AlphaMode::Blend,
        cull_mode: Some(Face::Front),
    });

    let material_opaque_cull_back = board_materials.add(CustomMaterial {
        alpha_mode: AlphaMode::Opaque,
        cull_mode: Some(Face::Back),
    });

    let material_mask_cull_back = board_materials.add(CustomMaterial {
        alpha_mode: AlphaMode::Mask(0.5),
        cull_mode: Some(Face::Back),
    });

    let material_blend_cull_back = board_materials.add(CustomMaterial {
        alpha_mode: AlphaMode::Blend,
        cull_mode: Some(Face::Back),
    });

    let custom_materials: &[Handle<CustomMaterial>] = &[
        material_opaque_no_cull,
        material_mask_no_cull,
        material_blend_no_cull,
        material_opaque_cull_front,
        material_mask_cull_front,
        material_blend_cull_front,
        material_opaque_cull_back,
        material_mask_cull_back,
        material_blend_cull_back,
    ];

    let material_texture_1 = texture_materials.add(TextureMaterial {
        texture: asset_server.load("texture/text_1.png"),
        alpha_mode: AlphaMode::Opaque,
        cull_mode: Some(Face::Back),
    });

    let material_texture_2 = texture_materials.add(TextureMaterial {
        texture: asset_server.load("texture/text_2.png"),
        alpha_mode: AlphaMode::Mask(0.2),
        cull_mode: Some(Face::Back),
    });

    let material_texture_3 = texture_materials.add(TextureMaterial {
        texture: asset_server.load("texture/text_3.png"),
        alpha_mode: AlphaMode::Blend,
        cull_mode: Some(Face::Back),
    });

    let material_texture_smiley = texture_materials.add(TextureMaterial {
        texture: asset_server.load("texture/text_smiley.png"),
        alpha_mode: AlphaMode::Opaque,
        cull_mode: Some(Face::Back),
    });

    let texture_materials = &[
        material_texture_1,
        material_texture_2,
        material_texture_3,
        material_texture_smiley,
    ];

    let colors = (0..24)
        .into_iter()
        .map(|i| i as f32 / 16.0)
        .map(|i| Color::hsla(i * 360.0, 1.0, 0.5, 0.5))
        .collect::<Vec<_>>();

    for (x, mesh) in meshes.into_iter().enumerate() {
        commands
            .spawn()
            .insert(Name::new("Basic Instance"))
            .insert_bundle(MeshInstanceBundle::<BasicMaterial> {
                mesh: mesh.clone(),
                material: material_basic.clone(),
                spatial_bundle: SpatialBundle {
                    transform: Transform::from_xyz(x as f32 * 1.5, 0.0, 0.0).into(),
                    ..default()
                },
                ..default()
            });

        for (z, color) in colors.iter().copied().enumerate() {
            let mut y = 0;
            for material in custom_materials.iter() {
                commands
                    .spawn()
                    .insert(Name::new(format!("Custom Instance ({x:}, {y:}, {z:})")))
                    .insert_bundle(ColorInstanceBundle {
                        instance_bundle: MeshInstanceBundle {
                            mesh: mesh.clone(),
                            material: material.clone(),
                            spatial_bundle: SpatialBundle {
                                transform: Transform::from_xyz(
                                    x as f32 * 1.5,
                                    1.5 + y as f32 * 1.5,
                                    z as f32 * -1.5,
                                )
                                .into(),
                                ..default()
                            },
                            ..default()
                        },
                        mesh_instance_color: color.into(),
                    });

                y += 1;
            }

            for material in texture_materials.iter() {
                commands
                    .spawn()
                    .insert(Name::new(format!("Texture Instance ({x:}, {y:}, {z:})")))
                    .insert_bundle(ColorInstanceBundle {
                        instance_bundle: MeshInstanceBundle {
                            mesh: mesh.clone(),
                            material: material.clone(),
                            spatial_bundle: SpatialBundle {
                                transform: Transform::from_xyz(
                                    x as f32 * 1.5,
                                    1.5 + y as f32 * 1.5,
                                    z as f32 * -1.5,
                                )
                                .into(),
                                ..default()
                            },
                            ..default()
                        },
                        mesh_instance_color: color.into(),
                    });

                y += 1;
            }
        }
    }
}
