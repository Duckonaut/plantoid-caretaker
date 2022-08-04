use std::f32::consts::PI;

use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::AsBindGroup};

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Component)]
#[uuid = "1e55b055-b1b2-c1c2-d1d2-d3d4d5d6d7d8"]
pub struct PlanetoidMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_ramp: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub heightmap: Handle<Image>,
    #[uniform(4)]
    pub sun_pos: Vec3,
    #[uniform(5)]
    pub sun_intensity: f32,
}

impl Material for PlanetoidMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/planetoid_material.wgsl".into()
    }

    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/planetoid_material.wgsl".into()
    }
}

pub fn update_material_sun_pos(
    time: Res<Time>,
    mut materials: ResMut<Assets<PlanetoidMaterial>>,
    query: Query<&Handle<PlanetoidMaterial>>,
) {
    for handle in query.iter() {
        let mat = &mut materials.get_mut(handle);

        if let Some(mat) = mat {
            let pos = Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0));
            let rot_y = Mat4::from_rotation_y(time.seconds_since_startup() as f32 * 0.25);
            let rot_z = Mat4::from_rotation_z(PI / 4.0);
            let transform = rot_z * rot_y * pos;

            mat.sun_pos = transform.to_scale_rotation_translation().2;
        }
    }
}
