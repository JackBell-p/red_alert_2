use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
};

use image::{Rgba, RgbaImage};

use super::{
    enums::scene_type::SceneType,
    types::{ColorPoint, FrameHeader, ShapeUnitFrame, ShpHeader},
};
use crate::plugins::pal::reader::get_color_array;

pub fn read_shp(
    shp_path: &str,
    pal_prefix: &str,
    half: bool,
) -> std::io::Result<Vec<ShapeUnitFrame>> {
    let color_array = get_color_array(&pal_prefix, None)?;

    //Read the
    let mut reader = BufReader::new(File::open(shp_path)?);

    let shp_header = parse_header(&mut reader)?;

    let mut frame_header = parse_all_frame_header(&mut reader, shp_header.frames)?;

    let frame_data = parse_all_frame_data(
        &mut reader,
        shp_header,
        &mut frame_header,
        false,
        half,
        pal_prefix,
        color_array,
    )?;

    Ok(frame_data.unwrap())
}

fn parse_header(reader: &mut BufReader<File>) -> std::io::Result<ShpHeader> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;

    Ok(ShpHeader {
        width: u16::from_le_bytes([buf[2], buf[3]]),
        height: u16::from_le_bytes([buf[4], buf[5]]),
        frames: u16::from_le_bytes([buf[6], buf[7]]),
    })
}

fn parse_all_frame_header(
    reader: &mut BufReader<File>,
    frame: u16,
) -> std::io::Result<Vec<FrameHeader>> {
    let mut list: Vec<FrameHeader> = Vec::with_capacity(frame as usize);

    for _ in 0..frame {
        let mut buf = [0u8; 24];
        reader.read_exact(&mut buf)?;

        let header = FrameHeader {
            min_x: u16::from_le_bytes([buf[0], buf[1]]),
            min_y: u16::from_le_bytes([buf[2], buf[3]]),
            width: u16::from_le_bytes([buf[4], buf[5]]),
            height: u16::from_le_bytes([buf[6], buf[7]]),
            frame_type: u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]),
            data_offset: u32::from_le_bytes([buf[20], buf[21], buf[22], buf[23]]),
        };

        list.push(header);
    }

    Ok(list)
}

fn parse_all_frame_data(
    reader: &mut BufReader<File>,
    shp_header: ShpHeader,
    frame_header: &mut Vec<FrameHeader>,
    is_use_shadow: bool,
    half: bool,
    pal_prefix: &str,
    color_array: [u32; 256],
) -> std::io::Result<Option<Vec<ShapeUnitFrame>>> {
    let read_size = decide_read_size(frame_header, is_use_shadow, half);
    Ok(Some(decode_frames(
        reader,
        read_size,
        shp_header,
        frame_header,
        pal_prefix,
        color_array,
    )?))
}

fn decide_read_size(frame_header: &mut Vec<FrameHeader>, is_use_shadow: bool, half: bool) -> usize {
    let frame_size = frame_header.len();

    if is_use_shadow {
        frame_size
    } else if frame_size == 1 {
        1
    } else if half {
        frame_size / 2
    } else {
        frame_size
    }
}

fn decode_frames(
    reader: &mut BufReader<File>,
    read_size: usize,
    header: ShpHeader,
    head_list: &mut Vec<FrameHeader>,
    pal_prefix: &str,
    color_array: [u32; 256],
) -> std::io::Result<Vec<ShapeUnitFrame>> {
    let mut shape_unit_list: Vec<ShapeUnitFrame> = Vec::with_capacity(read_size);

    for i in 0..read_size {
        let target_image: RgbaImage = RgbaImage::new(header.width as u32, header.height as u32);
        let head = &mut head_list[i];

        let (mut shape_unit_frame, frame_type) = build_shape_unit_frame(head, target_image);

        let is_use_unit_color = enable_unit_color(pal_prefix, &mut shape_unit_frame);

        if let Some((mut cur_x, mut cur_y)) = init_draw_position(head, &shape_unit_frame) {
            if frame_type == 0 || frame_type == 1 {
                let _ = draw_frame_type01(
                    head,
                    &mut shape_unit_frame,
                    &mut cur_x,
                    &mut cur_y,
                    reader,
                    color_array,
                    is_use_unit_color,
                );
            } else {
                let _ = draw_frame_type03(
                    head,
                    &mut shape_unit_frame,
                    &mut cur_x,
                    &mut cur_y,
                    reader,
                    is_use_unit_color,
                    color_array,
                );
            }
        }

        let pos = reader.stream_position()?;
        let e = 8 - (pos % 8);
        if e != 8 {
            reader.seek(SeekFrom::Current(e as i64))?;
        }

        shape_unit_list.push(shape_unit_frame);
    }

    Ok(shape_unit_list)
}

fn build_shape_unit_frame(head: &FrameHeader, target_image: RgbaImage) -> (ShapeUnitFrame, u32) {
    let min_x = head.min_x as usize;
    let min_y = head.min_y as usize;
    let max_x = min_x + head.width as usize - 1;
    let max_y = min_y + head.height as usize - 1;
    let real_part_width = head.width as usize;
    let real_part_height = head.height as usize;
    let frame_type = head.frame_type;

    (
        ShapeUnitFrame {
            image: target_image,
            min_x,
            min_y,
            max_x,
            max_y,
            real_part_width,
            real_part_height,
            color_points: None,
        },
        frame_type,
    )
}

fn enable_unit_color(pal_prefix: &str, shape_unit_frame: &mut ShapeUnitFrame) -> bool {
    if SceneType::Snow.info().pal_prefix == pal_prefix
        || SceneType::Tem.info().pal_prefix == pal_prefix
        || SceneType::Urban.info().pal_prefix == pal_prefix
    {
        shape_unit_frame.color_points = Some(Vec::new());
        true
    } else {
        false
    }
}

