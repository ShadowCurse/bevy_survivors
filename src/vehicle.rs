use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct VehicePlugin;

impl Plugin for VehicePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player_vehicle);
        app.add_system(vehicle_controller);
    }
}

#[derive(Debug, Default, Component)]
pub struct Vehice {}

fn spawn_player_vehicle(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mesh = meshes.add(shape::Box::new(10.0, 1.0, 5.0).into());
    commands
        .spawn(PbrBundle { mesh, ..default() })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 2.0, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(5.0, 0.5, 2.5))
        // .insert(Restitution::coefficient(0.7))
        .insert(ExternalForce::default())
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        })
        .insert(KinematicCharacterController::default())
        .insert(Vehice {});
}

pub fn vehicle_controller(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut bodies: Query<(&Transform, &mut KinematicCharacterController)>,
) {
    let mut dir = None;
    if keys.pressed(KeyCode::W) {
        dir = Some(Vec3::X);
    }
    if keys.pressed(KeyCode::S) {
        dir = Some(Vec3::NEG_X);
    }
    if keys.pressed(KeyCode::A) {
        dir = Some(Vec3::NEG_Z);
    }
    if keys.pressed(KeyCode::D) {
        dir = Some(Vec3::Z);
    }
    for (transform, mut controller) in bodies.iter_mut() {
        let dir =
            dir.map(|dir| transform.rotation.mul_vec3(dir) * time.delta().as_secs_f32() * 10.0);
        controller.translation = dir;
    }
}
