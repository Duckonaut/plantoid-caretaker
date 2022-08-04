//! Loads and renders a glTF file as a scene.

use std::f32::consts::PI;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{
        shape::{Icosphere, Plane},
        *,
    },
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, SamplerDescriptor, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        texture::ImageSampler,
        view::RenderLayers,
    },
    window::WindowMode,
};

#[derive(Component)]
struct Planetoid;

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
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate_light_direction)
        .add_system(sphere_scale)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let first_pass_layer = RenderLayers::layer(1);
    const HALF_SIZE: f32 = 1.0;
    commands
        .spawn_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadow_projection: OrthographicProjection {
                    left: -HALF_SIZE,
                    right: HALF_SIZE,
                    bottom: -HALF_SIZE,
                    top: HALF_SIZE,
                    near: -10.0 * HALF_SIZE,
                    far: 10.0 * HALF_SIZE,
                    ..default()
                },
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .insert(first_pass_layer);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(Icosphere {
                radius: 0.5,
                subdivisions: 0,
            })),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            ..default()
        })
        .insert(Planetoid)
        .insert(first_pass_layer);

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

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        unlit: true,
        ..Default::default()
    });

    let plane_handle = meshes.add(Mesh::from(Plane { size: 1.0 }));

    commands.spawn_bundle(PbrBundle {
        material: material_handle,
        mesh: plane_handle,
        transform: Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
        ..default()
    });

    commands
        .spawn_bundle(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::WHITE),
                ..default()
            },
            camera: Camera {
                // render before the "main pass" camera
                priority: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert(first_pass_layer);

    commands.spawn_bundle(Camera3dBundle {
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 2.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
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

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}

fn sphere_scale(time: Res<Time>, mut query: Query<&mut Transform, With<Planetoid>>) {
    for mut transform in &mut query {
        transform.scale = Vec3::new(
            1.0 + time.seconds_since_startup().sin() as f32 * 0.5,
            1.0 + time.seconds_since_startup().sin() as f32 * 0.5,
            1.0 + time.seconds_since_startup().sin() as f32 * 0.5,
        );
    }
}
