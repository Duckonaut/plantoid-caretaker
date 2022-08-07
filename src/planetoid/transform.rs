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
            * Mat4::from_axis_angle(
                Quat::from_rotation_y(-(planetoid_transform.sphere_coords.x - 0.5) * PI * 2.0 + PI)
                    * Vec3::new(0.0, 0.0, 1.0),
                planetoid_transform.sphere_coords.y * PI,
            )
            * Mat4::from_translation(Vec3::new(0.0, 1.0, 0.0))
            * Mat4::from_rotation_y(-planetoid_transform.rotation);

        *transform = Transform::from_matrix(matrix);
    }
}

pub(crate) fn cartesian_to_normalized_sphere(pos: Vec3) -> Vec2 {
    Vec2::new(
        0.5 + f32::atan2(pos.z, pos.x) / (PI * 2.0),
        f32::acos(pos.y / pos.length()) / PI,
    )
}

pub(crate) fn normalized_sphere_to_cartesian(sphere_coords: Vec2) -> Vec3 {
    let x = f32::sin(sphere_coords.y * PI) * f32::cos(sphere_coords.x * PI);
    let y = f32::cos(sphere_coords.y * PI);
    let z = f32::sin(sphere_coords.y * PI) * f32::sin(sphere_coords.x * PI);
    Vec3::new(x, y, z)
}
