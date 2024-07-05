use std::{error::Error, fs};

use file_format::{FileFormat, Kind};
use nanorand::{Rng, WyRand};

macro_rules! tsuts {
    ($e:expr) => {
        $e.to_str().unwrap().to_string()
    };
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct SubFolder {
    pub name: String,
    root_path: String,
    pub thumbnail: Option<String>,
}

impl SubFolder {
    pub fn new(name: &str, root_path: &str) -> Self {
        let path = format!("{}/{}", root_path, name);
        Self {
            name: name.to_string(),
            root_path: root_path.to_string(),
            thumbnail: Self::get_thumbnail(&path).ok(),
        }
    }

    pub fn open_dir(&self) -> bool {
        opener::reveal(self.get_path()).is_ok()
    }

    pub fn open_image(&self) -> bool {
        if let Some(thumbnail) = &self.thumbnail {
            opener::open(format!("{}/{}", self.get_path(), thumbnail)).is_ok()
        } else {
            false
        }
    }

    pub fn get_images(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut images = vec![];
        let mut entries = fs::read_dir(self.get_path())?;
        while let Some(Ok(entry)) = entries.next() {
            if entry.file_type().is_ok() && entry.file_type().unwrap().is_file() {
                images.push(tsuts!(entry.file_name()));
            }
        }
        Ok(images)
    }

    fn get_thumbnail(subpath: &str) -> Result<String, ()> {
        let mut entries = fs::read_dir(subpath).map_err(|_| ())?;
        while let Some(Ok(entry)) = entries.next() {
            if entry.file_type().is_ok() && entry.file_type().unwrap().is_file() {
                let name = tsuts!(entry.file_name());
                let path = format!("{}/{name}", subpath);
                if FileFormat::from_file(path).map_err(|_| ())?.kind() == Kind::Image {
                    return Ok(name);
                }
            }
        }
        Err(())
    }
    pub fn get_path(&self) -> String {
        format!("{}/{}", self.root_path, self.name)
    }
}

#[derive(Debug)]
pub struct AppFolderManager {
    pub root_path: String,
    pub subfolders: Vec<SubFolder>,
    pub history: Vec<SubFolder>,
}

impl AppFolderManager {
    pub fn set_folder(root_path: String) -> Result<Self, ()> {
        Ok(Self {
            subfolders: Self::scan_subfolder(&root_path).map_err(|_| ())?,
            root_path,
            history: vec![],
        })
    }

    fn scan_subfolder(root_folder: &str) -> Result<Vec<SubFolder>, Box<dyn Error>> {
        let mut subfolders = Vec::new();
        let mut entries = fs::read_dir(root_folder)?;
        while let Some(Ok(entry)) = entries.next() {
            if entry.file_type().is_ok() && entry.file_type().unwrap().is_dir() {
                subfolders.push(SubFolder::new(&tsuts!(entry.file_name()), root_folder));
            }
        }
        Ok(subfolders)
    }

    pub fn choose(&mut self) -> SubFolder {
        let mut rng = WyRand::new();
        let index = rng.generate_range(0..self.subfolders.len());
        let picked_sf = self.subfolders.swap_remove(index); // don't care about ordering

        self.history.push(picked_sf.clone());
        picked_sf
    }

    pub fn rollback(&mut self) -> Option<SubFolder> {
        let lastsf = match self.history.pop() {
            Some(sf) => sf,
            None => return None,
        };
        self.subfolders.push(lastsf.clone());
        Some(lastsf)
    }
}
