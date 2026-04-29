use image::RgbaImage;

pub struct ShpHeader {
    pub width: u16,
    pub height: u16,
    pub frames: u16,
}

pub struct FrameHeader {
    pub min_x: u16,
    pub min_y: u16,
    pub width: u16,
    pub height: u16,
    pub frame_type: u32,
    pub data_offset: u32, 
}

pub struct ShapeUnitFrame {
    pub image: RgbaImage,
    pub min_y: usize,
    pub min_x: usize,
    pub max_y: usize,
    pub max_x: usize,
    pub real_part_width: usize,
    pub real_part_height: usize,
    pub color_points: Option<Vec<ColorPoint>>
}

pub struct ColorPoint {
    pub x: usize,
    pub y: usize,
}
