use bevy::prelude::*;

mod errors;
mod pal;
mod shp;
mod systems;
mod window_setup;

pub struct Plugins;

impl Plugin for Plugins {
    fn build(&self, app: &mut App) {
        //app.add_plugins(shp::Shp);
        app.add_plugins(window_setup::WindowSetup)
            .add_plugins(systems::Systems)
            .add_plugins(shp::Shp);
    }
}
