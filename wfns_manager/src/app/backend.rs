use nanorand::{Rng, WyRand};
use std::fs;

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

    /// reveal subfolder in default file explorer
    pub fn open_dir(&self) -> bool {
        if let Some(thumbnail) = &self.thumbnail {
            opener::reveal(format!("{}/{}", self.get_path(), thumbnail)).is_ok()
        } else {
            opener::reveal(self.get_path()).is_ok()
        }
    }

    /// open first image directly in eog
    pub fn open_image(&self) -> bool {
        if let Some(thumbnail) = &self.thumbnail {
            opener::open(format!("{}/{}", self.get_path(), thumbnail)).is_ok()
        } else {
            false
        }
    }

    fn get_thumbnail(subpath: &str) -> Result<String, ()> {
        let mut entries = fs::read_dir(subpath).map_err(|_| ())?;
        while let Some(Ok(entry)) = entries.next() {
            let name = tsuts!(entry.file_name());
            let without_ext = match name.split('.').next() {
                Some(n) => n,
                None => continue,
            };
            // fastest/cheapest way to find the first image, if not accurate, get all images and sort to find first one...
            if [".jpg", ".png", ".gif"]
                .iter()
                .any(|ext| name.ends_with(ext))
                && ["1", "01", "001", "0001"].iter().any(|x| &without_ext == x)
            {
                return Ok(name);
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
    pub curr_sf: SubFolder,
}

impl AppFolderManager {
    pub fn set_folder(root_path: String) -> Result<Self, ()> {
        let res = Self::scan_subfolder(&root_path).map_err(|_| ())?;
        Ok(Self {
            curr_sf: res[0].clone(),
            subfolders: res,
            root_path,
            history: vec![],
        })
    }

    fn scan_subfolder(root_folder: &str) -> Result<Vec<SubFolder>, ()> {
        let mut subfolders = Vec::new();
        let mut entries = fs::read_dir(root_folder).map_err(|_| ())?;
        while let Some(Ok(entry)) = entries.next() {
            if entry.file_type().is_ok() && entry.file_type().unwrap().is_dir() {
                subfolders.push(SubFolder::new(&tsuts!(entry.file_name()), root_folder));
            }
        }
        if subfolders.is_empty() {
            return Err(());
        }
        Ok(subfolders)
    }

    pub fn reset(&mut self) -> bool {
        match Self::set_folder(self.root_path.clone()) {
            Ok(f) => {
                *self = f;
                true
            }
            Err(_) => false,
        }
    }

    pub fn choose(&mut self, write: bool) -> Result<SubFolder, ()> {
        if self.subfolders.is_empty() && !self.reset() {
            return Err(()); // if reset ok continue to choose otherwise error and in the app return to placeholder page
        }
        if write {
            self.history.push(self.curr_sf.clone());
        }

        let mut rng = WyRand::new();
        let index = rng.generate_range(0..self.subfolders.len());
        let picked_sf = self.subfolders[index].clone();

        if write {
            self.subfolders.swap_remove(index); // don't care about ordering
        }

        Ok(picked_sf)
    }

    pub fn rollback(&mut self) -> Option<SubFolder> {
        let lastsf = match self.history.pop() {
            Some(sf) => sf,
            None => return self.choose(false).ok(),
        };
        Some(lastsf)
    }
}
