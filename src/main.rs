//! Loads and renders a glTF file as a scene.

use std::f32::consts::PI;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{shape::Plane, *},
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        render_resource::{
            AsBindGroup, Extent3d, SamplerDescriptor, TextureDescriptor, TextureDimension,
            TextureFormat, TextureUsages,
        },
        texture::ImageSampler,
        view::RenderLayers,
    },
    window::WindowMode,
};
use camera::MainCamera;

mod camera;
mod creature;
mod planetoid;

pub struct GameWorldRenderLayer(RenderLayers);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 64.0,
            height: 64.0,
            scale_factor_override: Some(8.0),
            title: "Plantoid Caretaker".to_string(),
            resizable: false,
            cursor_visible: true,
            cursor_locked: false,
            mode: WindowMode::Windowed,
            ..default()
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(GameWorldRenderLayer(RenderLayers::layer(1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<PostProcessMaterial>::default())
        .add_plugin(planetoid::PlanetoidPlugin)
        .add_plugin(camera::MainCameraPlugin)
        .add_plugin(creature::CreaturePlugin)
        .add_startup_system(setup_dpass)
        .add_startup_system(setup_msaa)
        .run();
}

fn setup_dpass(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PostProcessMaterial>>,
    mut images: ResMut<Assets<Image>>,
    assets: Res<AssetServer>,
    game_world_render_layer: Res<GameWorldRenderLayer>,
) {
    let size = Extent3d {
        width: 64,
        height: 64,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        sampler_descriptor: ImageSampler::Descriptor(SamplerDescriptor {
            mag_filter: bevy::render::render_resource::FilterMode::Nearest,
            min_filter: bevy::render::render_resource::FilterMode::Nearest,
            ..default()
        }),
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    let material_handle = materials.add(PostProcessMaterial {
        render_texture: image_handle.clone(),
        noise_texture: assets.load("textures/noise.png"),
    });

    let plane_handle = meshes.add(Mesh::from(Plane { size: 1.0 }));

    commands.spawn_bundle(MaterialMeshBundle {
        material: material_handle,
        mesh: plane_handle,
        transform: Transform::from_rotation(
            Quat::from_rotation_z(PI) * Quat::from_rotation_x(PI / 2.0),
        ),
        ..default()
    });

    commands
        .spawn_bundle(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            camera: Camera {
                // render before the "main pass" camera
                priority: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -5.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert(game_world_render_layer.0)
        .insert(MainCamera);

    commands.spawn_bundle(Camera3dBundle {
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 2.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        projection: bevy::render::camera::Projection::Orthographic(OrthographicProjection {
            left: -0.5,
            right: 0.5,
            bottom: -0.5,
            top: 0.5,
            scaling_mode: bevy::render::camera::ScalingMode::None,
            ..default()
        }),
        ..default()
    });
}

fn setup_msaa(mut msaa: ResMut<Msaa>) {
    msaa.samples = 1;
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Component)]
#[uuid = "1e55b055-f4c4-c1c2-d1d2-d3d4d5d6d7d8"]
pub struct PostProcessMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub render_texture: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub noise_texture: Handle<Image>,
}

impl Material for PostProcessMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/postprocess_material.wgsl".into()
    }
}
