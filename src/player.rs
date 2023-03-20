use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{enemy::EnemyWave, guns::Gun};

pub const PLAYER_SPEED: f32 = 696.0;

pub const PLAYER_GUN_DAMAGE: u32 = 10;
pub const PLAYER_GUN_RANGE: f32 = 300.0;
pub const PLAYER_ATTACKSPEED: f32 = 1.0;

pub const ENEMY_WAVE_NUMBER: u32 = 3;
pub const ENEMY_WAVE_RADIUS: f32 = 300.0;
pub const ENEMY_WAVE_SPAWN_TIME: f32 = 3.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(update_player);
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(20.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(20.0))
        .insert(Velocity::default())
        .insert(ExternalForce::default())
        .insert(Damping {
            linear_damping: 10.0,
            angular_damping: 1.0,
        })
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(Player {
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
        });
}

fn update_player(
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
    // TODO deal with constant
    velocity.linvel = movement * player.speed * 500.0;
}
