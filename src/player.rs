use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    enemy::{EnemyWave, Experience},
    guns::Gun,
    utils::remove_all_with,
    GameAssets, GameState,
};

pub const CHARACTER_RADIUS: f32 = 20.0;

pub const PLAYER_SPEED: f32 = 120.0;
pub const PLAYER_HEALTH: i32 = 100;
pub const PLAYER_MOVEMENT_FORCE: f32 = 1000.0;

pub const PLAYER_GUN_DAMAGE: i32 = 10;
pub const PLAYER_GUN_RANGE: f32 = 900.0;
pub const PLAYER_ATTACKSPEED: f32 = 0.5;

pub const PLAYER_PULL_EXP_RANGE: f32 = 600.0;
pub const PLAYER_COLLECT_EXP_RANGE: f32 = 10.0;

pub const EXP_SPEED: f32 = 400.0;

pub const ENEMY_WAVE_NUMBER: u32 = 4;
pub const ENEMY_WAVE_RADIUS: f32 = 800.0;
pub const ENEMY_WAVE_SPAWN_TIME: f32 = 3.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::InGame)))
            .add_systems(
                (player_movement, player_exp, player_death).in_set(OnUpdate(GameState::InGame)),
            )
            .add_system(remove_all_with::<PlayerMarker>.in_schedule(OnExit(GameState::InGame)));
    }
}

#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub speed: f32,
    pub exp: u32,
}

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Bundle)]
pub struct CharacterBundle {
    rigid_body: RigidBody,
    collider: Collider,
    locked_axis: LockedAxes,
    velocity: Velocity,
    damping: Damping,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            locked_axis: LockedAxes::ROTATION_LOCKED,
            collider: Collider::ball(CHARACTER_RADIUS),
            velocity: Velocity::default(),
            damping: Damping {
                linear_damping: 10.0,
                angular_damping: 1.0,
            },
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    character: CharacterBundle,
    player: Player,
    weapon: Gun,
    wave: EnemyWave,
    marker: PlayerMarker,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            character: CharacterBundle::default(),
            player: Player {
                health: PLAYER_HEALTH,
                speed: PLAYER_SPEED,
                exp: 0,
            },
            weapon: Gun {
                damage: PLAYER_GUN_DAMAGE,
                range: PLAYER_GUN_RANGE,
                attack: Timer::from_seconds(PLAYER_ATTACKSPEED, TimerMode::Repeating),
            },
            wave: EnemyWave {
                number: ENEMY_WAVE_NUMBER,
                radius: ENEMY_WAVE_RADIUS,
                timer: Timer::from_seconds(ENEMY_WAVE_SPAWN_TIME, TimerMode::Repeating),
            },
            marker: PlayerMarker,
        }
    }
}

fn setup(game_assets: Res<GameAssets>, mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 1.0)),
            texture: game_assets.player.clone(),
            ..default()
        })
        .insert(PlayerBundle::default());
}

fn player_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&Player, &mut Velocity)>,
) {
    let mut movement = Vec2::ZERO;

    if input.pressed(KeyCode::W) {
        movement.y = 1.0;
    }
    if input.pressed(KeyCode::S) {
        movement.y = -1.0;
    }
    if input.pressed(KeyCode::A) {
        movement.x = -1.0;
    }
    if input.pressed(KeyCode::D) {
        movement.x = 1.0;
    }

    if movement == Vec2::ZERO {
        return;
    }

    let movement = movement.normalize() * time.delta().as_secs_f32();

    let (player, mut velocity) = player.single_mut();
    velocity.linvel = movement * player.speed * PLAYER_MOVEMENT_FORCE;
}

fn player_exp(
    time: Res<Time>,
    mut commands: Commands,
    mut player: Query<(&Transform, &mut Player), Without<Experience>>,
    mut exp: Query<(Entity, &Experience, &mut Transform), Without<Player>>,
) {
    let (player_transform, mut player) = player.single_mut();

    for (entity, exp, mut transform) in exp.iter_mut() {
        let vec = player_transform.translation - transform.translation;
        let len = vec.length();
        let dir = vec.normalize();
        if len < PLAYER_PULL_EXP_RANGE {
            transform.translation += dir * time.delta().as_secs_f32() * EXP_SPEED;
        }
        if len < PLAYER_COLLECT_EXP_RANGE {
            commands.entity(entity).despawn();
            player.exp += exp.exp;
        }
    }
}

fn player_death(player: Query<&Player>, mut state: ResMut<NextState<GameState>>) {
    let player = player.single();

    if player.health <= 0 {
        state.set(GameState::MainMenu);
    }
}
