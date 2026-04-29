use bevy::prelude::*;


pub mod loader;
mod reader;
mod types;
mod enums;

pub struct Shp;

impl Plugin for Shp {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shp_system);
    }
}

fn shp_system() {
    
}
