use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

mod enemy;
mod guns;
mod player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(guns::GunsPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_event::<EntityDespawnEvent>()
        .add_startup_system(setup)
        .add_system(despawn_entites)
        .run();
}

#[derive(Debug)]
struct EntityDespawnEvent {
    entity: Entity,
}

fn setup(
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
}

fn despawn_entites(mut commands: Commands, mut events: EventReader<EntityDespawnEvent>) {
    for event in events.iter() {
        commands.entity(event.entity).despawn();
    }
}
