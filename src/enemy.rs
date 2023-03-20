use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const ENEMY_SPAWN_TIME: f32 = 1.0;
pub const ENEMY_HEALTH: i64 = 100;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(spawn_enemies.in_set(EnemySystemSet::Spawn));
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum EnemySystemSet {
    Spawn,
    Move,
}

#[derive(Component)]
pub struct Enemy {
    pub health: i64,
}

#[derive(Resource)]
struct EnemyTimer(Timer);

fn setup(mut commands: Commands) {
    commands.insert_resource(EnemyTimer(Timer::from_seconds(
        ENEMY_SPAWN_TIME,
        TimerMode::Repeating,
    )));
}

fn spawn_enemies(time: Res<Time>, mut timer: ResMut<EnemyTimer>, mut commands: Commands) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(10.0))
        .insert(Restitution::coefficient(0.9))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)))
        .insert(Enemy {
            health: ENEMY_HEALTH,
        });
}
