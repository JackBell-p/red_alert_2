use std::{collections::HashMap, fs, path::PathBuf};

use super::{reader, types::ShapeUnitFrame};

pub struct Loader {
    pub paths: HashMap<String, PathBuf>,
    cache: HashMap<String, Vec<ShapeUnitFrame>>,
}

impl Loader {
    //Load shp path.
    pub fn new(shp_dir: &str) -> std::io::Result<Self> {
        let mut paths = HashMap::new();

        for entry in fs::read_dir(shp_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("shp") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    paths.insert(stem.to_string(), path);
                }
            }
        }

        Ok(Self {
            paths,
            cache: HashMap::new(),
        })
    }

    pub fn load_shp(
        &mut self,
        shp_prefix: &str,
        pal_prefix: &str,
        half: bool,
    ) -> std::io::Result<&Vec<ShapeUnitFrame>> {
        let key = format!("{shp_prefix}-{pal_prefix}");
        let shp_path = match self.get_shp_by_prefix(shp_prefix) {
            Some(shp_path) => shp_path,
            None => {
                println!("SHP file not found for prefix: {}", shp_prefix);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "SHP file not found",
                ));
            }
        };

        if self.cache.contains_key(&key) {
            return Ok(self.cache.get(&key).unwrap());
        }

        let frames = reader::read_shp(shp_path.to_str().unwrap(), pal_prefix, half).unwrap();

        self.cache.insert(key.clone(), frames);

        Ok(self.cache.get(&key).unwrap())
    }

    fn get_shp_by_prefix(&self, shp_prefix: &str) -> Option<&PathBuf> {
        self.paths.get(shp_prefix)
    }
}
