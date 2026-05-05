use std::{
    collections::{HashMap, hash_map::Entry},
    fs,
    path::PathBuf,
};

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
        let shp_path = self.get_shp_by_prefix(shp_prefix).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "SHP file not found.")
        })?;

        match self.cache.entry(key) {
            Entry::Occupied(entry) => {
                //Do have key return it's ref.
                Ok(&*entry.into_mut())
            }
            Entry::Vacant(entry) => {
                //Do not have key read the file.
                let frames = reader::read_shp(&shp_path, pal_prefix, half)?;

                Ok(&*entry.insert(frames))
            }
        }
    }

    fn get_shp_by_prefix(&self, shp_prefix: &str) -> Option<String> {
        self.paths
            .get(shp_prefix)
            .and_then(|s| s.to_str().map(|s| s.to_owned()))
    }
}
