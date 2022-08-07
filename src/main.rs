use std::{f32::consts::PI, thread::panicking};

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{shape::Plane, *},
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        primitives::Sphere,
        render_resource::{
            AsBindGroup, Extent3d, SamplerDescriptor, TextureDescriptor, TextureDimension,
            TextureFormat, TextureUsages,
        },
        texture::ImageSampler,
        view::RenderLayers,
    },
    window::WindowMode,
};
use bevy_mod_raycast::{
    ray_intersection_over_mesh, DefaultRaycastingPlugin, Ray3d, RayCastMethod, RayCastSource,
    RaycastSystem,
};
use camera::{MainCamera, MainCameraTransform};
use planetoid::{transform::cartesian_to_normalized_sphere, Planetoid, Sky, PlanetoidRotation};

mod camera;
mod creature;
mod planetoid;

pub struct GameWorldRenderLayer(RenderLayers);
pub(crate) struct PlanetoidRaycastSet;

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
        .insert_resource(GameWorldRenderLayer(RenderLayers::layer(1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(DefaultRaycastingPlugin::<PlanetoidRaycastSet>::default())
        .add_system_to_stage(
            CoreStage::First,
            update_raycast_with_cursor.before(RaycastSystem::BuildRays::<PlanetoidRaycastSet>),
        )
        .add_system(set_creature_target)
        .add_plugin(MaterialPlugin::<PostProcessMaterial>::default())
        .add_plugin(planetoid::PlanetoidPlugin)
        .add_plugin(camera::MainCameraPlugin)
        .add_plugin(creature::CreaturePlugin)
        .add_startup_system(setup_dpass)
        .add_startup_system(setup_msaa)
        .add_system(update_postprocess)
        .add_system(make_images_nearest_filtered)
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
    let mut image = create_render_texture(size);

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    let material_handle = materials.add(PostProcessMaterial {
        render_texture: image_handle.clone(),
        noise_texture: assets.load("textures/noise.png"),
        orientation: Mat4::IDENTITY,
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
                target: RenderTarget::Image(image_handle),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -5.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert(game_world_render_layer.0)
        .insert(RayCastSource::<PlanetoidRaycastSet>::new())
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

#[cfg(target_arch = "wasm32")]
fn create_render_texture(size: Extent3d) -> Image {
    Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
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
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn create_render_texture(size: Extent3d) -> Image {
    Image {
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
    }
}

fn make_images_nearest_filtered(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
    materials: Res<Assets<StandardMaterial>>,
    query: Query<&Handle<StandardMaterial>, With<Sky>>,
) {
    for event in ev_asset.iter() {
        if let AssetEvent::Created { handle } = event {
            if query.iter().any(|sm| {
                materials
                    .get(sm)
                    .unwrap()
                    .base_color_texture
                    .as_ref()
                    .or(None)
                    == Some(handle)
            }) {
                bevy::log::info!("making image nearest filtered: {:?}", handle);
                let image = images.get_mut(handle).unwrap();
                image.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
                    mag_filter: bevy::render::render_resource::FilterMode::Nearest,
                    min_filter: bevy::render::render_resource::FilterMode::Nearest,
                    ..default()
                });
            }
        }
    }
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
    #[uniform(4)]
    pub orientation: Mat4,
}

impl Material for PostProcessMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/postprocess_material.wgsl".into()
    }
}

fn update_postprocess(
    cam_transform: Res<MainCameraTransform>,
    mut materials: ResMut<Assets<PostProcessMaterial>>,
    query: Query<&Handle<PostProcessMaterial>>,
) {
    for handle in query.iter() {
        let mat = &mut materials.get_mut(handle);

        if let Some(mat) = mat {
            mat.orientation = cam_transform.value;
        }
    }
}

fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<PlanetoidRaycastSet>>,
) {
    for mut pick_source in &mut query {
        if let Some(cursor_latest) = cursor.iter().last() {
            pick_source.cast_method = RayCastMethod::Screenspace(cursor_latest.position);
        }
    }
}

fn set_creature_target(
    buttons: Res<Input<MouseButton>>,
    planetoid_rotation: Res<PlanetoidRotation>,
    mut target: ResMut<creature::CreatureTarget>,
    camera: Query<(&Transform, &Camera), With<MainCamera>>,
    planetoid: Query<(&Handle<Mesh>, &Transform), With<Planetoid>>,
    meshes: Res<Assets<Mesh>>,
) {
    if buttons.pressed(MouseButton::Left) {
        let (camera_transform, camera) = camera.iter().next().unwrap();

        let ray = Ray3d::new(
            camera_transform.translation,
            camera_transform.rotation * Vec3::new(0.0, 0.0, -1.0),
        );
        bevy::log::info!("ray source: {:?}", ray.origin());
        bevy::log::info!("ray direction: {:?}", ray.direction());

        let (planetoid_handle, planetoid_transform) = planetoid.single();

        let mesh = meshes.get(planetoid_handle).unwrap();

        let intersection = ray_intersection_over_mesh(
            mesh,
            &planetoid_transform.compute_matrix(),
            &ray,
            bevy_mod_raycast::Backfaces::Cull,
        );

        if let Some(intersection) = intersection {
            let pos = intersection.position();
            bevy::log::info!("creature target: {:?}", pos);

            let target_on_planetoid = planetoid_rotation.0.inverse() * pos;
            bevy::log::info!("creature target on planetoid: {:?}", target_on_planetoid);

            let sphere_pos = cartesian_to_normalized_sphere(target_on_planetoid);
            bevy::log::info!("creature target sphere: {:?}", sphere_pos);

            target.target = Some(sphere_pos);
        }
    }
}
