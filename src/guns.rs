use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{enemy::Enemy, player::Player, EntityDespawnEvent};

pub const BULLET_LIFETIME: f32 = 1.0;

pub struct GunsPlugin;

impl Plugin for GunsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootEvent>()
            .add_system(player_shoot.in_set(GunsSystemSet::Shoot))
            .add_systems(
                (bullets_get_fired, update_bullets)
                    .chain()
                    .in_set(GunsSystemSet::Update),
            );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GunsSystemSet {
    Shoot,
    Update,
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
        QueryFilter::default(),
        callback,
    );
}

fn bullets_get_fired(
    player: Query<&Transform, With<Player>>,
    enemies: Query<&Transform, With<Enemy>>,
    mut commands: Commands,
    mut events: EventReader<ShootEvent>,
) {
    let player_transform = player.single();
    for e in events.iter() {
        if let Ok(enemy_transform) = enemies.get(e.target) {
            let dir = (enemy_transform.translation - player_transform.translation)
                .truncate()
                .normalize();

            let mut bullet_transform = *player_transform;
            bullet_transform.translation += (dir * 25.0).extend(0.0);

            commands
                .spawn(RigidBody::Dynamic)
                .insert(Collider::ball(2.5))
                .insert(Velocity {
                    linvel: dir * 2000.0,
                    ..default()
                })
                .insert(ColliderMassProperties::Mass(1.0))
                .insert(TransformBundle::from(bullet_transform))
                .insert(Bullet {
                    lifespan: Timer::from_seconds(BULLET_LIFETIME, TimerMode::Once),
                    damage: e.damage,
                });
        }
    }
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
                despawn_event.send(EntityDespawnEvent { entity });
            }
        }
    }
}
