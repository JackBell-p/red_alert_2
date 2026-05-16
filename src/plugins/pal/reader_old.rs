use std::{
    fs::File,
    io::{BufReader, Read},
};

use super::enums::UnitColor;

fn del_alpha(color: u32) -> u32 {
    color & 0x00FF_FFFF
}

fn add_alpha(color: u32) -> u32 {
    color | 0xFF00_0000
}

fn turn_blue(ori: u32) -> u32 {
    ori >> 16
}

fn turn_green(ori: u32) -> u32 {
    ori >> 8
}

fn turn_yellow(ori: u32) -> u32 {
    ori | turn_green(ori)
}

fn turn_purple(ori: u32) -> u32 {
    ori | turn_blue(ori)
}

fn turn_light_blue(ori: u32) -> u32 {
    turn_green(ori) | turn_blue(ori)
}

fn turn_gray(ori: u32) -> u32 {
    ori | turn_green(ori) | turn_blue(ori)
}

fn turn_orange(ori: u32) -> u32 {
    let a = ori >> 16;
    let bl = a as f64 / 255.0;
    let b = (124.0 * bl) as u32;
    let d = b << 8;
    ori | d
}

pub fn get_color_array(pal_prefix: &str, team: Option<UnitColor>) -> std::io::Result<[u32; 256]> {
    let mut reader = BufReader::new(File::open(format!("assets\\pal\\{}.pal", pal_prefix))?);
    let mut pal = [0u8; 768];
    reader.read_exact(&mut pal)?;

    let mut color_array = [0u32; 256];
    for (i, chunk) in pal.chunks_exact(3).enumerate() {
        let r = (chunk[0] as u32) * 4;
        let g = (chunk[1] as u32) * 4;
        let b = (chunk[2] as u32) * 4;
        color_array[i] = (0xFF << 24) | (r << 16) | (g << 8) | b;
    }

    if let Some(color) = team {
        for i in 17..32 {
            let da = del_alpha(color_array[i]);
            let mc = match color {
                UnitColor::Red => da,
                UnitColor::Blue => turn_blue(da),
                UnitColor::Green => turn_green(da),
                UnitColor::Yellow => turn_yellow(da),
                UnitColor::Purple => turn_purple(da),
                UnitColor::LightBlue => turn_light_blue(da),
                UnitColor::Orange => turn_orange(da),
                UnitColor::Gray => turn_gray(da),
            };
            color_array[i] = add_alpha(mc);
        }
    }

    color_array[0] = 0;

    Ok(color_array)
}
