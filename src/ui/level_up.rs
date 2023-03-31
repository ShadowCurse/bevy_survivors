use bevy::prelude::*;

use crate::{player::PlayerUpgradeEvent, utils::remove_all_with, GameState};

use super::{spawn_button, UiConfig, UiState};

pub struct UiLevelUpPlugin;

impl Plugin for UiLevelUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(UiState::LevelUp)))
            .add_system(button_system.in_set(OnUpdate(UiState::LevelUp)))
            .add_system(remove_all_with::<UiLevelUpMarker>.in_schedule(OnExit(UiState::LevelUp)));
    }
}

#[derive(Debug, Clone, Copy, Component)]
struct UiLevelUpMarker;

#[derive(Debug, Clone, Copy, Component)]
enum UiLevelUpButton {
    AttackSpeed,
    AttackDamage,
}

fn setup(mut commands: Commands, config: Res<UiConfig>) {
    commands
        .spawn(NodeBundle {
            style: config.menu_style.clone(),
            background_color: config.menu_color.into(),
            ..default()
        })
        .insert(UiLevelUpMarker)
        .with_children(|builder| {
            spawn_button(
                builder,
                &config,
                UiLevelUpButton::AttackSpeed,
                UiLevelUpMarker,
            );
            spawn_button(
                builder,
                &config,
                UiLevelUpButton::AttackDamage,
                UiLevelUpMarker,
            );
        });
}

fn button_system(
    style: Res<UiConfig>,
    mut game_state: ResMut<NextState<GameState>>,
    mut player_upgrade_event: EventWriter<PlayerUpgradeEvent>,
    mut interaction_query: Query<
        (&UiLevelUpButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.button_color_pressed.into();
                match button {
                    UiLevelUpButton::AttackSpeed => {
                        player_upgrade_event.send(PlayerUpgradeEvent::AttackSpeed);
                        game_state.set(GameState::InGame);
                    }
                    UiLevelUpButton::AttackDamage => {
                        player_upgrade_event.send(PlayerUpgradeEvent::AttackDamage);
                        game_state.set(GameState::InGame);
                    }
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