fn init_draw_position(
    frame_header: &FrameHeader,
    shape_unit_frame: &ShapeUnitFrame,
) -> Option<(usize, usize)> {
    let cur_x = 0;
    let cur_y = frame_header.min_y as usize;

    if shape_unit_frame.real_part_width == 0 && shape_unit_frame.real_part_height == 0 {
        None
    } else {
        Some((cur_x, cur_y))
    }
}

fn draw_frame_type01(
    frame_header: &FrameHeader,
    shape_unit_frame: &mut ShapeUnitFrame,
    cur_x: &mut usize,
    cur_y: &mut usize,
    reader: &mut BufReader<File>,
    color_array: [u32; 256],
    is_use_unit_color: bool,
) -> std::io::Result<()> {
    for _ in 0..shape_unit_frame.real_part_height {
        *cur_x = frame_header.min_x as usize;
        let mut row_bytes = vec![0u8; shape_unit_frame.real_part_width];

        reader.read_exact(&mut row_bytes)?;

        for i in 0..row_bytes.len() {
            let color_index = row_bytes[i] as usize;

            shape_unit_frame.image.put_pixel(
                *cur_x as u32,
                *cur_y as u32,
                argb_to_rgba(color_array[color_index]),
            );

            if is_use_unit_color && (16..32).contains(&color_index) {
                if let Some(color_points) = &mut shape_unit_frame.color_points {
                    color_points.push(ColorPoint {
                        x: *cur_x,
                        y: *cur_y,
                    });
                }
            }

            *cur_x += 1;
        }
        *cur_y += 1;
    }
    Ok(())
}

fn draw_frame_type03(
    frame_header: &FrameHeader,
    shape_unit_frame: &mut ShapeUnitFrame,
    cur_x: &mut usize,
    cur_y: &mut usize,
    reader: &mut BufReader<File>,
    is_use_unit_color: bool,
    color_array: [u32; 256],
) -> std::io::Result<()> {
    for _ in 0..shape_unit_frame.real_part_height {
        let mut effect_buf = [0u8; 2];
        reader.read_exact(&mut effect_buf)?;
        let effect_len = u16::from_le_bytes(effect_buf);

        if effect_len == 4 {
            let mut skip_buf = [0u8; 2];
            reader.read_exact(&mut skip_buf)?;
            *cur_y += 1;
            continue;
        }

        let mut rest_bytes = vec![0u8; (effect_len - 2) as usize];
        reader.read_exact(&mut rest_bytes)?;

        let mut pixel_bytes: Vec<u8> = Vec::new();
        //2+2+N+2.
        if rest_bytes[0] == 0 && rest_bytes[rest_bytes.len() - 2] == 0 {
            let null_bytes_left = rest_bytes[1] as usize;
            *cur_x = frame_header.min_x as usize + null_bytes_left;

            pixel_bytes = rest_bytes[2..rest_bytes.len() - 2].to_vec();
        }
        //2+N+2.
        if rest_bytes[0] != 0 && rest_bytes[rest_bytes.len() - 2] == 0 {
            *cur_x = frame_header.min_x as usize;
            pixel_bytes = rest_bytes[..rest_bytes.len() - 2].to_vec();
        }
        //2+2+N.
        if rest_bytes[0] == 0 && rest_bytes[rest_bytes.len() - 2] != 0 {
            let null_bytes_left = rest_bytes[1] as usize;
            *cur_x = frame_header.min_x as usize + null_bytes_left;

            pixel_bytes = rest_bytes[2..].to_vec();
        }
        //2+N.
        if rest_bytes[0] != 0 && rest_bytes[rest_bytes.len() - 2] != 0 {
            *cur_x = frame_header.min_x as usize;

            pixel_bytes = rest_bytes.to_vec();
        }

        draw_pixel_row(
            &pixel_bytes,
            cur_x,
            cur_y,
            shape_unit_frame,
            color_array,
            is_use_unit_color,
        );
    }

    Ok(())
}

fn draw_pixel_row(
    pixel_bytes: &[u8],
    cur_x: &mut usize,
    cur_y: &mut usize,
    shape_unit_frame: &mut ShapeUnitFrame,
    color_array: [u32; 256],
    is_use_unit_color: bool,
) {
    let mut i = 0;
    while i < pixel_bytes.len() {
        if pixel_bytes[i] == 0 && i + 1 < pixel_bytes.len() && pixel_bytes[i + 1] > 0 {
            let ntime0 = pixel_bytes[i + 1] as usize;
            for _ in 0..ntime0 {
                shape_unit_frame.image.put_pixel(
                    *cur_x as u32,
                    *cur_y as u32,
                    argb_to_rgba(color_array[0]),
                );
                *cur_x += 1;
            }
            i += 2;
        } else {
            let color_index = pixel_bytes[i] as usize;

            shape_unit_frame.image.put_pixel(
                *cur_x as u32,
                *cur_y as u32,
                argb_to_rgba(color_array[color_index]),
            );

            if is_use_unit_color && (16..32).contains(&color_index) {
                if let Some(list) = &mut shape_unit_frame.color_points {
                    list.push(ColorPoint {
                        x: *cur_x,
                        y: *cur_y,
                    });
                }
            }

            *cur_x += 1;
            i += 1;
        }
    }
    *cur_y += 1;
}

fn argb_to_rgba(argb: u32) -> Rgba<u8> {
    let a = ((argb >> 24) & 0xFF) as u8;
    let r = ((argb >> 16) & 0xFF) as u8;
    let g = ((argb >> 8) & 0xFF) as u8;
    let b = (argb & 0xFF) as u8;
    Rgba([r, g, b, a])
}
