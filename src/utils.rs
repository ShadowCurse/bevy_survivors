use bevy::prelude::*;

pub fn remove_all_with<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for e in entities.iter() {
        commands.entity(e).despawn();
    }
}
