use bevy::prelude::*;

mod enums;
mod image_utils;
mod loader;
mod reader;
mod types;

pub struct Shp;

impl Plugin for Shp {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shp_system);
    }
}

fn shp_system() {
    let _ = shp_to_image("numislmk", "f:\\shp", "uniturb");
}

fn shp_to_image(shp_prefix: &str, save_path: &str, pal_prefix: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(save_path)?;

    let mut loader = loader::Loader::new("assets\\shp").unwrap();
    let frames = loader.load_shp(shp_prefix, pal_prefix, false).unwrap();

    println!("Loaded {} frames", frames.len());

    for (i, frame) in frames.iter().enumerate() {
        let file_path = format!("{}\\{}{}.png", save_path, shp_prefix, i);

        frame
            .image
            .save_with_format(&file_path, image::ImageFormat::Png)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        println!("Saved: {}", file_path);

        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    Ok(())
}
