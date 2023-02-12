use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use crate::vehicle::Vehice;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OrbitCameraKeys::default());
        app.add_system(pan_orbit_camera);
        app.add_system(follow_vehicle);
    }
}

#[derive(Resource)]
pub struct OrbitCameraKeys {
    orbit_button: MouseButton,
}

impl Default for OrbitCameraKeys {
    fn default() -> Self {
        Self {
            orbit_button: MouseButton::Right,
        }
    }
}

#[derive(Component)]
pub struct OrbitCamera {
    pub focus_point: Vec3,
    pub radius: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        OrbitCamera {
            focus_point: Vec3::ZERO,
            radius: 5.0,
        }
    }
}

fn pan_orbit_camera(
    windows: Res<Windows>,
    input_mouse: Res<Input<MouseButton>>,
    orbit_camera_keys: Res<OrbitCameraKeys>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut query: Query<(&mut OrbitCamera, &mut Transform)>,
) {
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;

    if input_mouse.pressed(orbit_camera_keys.orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }

    if let (Ok((mut pan_orbit, mut transform)), Some(window)) =
        (query.get_single_mut(), windows.get_primary())
    {
        if rotation_move.length_squared() > 0.0 {
            let delta_x = rotation_move.x / window.width() * std::f32::consts::PI * 2.0;
            let delta_y = rotation_move.y / window.height() * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation * pitch;
        } else if scroll.abs() > 0.0 {
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            pan_orbit.radius = pan_orbit.radius.clamp(2.0, 90.0);
        }

        let rot_matrix = Mat3::from_quat(transform.rotation);
        transform.translation =
            pan_orbit.focus_point + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
    }
}

fn follow_vehicle(mut camera: Query<(&mut OrbitCamera)>, vehicle: Query<&Transform, With<Vehice>>) {
    if let (Ok(mut camera), Ok(vehicle_transform)) = (camera.get_single_mut(), vehicle.get_single()) {
        camera.focus_point = vehicle_transform.translation;
    }
}
