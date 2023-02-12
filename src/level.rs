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
    commands
        .spawn(PbrBundle {
            mesh,
            material,
            ..default()
        })
        .insert(Collider::cuboid(50.0, 0.5, 50.0));
}
