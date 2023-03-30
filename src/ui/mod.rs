use bevy::prelude::*;

use crate::{
    impl_into_state,
    utils::{set_state, IntoState},
    GameState,
};

mod level_up;
mod main_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<UiState>()
            .add_startup_system(setup_ui_config)
            .add_system(
                set_state::<UiState, { UiState::MainMenu as u8 }>
                    .in_schedule(OnEnter(GameState::MainMenu)),
            )
            .add_system(
                set_state::<UiState, { UiState::InGame as u8 }>
                    .in_schedule(OnEnter(GameState::InGame)),
            )
            .add_system(
                set_state::<UiState, { UiState::LevelUp as u8 }>
                    .in_schedule(OnEnter(GameState::LevelUp)),
            )
            .add_plugin(level_up::UiLevelUpPlugin)
            .add_plugin(main_menu::UiMainMenuPlugin);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, States)]
enum UiState {
    #[default]
    MainMenu,
    Settings,
    InGame,
    LevelUp,
}
impl_into_state!(UiState);

#[derive(Debug, Clone, Resource)]
pub struct UiConfig {
    pub button_style: Style,
    pub button_color_normal: Color,
    pub button_color_hover: Color,
    pub button_color_pressed: Color,
    pub menu_style: Style,
    pub menu_color: Color,
    pub text_style: TextStyle,
}

fn setup_ui_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiConfig {
        button_style: Style {
            size: Size::new(Val::Px(200.0), Val::Px(100.0)),
            margin: UiRect::all(Val::Percent(1.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        button_color_normal: Color::rgb(0.15, 0.15, 0.15),
        button_color_hover: Color::rgb(0.25, 0.25, 0.25),
        button_color_pressed: Color::rgb(0.35, 0.75, 0.35),
        menu_style: Style {
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        menu_color: Color::NONE,
        text_style: TextStyle {
            font: asset_server.load("fonts/monaco.ttf"),
            font_size: 20.0,
            color: Color::hex("faa307").unwrap(),
        },
    });
}

fn spawn_button<B, M>(child_builder: &mut ChildBuilder, style: &UiConfig, button: B, marker: M)
where
    B: Component + std::fmt::Debug,
    M: Component + Copy,
{
    child_builder
        .spawn(ButtonBundle {
            style: style.button_style.clone(),
            background_color: style.button_color_normal.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text::from_section(format!("{button:?}"), style.text_style.clone()),
                    ..default()
                })
                .insert(marker);
        })
        .insert(button)
        .insert(marker);
}
