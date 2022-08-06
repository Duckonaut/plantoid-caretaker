use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

#[derive(Component)]
pub(crate) struct MainCamera;

pub(crate) struct MainCameraTransform {
    pub(crate) value: Mat4,
}

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MainCameraTransform {
            value: Mat4::IDENTITY,
        })
        .add_system(scroll_events)
        .add_system(camera_pan)
        .add_system(update_cam_transform);
    }
}

fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    for mut transform in query.iter_mut() {
        if let Some(mouse_wheel) = scroll_evr.iter().next() {
            let zoom = -mouse_wheel.y;
            let zoom_factor = 1.0 + zoom * 0.05;

            transform.translation *= zoom_factor;
        }
    }
}

fn camera_pan(
    buttons: Res<Input<MouseButton>>,
    mut mouse_motion_evr: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    if !buttons.pressed(MouseButton::Right) {
        return;
    }

    for mut transform in query.iter_mut() {
        for mouse_motion in mouse_motion_evr.iter() {
            let delta = mouse_motion.delta;

            let rot_y = Quat::from_rotation_y(delta.x * 0.01);
            let rot_x = Quat::from_axis_angle(transform.local_x(), delta.y * -0.01);

            let delta = Mat4::from_quat(rot_y * rot_x);

            *transform = Transform::from_matrix(delta).mul_transform(*transform);
        }
    }
}

fn update_cam_transform(
    mut cam_transform: ResMut<MainCameraTransform>,
    query: Query<&mut Transform, With<MainCamera>>,
) {
    if let Some(transform) = query.iter().next() {
        cam_transform.value = transform.compute_matrix();
    }
}
