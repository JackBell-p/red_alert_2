use bevy::prelude::*;

use super::reader;

pub struct Loader;

impl Plugin for Loader {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shp_system);
    }
}

fn shp_system() {
    reader::read_shp("assets/shp/numislmk.shp");
}
