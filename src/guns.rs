use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    damage::EnemyDamageEvent, enemy::Enemy, player::Player, utils::remove_all_with, GameState,
};

pub const BULLET_LIFETIME: f32 = 1.0;
pub const BULLET_LIFETIME_AFTER_IMPACT: f32 = 0.1;
pub const BULLET_VELOCITY: f32 = 2000.0;

pub struct GunsPlugin;

impl Plugin for GunsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootEvent>()
            .add_systems(
                (player_shoot, fire_bullets, update_bullets).in_set(OnUpdate(GameState::InGame)),
            )
            .add_system(remove_all_with::<BulletMarker>.in_schedule(OnExit(GameState::InGame)));
    }
}

#[derive(Component)]
pub struct Gun {
    pub damage: i32,
    pub range: f32,
    pub attack: Timer,
}

#[derive(Component)]
pub struct Bullet {
    lifespan: Timer,
    damage: i32,
}

#[derive(Component)]
pub struct BulletMarker;

#[derive(Bundle)]
pub struct BulletBundle {
    rigit_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    mass: ColliderMassProperties,
    bullet: Bullet,
    marker: BulletMarker,
}

impl BulletBundle {
    fn new(direction: Vec2, damage: i32) -> Self {
        Self {
            rigit_body: RigidBody::Dynamic,
            collider: Collider::ball(2.5),
            velocity: Velocity {
                linvel: direction * BULLET_VELOCITY,
                ..default()
            },
            mass: ColliderMassProperties::Mass(0.01),
            bullet: Bullet {
                lifespan: Timer::from_seconds(BULLET_LIFETIME, TimerMode::Once),
                damage,
            },
            marker: BulletMarker,
        }
    }
}

#[derive(Debug)]
pub struct ShootEvent {
    target: Entity,
    damage: i32,
}

fn player_shoot(
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut shoot_event: EventWriter<ShootEvent>,
    mut player_with_gun: Query<(&Transform, &mut Gun), With<Player>>,
) {
    let (pt, mut pg) = player_with_gun.single_mut();

    if !pg.attack.tick(time.delta()).finished() {
        return;
    }

    let callback = |e| {
        shoot_event.send(ShootEvent {
            target: e,
            damage: pg.damage,
        });
        true
    };

    rapier_context.intersections_with_shape(
        pt.translation.truncate(),
        0.0,
        &Collider::ball(pg.range),
        QueryFilter::only_dynamic(),
        callback,
    );
}

fn fire_bullets(
    player: Query<&Transform, With<Player>>,
    enemies: Query<&Transform, With<Enemy>>,
    mut commands: Commands,
    mut events: EventReader<ShootEvent>,
) {
    if events.is_empty() {
        return;
    }

    let player_transform = player.single();

    let (mut direction, mut damage, mut length) = (Vec3::ZERO, 0, f32::MAX);
    for e in events.iter() {
        if let Ok(enemy_transform) = enemies.get(e.target) {
            let dir = enemy_transform.translation - player_transform.translation;
            if dir.length_squared() < length {
                direction = dir;
                length = dir.length_squared();
                damage = e.damage;
            }
        }
    }
    let direction = direction.truncate().normalize();

    let mut bullet_transform = *player_transform;
    bullet_transform.translation += (direction * 25.0).extend(0.0);

    commands
        .spawn(BulletBundle::new(direction, damage))
        .insert(TransformBundle::from(bullet_transform));
}

fn update_bullets(
    time: Res<Time>,
    enemies: Query<Entity, With<Enemy>>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Bullet)>,
    mut damage_event: EventWriter<EnemyDamageEvent>,
) {
    for (entity, mut bullet) in bullets.iter_mut() {
        if bullet.lifespan.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        } else {
            let mut hit = false;
            for contact_pair in rapier_context.contacts_with(entity) {
                if let Ok(enemy) = enemies.get(contact_pair.collider1()) {
                    hit = true;
                    damage_event.send(EnemyDamageEvent {
                        target: enemy,
                        damage: bullet.damage,
                    });
                } else if let Ok(enemy) = enemies.get(contact_pair.collider2()) {
                    hit = true;
                    damage_event.send(EnemyDamageEvent {
                        target: enemy,
                        damage: bullet.damage,
                    });
                }
            }
            if hit {
                bullet
                    .lifespan
                    .set_duration(std::time::Duration::from_secs_f32(
                        BULLET_LIFETIME_AFTER_IMPACT,
                    ));
            }
        }
    }
}
