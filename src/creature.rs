use bevy::prelude::*;

use crate::{planetoid::transform::PlanetoidTransform, GameWorldRenderLayer};

pub(crate) struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_creature)
            .add_system(creature_movement);
    }
}

#[derive(Component)]
pub(crate) struct Creature;

fn setup_creature(
    mut commands: Commands,

    game_world_render_layer: Res<GameWorldRenderLayer>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: asset_server.load("models/creature.glb#Mesh0/Primitive0"),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.3, 1.0),
                unlit: true,
                ..default()
            }),
            ..default()
        })
        .insert(PlanetoidTransform::default())
        .insert(Creature)
        .insert(game_world_render_layer.0);
    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: asset_server.load("models/creature.glb#Mesh0/Primitive0"),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.3, 0.3),
                unlit: true,
                ..default()
            }),
            ..default()
        })
        .insert(PlanetoidTransform {
            sphere_coords: Vec2::new(1.0, 3.14),
            rotation: 5.0,
        })
        .insert(Creature)
        .insert(game_world_render_layer.0);
    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: asset_server.load("models/creature.glb#Mesh0/Primitive0"),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 1.0, 0.3),
                unlit: true,
                ..default()
            }),
            ..default()
        })
        .insert(PlanetoidTransform {
            sphere_coords: Vec2::new(5.0, 3.14),
            rotation: 3.0,
        })
        .insert(Creature)
        .insert(game_world_render_layer.0);
    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: asset_server.load("models/creature.glb#Mesh0/Primitive0"),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.3, 1.0),
                unlit: true,
                ..default()
            }),
            ..default()
        })
        .insert(PlanetoidTransform {
            sphere_coords: Vec2::new(0.5, 1.14),
            rotation: 2.0,
        })
        .insert(Creature)
        .insert(game_world_render_layer.0);
}

fn creature_movement(time: Res<Time>, mut query: Query<&mut PlanetoidTransform, With<Creature>>) {
    for mut transform in &mut query {
        transform.rotation += time.delta_seconds();

        transform.sphere_coords.x += transform.rotation.cos() * 0.5 * time.delta_seconds();
        transform.sphere_coords.y += transform.rotation.sin() * 0.5 * time.delta_seconds();
    }
}
