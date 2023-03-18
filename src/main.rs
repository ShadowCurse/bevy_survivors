use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_event::<ShootEvent>()
        .add_startup_system(setup)
        .add_system(update_player)
        .add_system(spawn_enemies)
        .add_system(player_shoot)
        .add_system(bullets_get_fired)
        .add_system(update_bullets)
        .add_system(despawn_enemies)
        .run();
}

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct Gun {
    damage: u32,
    range: f32,
    attack: Timer,
}

#[derive(Debug)]
struct ShootEvent {
    target: Entity,
    damage: u32,
}

#[derive(Component)]
struct Bullet {
    lifespan: Timer,
    damage: u32,
}

#[derive(Component)]
struct Enemy {
    health: i64,
}

#[derive(Resource)]
struct EnemyTimer(Timer);

fn setup(
    mut commands: Commands,
    mut physics: ResMut<RapierConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(EnemyTimer(Timer::from_seconds(5.0, TimerMode::Repeating)));

    // disable gravity because top down 2d
    physics.gravity = Vec2::ZERO;

    commands.spawn(Camera2dBundle::default());

    let horizontal_wall_mesh = meshes.add(shape::Box::new(1000.0, 100.0, 0.0).into());
    let vertical_wall_mesh = meshes.add(shape::Box::new(100.0, 1000.0, 0.0).into());
    let wall_material = materials.add(ColorMaterial::from(Color::GRAY));

    // Create the walls
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: horizontal_wall_mesh.clone().into(),
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -500.0, 0.0)),
            ..default()
        })
        .insert(Collider::cuboid(500.0, 50.0));
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: horizontal_wall_mesh.into(),
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 500.0, 0.0)),
            ..default()
        })
        .insert(Collider::cuboid(500.0, 50.0));
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: vertical_wall_mesh.clone().into(),
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(500.0, 0.0, 0.0)),
            ..default()
        })
        .insert(Collider::cuboid(50.0, 500.0));
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: vertical_wall_mesh.into(),
            material: wall_material,
            transform: Transform::from_translation(Vec3::new(-500.0, 0.0, 0.0)),
            ..default()
        })
        .insert(Collider::cuboid(50.0, 500.0));

    // Create the bouncing ball
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));

    // Create player
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
        .insert(Player { speed: 696.0 })
        .insert(Gun {
            damage: 10,
            range: 300.0,
            attack: Timer::from_seconds(1.0, TimerMode::Repeating),
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
    velocity.linvel = movement * player.speed * 500.0;
}

fn spawn_enemies(
    time: Res<Time>,
    mut timer: ResMut<EnemyTimer>,
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    // Create the bouncing ball
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(10.0))
        .insert(Restitution::coefficient(0.9))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)))
        .insert(Enemy { health: 20 });
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
                    lifespan: Timer::from_seconds(1.0, TimerMode::Once),
                    damage: e.damage,
                });
        }
    }
}

fn update_bullets(
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    mut enemies: Query<&mut Enemy>,
    mut bullets: Query<(Entity, &mut Bullet)>,
) {
    for (entity, mut bullet) in bullets.iter_mut() {
        if bullet.lifespan.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
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
                commands.entity(entity).despawn();
            }
        }
    }
}

fn despawn_enemies(mut commands: Commands, enemies: Query<(Entity, &Enemy)>) {
    for (entity, enemy) in enemies.iter() {
        if enemy.health <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
