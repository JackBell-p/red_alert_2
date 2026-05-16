use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::plugins::{errors::Ra2Error, pal::palette::Palette};

use super::shp_frame::ShpFrame;
use super::shp_header::ShpHeader;

pub struct ShpReader {
    header: ShpHeader,
    reader: BufReader<File>,
}

impl ShpReader {
    //Create a new ShpReader from a file.
    fn new(file_path: &Path) -> Result<Self, Ra2Error> {
        let file = File::open(file_path)?;

        let mut reader = BufReader::new(file);
        let shp_header = read_shp_header(&mut reader)?;

        Ok(Self {
            header: shp_header,
            reader,
        })
    }

    fn get_frame(&mut self, index: u64) -> Result<ShpFrame, Ra2Error> {
        self.reader.seek(SeekFrom::Start(8 + index * 24))?;
        let mut buffer = ShpFrame::default();
        buffer.read_shp_frame_header(&mut self.reader)?;
        buffer.read_shp_frame_data(&mut self.reader)?;
        Ok(buffer)
    }
}

fn read_shp_header<R: Read>(reader: &mut R) -> Result<ShpHeader, Ra2Error> {
    let reserved = reader.read_u16::<LittleEndian>()?;
    let width = reader.read_u16::<LittleEndian>()?;
    let height = reader.read_u16::<LittleEndian>()?;
    let number_of_frames = reader.read_u16::<LittleEndian>()?;
    Ok(ShpHeader {
        reserved,
        width,
        height,
        number_of_frames,
    })
}

//Convert shp file to png format
pub fn shp_to_png(shp_path: &Path, palette: &Palette) -> Result<(), Ra2Error> {
    match shp_path.extension() {
        Some(s) if s.eq("shp") => {
            let mut shp = ShpReader::new(shp_path)?;
            for i in 0..shp.header.number_of_frames {
                let frame = shp.get_frame(i as u64)?;
                let image =
                    frame.render(palette, shp.header.width as u32, shp.header.height as u32)?;
                image.save(Path::new(&format!("F:/shp/{:03}.png", i)))?;
            }
        }
        _ => {}
    }
    Ok(())
}
