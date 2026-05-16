use std::path::Path;

use bevy::prelude::*;

use crate::plugins::{pal::palette::Palette, shp::shp_reader::shp_to_png};

mod enums;
pub mod image_utils;
mod loader;
mod reader_old;
mod shp_frame;
mod shp_header;
mod shp_reader;
mod types;

pub struct Shp;

impl Plugin for Shp {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, shp_system);
    }
}

fn shp_system() {
    let pal = &Palette::load(Path::new("assets/pal/uniturb.pal")).unwrap();
    let _ = shp_to_png(Path::new("assets/shp/numislmk.shp"), pal);
}
