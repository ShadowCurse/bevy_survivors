use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_level);
    }
}

fn spawn_level(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mesh = meshes.add(shape::Box::new(100.0, 1.0, 100.0).into());
    commands
        .spawn(PbrBundle { mesh, ..default() })
        .insert(Collider::cuboid(50.0, 0.5, 50.0));
}
