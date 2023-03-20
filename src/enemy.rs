use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::Player;

pub const ENEMY_HEALTH: i64 = 100;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemies);
    }
}

#[derive(Component)]
pub struct Enemy {
    pub health: i64,
}

#[derive(Component)]
pub struct EnemyWave {
    pub number: u32,
    pub radius: f32,
    pub timer: Timer,
}

fn spawn_enemies(
    time: Res<Time>,
    mut commands: Commands,
    mut wave: Query<(&Transform, &mut EnemyWave), With<Player>>,
) {
    let (player_transform, mut wave) = wave.single_mut();

    if !wave.timer.tick(time.delta()).finished() {
        return;
    }

    for n in 0..wave.number {
        let position = player_transform.translation
            + Quat::from_rotation_z((2.0 * std::f32::consts::PI / wave.number as f32) * n as f32)
                .mul_vec3(Vec3::Y * wave.radius);

        commands
            .spawn(RigidBody::Dynamic)
            .insert(Collider::ball(10.0))
            .insert(Damping {
                linear_damping: 10.0,
                angular_damping: 1.0,
            })
            .insert(TransformBundle::from(Transform::from_translation(position)))
            .insert(Enemy {
                health: ENEMY_HEALTH,
            });
    }
}
