use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_level);
    }
}

fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(shape::Box::new(100.0, 1.0, 100.0).into());
    let material = materials.add(Color::GRAY.into());
    // flor
    commands
        .spawn(PbrBundle {
            mesh,
            material: material.clone(),
            ..default()
        })
        .insert(Collider::cuboid(50.0, 0.5, 50.0));
    // walls
    let mesh = meshes.add(shape::Box::new(100.0, 20.0, 1.0).into());
    let transform = Transform::from_xyz(0.0, 0.0, 50.0);
    commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform,
            ..default()
        })
        .insert(Collider::cuboid(50.0, 10.0, 0.5));
    let transform = Transform::from_xyz(0.0, 0.0, -50.0);
    commands
        .spawn(PbrBundle {
            mesh,
            material: material.clone(),
            transform,
            ..default()
        })
        .insert(Collider::cuboid(50.0, 10.0, 0.5));
    let mesh = meshes.add(shape::Box::new(1.0, 20.0, 100.0).into());
    let transform = Transform::from_xyz(50.0, 0.0, 0.0);
    commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform,
            ..default()
        })
        .insert(Collider::cuboid(0.5, 10.0, 50.0));
    let transform = Transform::from_xyz(-50.0, 0.0, 0.0);
    commands
        .spawn(PbrBundle {
            mesh,
            material,
            transform,
            ..default()
        })
        .insert(Collider::cuboid(0.5, 10.0, 50.0));
}
