use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{enemy::EnemyWave, guns::Gun, utils::remove_all_with, GameState};

pub const PLAYER_SPEED: f32 = 120.0;
pub const PLAYER_HEALTH: i32 = 100;
pub const PLAYER_MOVEMENT_FORCE: f32 = 1000.0;

pub const PLAYER_GUN_DAMAGE: i32 = 10;
pub const PLAYER_GUN_RANGE: f32 = 100.0;
pub const PLAYER_ATTACKSPEED: f32 = 1.0;

pub const ENEMY_WAVE_NUMBER: u32 = 3;
pub const ENEMY_WAVE_RADIUS: f32 = 150.0;
pub const ENEMY_WAVE_SPAWN_TIME: f32 = 3.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::InGame)))
            .add_systems((player_movement, player_death).in_set(OnUpdate(GameState::InGame)))
            .add_system(remove_all_with::<PlayerMarker>.in_schedule(OnExit(GameState::InGame)));
    }
}

#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub speed: f32,
}

#[derive(Component)]
pub struct PlayerMarker;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::ball(10.0))
        .insert(Velocity::default())
        .insert(Damping {
            linear_damping: 10.0,
            angular_damping: 1.0,
        })
        .insert(Player {
            health: PLAYER_HEALTH,
            speed: PLAYER_SPEED,
        })
        .insert(Gun {
            damage: PLAYER_GUN_DAMAGE,
            range: PLAYER_GUN_RANGE,
            attack: Timer::from_seconds(PLAYER_ATTACKSPEED, TimerMode::Repeating),
        })
        .insert(EnemyWave {
            number: ENEMY_WAVE_NUMBER,
            radius: ENEMY_WAVE_RADIUS,
            timer: Timer::from_seconds(ENEMY_WAVE_SPAWN_TIME, TimerMode::Repeating),
        })
        .insert(PlayerMarker);
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

fn player_death(player: Query<&Player>, mut state: ResMut<NextState<GameState>>) {
    let player = player.single();

    if player.health <= 0 {
        state.set(GameState::MainMenu);
    }
}
