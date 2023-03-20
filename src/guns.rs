use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{enemy::Enemy, player::Player, EntityDespawnEvent};

pub const BULLET_LIFETIME: f32 = 1.0;
pub const BULLET_LIFETIME_AFTER_IMPACT: f32 = 0.1;
pub const BULLET_VELOCITY: f32 = 2000.0;

pub struct GunsPlugin;

impl Plugin for GunsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootEvent>().add_systems((
            player_shoot,
            bullets_get_fired,
            update_bullets,
        ));
    }
}

#[derive(Component)]
pub struct Gun {
    pub damage: u32,
    pub range: f32,
    pub attack: Timer,
}

#[derive(Component)]
pub struct Bullet {
    lifespan: Timer,
    damage: u32,
}

#[derive(Debug)]
pub struct ShootEvent {
    target: Entity,
    damage: u32,
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

fn bullets_get_fired(
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
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(2.5))
        .insert(Velocity {
            linvel: direction * BULLET_VELOCITY,
            ..default()
        })
        .insert(TransformBundle::from(bullet_transform))
        .insert(Bullet {
            lifespan: Timer::from_seconds(BULLET_LIFETIME, TimerMode::Once),
            damage,
        });
}

fn update_bullets(
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut enemies: Query<&mut Enemy>,
    mut bullets: Query<(Entity, &mut Bullet)>,
    mut despawn_event: EventWriter<EntityDespawnEvent>,
) {
    for (entity, mut bullet) in bullets.iter_mut() {
        if bullet.lifespan.tick(time.delta()).finished() {
            despawn_event.send(EntityDespawnEvent { entity });
        } else {
            let mut hit = false;
            for contact_pair in rapier_context.contacts_with(entity) {
                if let Ok(mut enemy) = enemies.get_mut(contact_pair.collider1()) {
                    hit = true;
                    enemy.health -= bullet.damage as i64;
                } else if let Ok(mut enemy) = enemies.get_mut(contact_pair.collider2()) {
                    hit = true;
                    enemy.health -= bullet.damage as i64;
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
