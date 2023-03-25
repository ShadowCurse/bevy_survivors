use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{damage::PlayerDamageEvent, player::Player, GameState};

pub const ENEMY_HEALTH: i32 = 100;
pub const ENEMY_SPEED: f32 = 69.0;
pub const ENEMY_MOVEMENT_FORCE: f32 = 1000.0;

pub const ENEMY_ATTACK_DAMAGE: i32 = 5;
pub const ENEMY_ATTACK_RADIUS: f32 = 69.0;
pub const ENEMY_ATTACK_SPEED: f32 = 1.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (enemy_spawn, enemy_movement, enemy_damage).in_set(OnUpdate(GameState::InGame)),
        );
    }
}

#[derive(Component)]
pub struct Enemy {
    pub health: i32,
    pub speed: f32,

    pub distance_to_player: f32,
}

#[derive(Component)]
pub struct EnemyAttack {
    damage: i32,
    range: f32,
    timer: Timer,
}

#[derive(Component)]
pub struct EnemyWave {
    pub number: u32,
    pub radius: f32,
    pub timer: Timer,
}

fn enemy_spawn(
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
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Collider::ball(10.0))
            .insert(Velocity::default())
            .insert(Damping {
                linear_damping: 10.0,
                angular_damping: 1.0,
            })
            .insert(TransformBundle::from(Transform::from_translation(position)))
            .insert(Enemy {
                health: ENEMY_HEALTH,
                speed: ENEMY_SPEED,
                distance_to_player: f32::MAX,
            })
            .insert(EnemyAttack {
                damage: ENEMY_ATTACK_DAMAGE,
                range: ENEMY_ATTACK_RADIUS,
                timer: Timer::from_seconds(ENEMY_ATTACK_SPEED, TimerMode::Repeating),
            });
    }
}

fn enemy_movement(
    time: Res<Time>,
    player: Query<&Transform, With<Player>>,
    mut enemies: Query<(&Transform, &mut Enemy, &mut Velocity)>,
) {
    let player_transform = player.single();

    for (enemy_transform, mut enemy, mut enemy_velocity) in enemies.iter_mut() {
        let vector = player_transform.translation - enemy_transform.translation;
        let distance = vector.length();
        let direction = vector.truncate().normalize();
        let movement = direction * time.delta().as_secs_f32();

        enemy_velocity.linvel = movement * enemy.speed * ENEMY_MOVEMENT_FORCE;
        enemy.distance_to_player = distance;
    }
}

fn enemy_damage(
    time: Res<Time>,
    mut damage_event: EventWriter<PlayerDamageEvent>,
    mut enemies: Query<(&Enemy, &mut EnemyAttack)>,
) {
    for (enemy, mut attack) in enemies.iter_mut() {
        if !attack.timer.tick(time.delta()).finished() {
            continue;
        }
        if enemy.distance_to_player <= attack.range {
            damage_event.send(PlayerDamageEvent {
                damage: attack.damage,
            });
        }
    }
}
