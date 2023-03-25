use bevy::prelude::*;

use crate::{enemy::Enemy, player::Player, GameState};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDamageEvent>()
            .add_event::<EnemyDamageEvent>()
            .add_systems((damage_enemy, damage_player).in_set(OnUpdate(GameState::InGame)));
    }
}

#[derive(Debug)]
pub struct PlayerDamageEvent {
    pub damage: i32,
}

#[derive(Debug)]
pub struct EnemyDamageEvent {
    pub target: Entity,
    pub damage: i32,
}

fn damage_enemy(mut events: EventReader<EnemyDamageEvent>, mut enemies: Query<&mut Enemy>) {
    for event in events.iter() {
        if let Ok(mut enemy) = enemies.get_mut(event.target) {
            enemy.health -= event.damage;
        }
    }
}

fn damage_player(mut events: EventReader<PlayerDamageEvent>, mut player: Query<&mut Player>) {
    let mut player = player.single_mut();
    for event in events.iter() {
        println!("{event:?}");
        player.health -= event.damage;
    }
}
