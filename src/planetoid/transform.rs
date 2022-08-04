use std::f32::consts::PI;

use bevy::prelude::*;

use super::PlanetoidRotation;

#[derive(Component, Default)]
pub(crate) struct PlanetoidTransform {
    pub(crate) sphere_coords: Vec2,
    pub(crate) rotation: f32,
}

pub(crate) fn match_planetoid_transforms(
    planetoid_rotation: Res<PlanetoidRotation>,
    mut query: Query<(&mut Transform, &PlanetoidTransform)>,
) {
    for (mut transform, planetoid_transform) in query.iter_mut() {
        let matrix = Mat4::from_quat(planetoid_rotation.0)
            * Mat4::from_rotation_z(planetoid_transform.sphere_coords.x)
            * Mat4::from_rotation_x(planetoid_transform.sphere_coords.y)
            * Mat4::from_translation(Vec3::new(0.0, 1.0, 0.0))
            * Mat4::from_rotation_y(planetoid_transform.rotation - PI * 0.5);

        *transform = Transform::from_matrix(matrix);
    }
}
