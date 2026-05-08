use bevy::{
    asset::RenderAssetUsages,
    image::Image,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use image::RgbaImage;

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
