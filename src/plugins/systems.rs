use bevy::prelude::*;

mod spawn_sprite;

pub struct Systems;

impl Plugin for Systems {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sprite::setup)
            .add_systems(Update, spawn_sprite::animate_sprite);
    }
}
