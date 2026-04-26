use std::{fs::File, io::BufReader};

fn read_shp(path: &str) -> std::io::Result<()> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut buf = [0u8, 8];

    Ok(())
}

