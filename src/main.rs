use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_scene)
        .add_system(update_player)
        .run();
}

#[derive(Component)]
struct Player {
    speed: f32,
}

fn setup_scene(
    mut commands: Commands,
    mut physics: ResMut<RapierConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
        .insert(Damping { linear_damping: 10.0, angular_damping: 1.0 })
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(Player { speed: 696.0 });
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
