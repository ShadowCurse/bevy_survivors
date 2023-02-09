use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct VehicePlugin;

impl Plugin for VehicePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player_vehicle);
    }
}

struct Vehice {}

fn spawn_player_vehicle(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mesh = meshes.add(shape::Box::new(10.0, 1.0, 5.0).into());
    commands
        .spawn(PbrBundle { mesh, ..default() })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 2.0, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(5.0, 0.5, 2.5))
        .insert(Restitution::coefficient(0.7));
}
