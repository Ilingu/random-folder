use std::{
    collections::HashSet,
    fs::{self, File},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum AppMode {
    #[default]
    SubFolders,
    Images,
    Videos,
}

#[derive(Debug, Default)]
pub struct AppPreferences {
    pub favs_folders: HashSet<String>,
    pub app_mode: AppMode,
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

        config_path.push("favs_folders");
        if !Path::exists(&config_path) {
            File::create(&config_path).map_err(|_| ())?;
        }

        Ok(config_path)
    }

    pub fn load() -> Self {
        let config_file_path = tod!(Self::get_config_file_path());
        let datas = tod!(fs::read_to_string(config_file_path));
        let favs_folders = datas.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        match favs_folders.is_empty() {
            true => Self::default(),
            false => Self {
                favs_folders: HashSet::from_iter(favs_folders),
                ..Default::default()
            },
        }
    }

    pub fn save(&self) -> Result<(), ()> {
        let config_file_path = Self::get_config_file_path()?;
        fs::write(
            config_file_path,
            self.favs_folders
                .clone()
                .into_iter()
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .map_err(|_| ())
    }
}
