use std::{fs::File, io::{BufReader, Read}};

pub fn read_shp(path: &str) -> std::io::Result<()> {
    let mut reader = BufReader::new(File::open(path)?);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    for byte in buffer.iter().take(11) {
        println!("{:02X}", byte);
    }

    Ok(())
}

