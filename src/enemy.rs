use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    damage::PlayerDamageEvent,
    player::{CharacterBundle, Player},
    utils::remove_all_with,
    GameAssets, GameState,
};

pub const ENEMY_HEALTH: i32 = 20;
pub const ENEMY_SPEED: f32 = 69.0;
pub const ENEMY_MOVEMENT_FORCE: f32 = 1000.0;

pub const ENEMY_ATTACK_DAMAGE: i32 = 10;
pub const ENEMY_ATTACK_RADIUS: f32 = 80.0;
pub const ENEMY_ATTACK_SPEED: f32 = 1.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (enemy_spawn, enemy_movement, enemy_damage, enemy_despawn)
                .in_set(OnUpdate(GameState::InGame)),
        )
        .add_system(remove_all_with::<EnemyMarker>.in_schedule(OnExit(GameState::InGame)));
    }
}

#[derive(Component)]
pub struct Enemy {
    pub health: i32,
    pub speed: f32,

    pub distance_to_player: f32,
}

#[derive(Component)]
pub struct EnemyMarker;

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

#[derive(Bundle)]
pub struct EnemyBundle {
    character: CharacterBundle,
    enemy: Enemy,
    attack: EnemyAttack,
    marker: EnemyMarker,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            character: CharacterBundle::default(),
            enemy: Enemy {
                health: ENEMY_HEALTH,
                speed: ENEMY_SPEED,
                distance_to_player: f32::MAX,
            },
            attack: EnemyAttack {
                damage: ENEMY_ATTACK_DAMAGE,
                range: ENEMY_ATTACK_RADIUS,
                timer: Timer::from_seconds(ENEMY_ATTACK_SPEED, TimerMode::Repeating),
            },
            marker: EnemyMarker,
        }
    }
}

fn enemy_spawn(
    time: Res<Time>,
    game_assets: Res<GameAssets>,
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
            .spawn(SpriteBundle {
                transform: Transform::from_translation(position),
                texture: game_assets.enemy.clone(),
                ..default()
            })
            .insert(EnemyBundle::default());
    }
}

fn enemy_movement(
    time: Res<Time>,
    player: Query<&Transform, With<Player>>,
    mut enemies: Query<(&Transform, &mut Enemy, &mut Velocity)>,
) {
    let player_transform = player.single();

    for (enemy_transform, mut enemy, mut enemy_velocity) in enemies.iter_mut() {
        let vector = (player_transform.translation - enemy_transform.translation).truncate();
        let distance = vector.length();
        let direction = vector.normalize();
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

fn enemy_despawn(mut commands: Commands, enemies: Query<(Entity, &Enemy)>) {
    for (entity, enemy) in enemies.iter() {
        if enemy.health <= 0 {
            commands.entity(entity).despawn()
        }
    }
}
