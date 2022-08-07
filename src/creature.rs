use bevy::prelude::*;

use crate::{
    planetoid::transform::{
        cartesian_to_normalized_sphere, normalized_sphere_to_cartesian, PlanetoidTransform,
    },
    GameWorldRenderLayer,
};

pub(crate) struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CreatureTarget { target: None })
            .add_startup_system(setup_creature)
            .add_system(creature_movement);
    }
}

#[derive(Component)]
pub(crate) struct Creature;

pub(crate) struct CreatureTarget {
    pub(crate) target: Option<Vec2>,
}

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
        .insert(PlanetoidTransform {
            sphere_coords: Vec2::new(0.0, 0.0),
            rotation: 0.0,
        })
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
            sphere_coords: Vec2::new(0.5, 0.0),
            rotation: 0.0,
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
            sphere_coords: Vec2::new(0.0, 0.5),
            rotation: 0.0,
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
            sphere_coords: Vec2::new(0.5, 0.5),
            rotation: 0.0,
        })
        .insert(Creature)
        .insert(game_world_render_layer.0);
}

fn creature_movement(
    time: Res<Time>,
    target: Res<CreatureTarget>,
    mut query: Query<&mut PlanetoidTransform, With<Creature>>,
) {
    if let Some(target) = target.target {
        for mut transform in query.iter_mut() {
            transform.sphere_coords = target;
        }
    }
}
