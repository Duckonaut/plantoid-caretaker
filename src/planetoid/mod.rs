use bevy::{prelude::*, render::view::RenderLayers};

use crate::{GameWorldRenderLayer, Res};

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
            .add_system(update_material_sun_pos)
            .add_system(match_planetoid_transforms)
            .add_system(set_planetoid_rotation)
            .add_system(planetoid_rotation);
    }
}

#[derive(Component)]
pub(crate) struct Planetoid;

pub(crate) struct PlanetoidRotation(Quat);

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

    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: asset_server.load("models/planetoid.gltf#Mesh0/Primitive0"),
            material: materials.add(PlanetoidMaterial {
                color_ramp: asset_server.load("textures/planet_color.png"),
                heightmap: asset_server.load("textures/planet_height.png"),
                sun_pos: Vec3::new(0.0, 10.0, 0.0),
                sun_intensity: 1.0,
            }),
            ..default()
        })
        .insert(Planetoid)
        .insert(game_world_render_layer.0);
}
