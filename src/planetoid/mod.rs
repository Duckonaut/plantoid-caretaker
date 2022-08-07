use std::f32::consts::PI;

use bevy::{prelude::*, render::render_resource::Face};
use bevy_mod_raycast::RayCastMesh;

use crate::{GameWorldRenderLayer, PlanetoidRaycastSet, Res};

use self::{
    rendering::{update_material_sun_pos, PlanetoidMaterial},
    transform::match_planetoid_transforms,
};

mod rendering;
pub mod transform;

pub struct PlanetoidPlugin;

impl Plugin for PlanetoidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlanetoidRotation(Quat::IDENTITY))
            .add_plugin(MaterialPlugin::<PlanetoidMaterial>::default())
            .add_startup_system(setup_planetoid)
            .add_startup_system(setup_sun)
            .add_startup_system(setup_sky)
            .add_system(update_material_sun_pos)
            .add_system(match_planetoid_transforms)
            .add_system(set_planetoid_rotation)
            .add_system(planetoid_rotation)
            .add_system(update_sun);
    }
}

#[derive(Component)]
pub(crate) struct Planetoid;

#[derive(Component)]
pub(crate) struct Sun;

#[derive(Component)]
pub(crate) struct Sky;

pub(crate) struct PlanetoidRotation(pub(crate) Quat);

fn planetoid_rotation(time: Res<Time>, mut rotation: ResMut<PlanetoidRotation>) {
    *rotation = PlanetoidRotation(
        rotation.0
            * Quat::from_axis_angle(
                Vec3::new(1.0, 1.0, 1.0).normalize(),
                0.1 * time.delta_seconds(),
            ),
    );
}

fn set_planetoid_rotation(
    rotation: Res<PlanetoidRotation>,
    mut query: Query<&mut Transform, With<Planetoid>>,
) {
    for mut transform in &mut query {
        transform.rotation = rotation.0;
    }
}

fn setup_planetoid(
    mut commands: Commands,
    game_world_render_layer: Res<GameWorldRenderLayer>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<PlanetoidMaterial>>,
) {
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
        .insert(game_world_render_layer.0);

    let color_ramp: Handle<Image> = asset_server.load("textures/planet_color.png");

    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: asset_server.load("models/planetoid.gltf#Mesh0/Primitive0"),
            material: materials.add(PlanetoidMaterial {
                color_ramp,
                heightmap: asset_server.load("textures/planet_height.png"),
                sun_info: Vec4::new(0.0, 10.0, 0.0, 1.0),
            }),
            ..default()
        })
        .insert(Planetoid)
        .insert(game_world_render_layer.0)
        .insert(RayCastMesh::<PlanetoidRaycastSet>::default());
}

fn setup_sun(
    mut commands: Commands,
    game_world_render_layer: Res<GameWorldRenderLayer>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: asset_server.load("models/sun.glb#Mesh0/Primitive0"),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.9, 0.3),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(0.5)),
            ..default()
        })
        .insert(Sun)
        .insert(game_world_render_layer.0);
}

fn update_sun(time: Res<Time>, mut query: Query<&mut Transform, With<Sun>>) {
    for mut transform in &mut query {
        let pos = Mat4::from_translation(Vec3::new(5.0, 0.0, 0.0));
        let rot_y = Mat4::from_rotation_y(time.seconds_since_startup() as f32 * 0.25);
        let rot_z = Mat4::from_rotation_z(PI / 4.0);
        let new_transform = rot_z * rot_y * pos;

        *transform = transform.with_translation(new_transform.to_scale_rotation_translation().2);
    }
}

fn setup_sky(
    mut commands: Commands,
    game_world_render_layer: Res<GameWorldRenderLayer>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: asset_server.load("models/sky.glb#Mesh0/Primitive0"),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("textures/sky.png")),
                cull_mode: Some(Face::Front),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(10.0)),
            ..default()
        })
        .insert(Sky)
        .insert(game_world_render_layer.0);
}
