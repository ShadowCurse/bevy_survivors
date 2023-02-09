use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod level;
mod vehicle;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::BLACK));
    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.4,
    });

    app.add_plugins(DefaultPlugins);
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugin(RapierDebugRenderPlugin::default());

    app.add_plugin(level::LevelPlugin);
    app.add_plugin(vehicle::VehicePlugin);

    app.add_startup_system(setup);

    app.run();
}

fn setup(mut commands: Commands) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 100.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 2.0, 2.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}
