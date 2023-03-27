use bevy::{app::AppExit, prelude::*};

use crate::{
    impl_into_state,
    utils::{remove_all_with, set_state, IntoState},
    GameState,
};

pub struct UiMainMenuPlugin;

impl Plugin for UiMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<UiMainMenuState>()
            .add_startup_system(setup_ui_config)
            .add_system(main_menu_setup.in_schedule(OnEnter(UiMainMenuState::MainMenu)))
            .add_system(button_system.in_set(OnUpdate(UiMainMenuState::MainMenu)))
            .add_system(
                remove_all_with::<UiMainMenuMarker>.in_schedule(OnExit(UiMainMenuState::MainMenu)),
            )
            .add_system(
                set_state::<UiMainMenuState, { UiMainMenuState::MainMenu as u8 }>
                    .in_schedule(OnEnter(GameState::MainMenu)),
            );
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, States)]
enum UiMainMenuState {
    #[default]
    MainMenu,
    Settings,
    InGame,
}
impl_into_state!(UiMainMenuState);

#[derive(Debug, Clone, Copy, Component)]
struct UiMainMenuMarker;

#[derive(Debug, Clone, Copy, Component)]
enum UiMainMenuButton {
    Start,
    Settings,
    Exit,
}

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

fn main_menu_setup(mut commands: Commands, config: Res<UiConfig>) {
    commands
        .spawn(NodeBundle {
            style: config.menu_style.clone(),
            background_color: config.menu_color.into(),
            ..default()
        })
        .insert(UiMainMenuMarker)
        .with_children(|builder| {
            spawn_button(builder, &config, UiMainMenuButton::Start, UiMainMenuMarker);
            spawn_button(
                builder,
                &config,
                UiMainMenuButton::Settings,
                UiMainMenuMarker,
            );
            spawn_button(builder, &config, UiMainMenuButton::Exit, UiMainMenuMarker);
        });
}

fn button_system(
    style: Res<UiConfig>,
    mut game_state: ResMut<NextState<GameState>>,
    mut main_menu_state: ResMut<NextState<UiMainMenuState>>,
    mut interaction_query: Query<
        (&UiMainMenuButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.button_color_pressed.into();
                match button {
                    UiMainMenuButton::Start => {
                        game_state.set(GameState::InGame);
                        main_menu_state.set(UiMainMenuState::InGame)
                    }
                    UiMainMenuButton::Settings => main_menu_state.set(UiMainMenuState::Settings),
                    UiMainMenuButton::Exit => exit.send(AppExit),
                }
            }
            Interaction::Hovered => {
                *color = style.button_color_hover.into();
            }
            Interaction::None => {
                *color = style.button_color_normal.into();
            }
        }
    }
}
