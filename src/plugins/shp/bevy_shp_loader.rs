use bevy::prelude::*;


pub struct BevyShpLoader;

impl Plugin for BevyShpLoader {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shp_system);
    }
}

fn shp_system() {
    println!("SHP system running");
}
