use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_rapier2d::prelude::*;

mod damage;
mod enemy;
mod guns;
mod player;
mod ui;
mod utils;

use utils::IntoState;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1000.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(damage::DamagePlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(guns::GunsPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(ui::UiMainMenuPlugin)
        .add_startup_system(setup)
        .add_system(camera_zoom)
        .run();
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}
impl_into_state!(GameState);

#[derive(Debug, Clone, Resource)]
pub struct GameAssets {
    brick: Handle<Image>,
    player: Handle<Image>,
    enemy: Handle<Image>,
    bullet: Handle<Image>,
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut physics: ResMut<RapierConfiguration>,
) {
    // disable gravity because top down 2d
    physics.gravity = Vec2::ZERO;

    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.5;
    commands.spawn(camera_bundle);

    let assets = GameAssets {
        brick: asset_server.load("sprites/brick.png"),
        player: asset_server.load("sprites/player.png"),
        enemy: asset_server.load("sprites/enemy.png"),
        bullet: asset_server.load("sprites/bullet.png"),
    };

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        texture: assets.brick.clone(),
        ..default()
    });

    commands.insert_resource(assets);
}

// TODO make smooth transition
fn camera_zoom(
    mut proj: Query<&mut OrthographicProjection>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    let mut proj = proj.single_mut();
    let scroll = mouse_wheel_events.iter().map(|e| e.y).sum::<f32>();
    proj.scale = (proj.scale - scroll).clamp(0.5, 2.0);
}
