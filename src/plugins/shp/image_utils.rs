use bevy::{
    asset::{Assets, Handle, RenderAssetUsages},
    image::Image,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use image::RgbaImage;

use super::loader::Loader;

fn rgba_to_bevy_image(rgba: &RgbaImage) -> Image {
    let (width, height) = rgba.dimensions();
    // Vec<u8> RGBA.
    let raw = rgba.clone().into_raw();

    Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        raw,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}

pub fn load_shp_images(
    images: &mut Assets<Image>,
    shp_prefix: &str,
    pal_prefix: &str,
) -> std::io::Result<Vec<Handle<Image>>> {
    let mut loader = Loader::new("assets\\shp")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let frames = loader
        .load_shp(shp_prefix, pal_prefix, false)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut handles = Vec::new();

    for frame in frames {
        let bevy_image = rgba_to_bevy_image(&frame.image);

        handles.push(images.add(bevy_image));
    }

    Ok(handles)
}
