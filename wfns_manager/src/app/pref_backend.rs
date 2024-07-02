use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

#[derive(Debug, Default)]
pub struct AppPreferences {
    pub liked_folders: Vec<String>,
}

/// Simple macro to return default when error (can be seen as an enhance '?')
macro_rules! tod {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(_) => return Self::default(),
        }
    };
}

impl AppPreferences {
    /// return the path to the app's config file (and ensure that all the necessary directories and files exists)
    fn get_config_file_path() -> Result<PathBuf, ()> {
        let mut config_path = dirs::config_dir().ok_or(())?;

        config_path.push("wfns_manager");
        fs::create_dir_all(&config_path).map_err(|_| ())?;

        config_path.push("liked_folders");
        if !Path::exists(&config_path) {
            File::create(&config_path).map_err(|_| ())?;
        }

        Ok(config_path)
    }

    pub fn load() -> Self {
        let config_file_path = tod!(Self::get_config_file_path());
        let bytes = tod!(fs::read_to_string(config_file_path));
        Self {
            liked_folders: bytes.lines().map(|s| s.to_string()).collect::<Vec<_>>(),
        }
    }

    pub fn save(&self) -> Result<(), ()> {
        let config_file_path = Self::get_config_file_path()?;
        fs::write(config_file_path, self.liked_folders.join("\n")).map_err(|_| ())
    }
}
