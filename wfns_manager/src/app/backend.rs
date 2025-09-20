use nanorand::{Rng, WyRand};
use std::{
    fs::{self},
    path::Path,
    process::Command,
};

use crate::app::preferences::AppMode;

macro_rules! tsuts {
    ($e:expr) => {
        $e.to_str().unwrap().to_string()
    };
}

pub trait Openable {
    /// reveal subfolder in default file explorer
    fn open_dir(&self) -> bool;

    /// open first image directly in eog
    fn open_image(&self) -> bool;
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

    fn get_thumbnail(subpath: &str) -> Result<String, ()> {
        let mut entries = fs::read_dir(subpath).map_err(|_| ())?;
        let mut imgs_name = vec![];
        while let Some(Ok(entry)) = entries.next() {
            let name = tsuts!(entry.file_name());
            let without_ext = match name.split('.').next() {
                Some(n) => n,
                None => continue,
            };
            // fastest/cheapest way to find the first image, if not accurate, get all images and sort to find first one...
            if [".jpg", ".png", ".jpeg", ".webp", ".gif"]
                .iter()
                .any(|ext| name.ends_with(ext))
                && ["1", "01", "01_1", "001", "001_1", "0001", "0001_1", "00001"]
                    .iter()
                    .any(|x| &without_ext == x)
            {
                return Ok(name);
            }
            imgs_name.push(name);
        }

        imgs_name.sort();
        match imgs_name.first() {
            Some(n) => Ok(n.to_owned()),
            None => Err(()),
        }
    }

    pub fn get_path(&self) -> String {
        format!("{}/{}", self.root_path, self.name)
    }
}

impl Openable for SubFolder {
    fn open_dir(&self) -> bool {
        if let Some(thumbnail) = &self.thumbnail {
            opener::reveal(format!("{}/{}", self.get_path(), thumbnail)).is_ok()
        } else {
            opener::reveal(self.get_path()).is_ok()
        }
    }

    fn open_image(&self) -> bool {
        if let Some(thumbnail) = &self.thumbnail {
            opener::open(format!("{}/{}", self.get_path(), thumbnail)).is_ok()
        } else {
            false
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum MediaType {
    Image,
    Video,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Media {
    root_path: String,

    pub name: String,
    pub filepath: String,
    pub media_type: MediaType,
}

impl Media {
    pub fn new(name: &str, root_path: &str, media_type: MediaType) -> Self {
        Self {
            root_path: root_path.to_string(),

            name: name
                .split(".")
                .next()
                .map(|n| n.to_string())
                .unwrap_or(name.to_string()),
            filepath: format!("{}/{}", root_path, name),
            media_type,
        }
    }
}

impl Openable for Media {
    fn open_dir(&self) -> bool {
        opener::reveal(&self.filepath).is_ok()
    }

    fn open_image(&self) -> bool {
        opener::open(&self.filepath).is_ok()
    }
}

#[derive(Debug)]
pub struct AppFolderManager {
    pub root_path: String,

    pub subfolders: Vec<SubFolder>,
    pub images: Vec<Media>,
    pub videos: Vec<Media>,

    pub curr: usize,
}

impl AppFolderManager {
    /// scans and construct the folder datas, additionaly it give insight to the app on the right app mode to use depending
    /// on the folder content
    pub fn set_folder(root_path: String) -> Result<(Self, Option<AppMode>), ()> {
        let (sf, img, vid, rec_app_mode) = Self::scan_folder(&root_path).map_err(|_| ())?;

        Ok((
            Self {
                curr: 0,

                subfolders: sf,
                images: img,
                videos: vid,

                root_path,
            },
            rec_app_mode,
        ))
    }

    fn scan_folder(
        root_folder: &str,
    ) -> Result<(Vec<SubFolder>, Vec<Media>, Vec<Media>, Option<AppMode>), ()> {
        let mut sf = Vec::new();
        let mut img = Vec::new();
        let mut vid = Vec::new();

        let mut entries = fs::read_dir(root_folder).map_err(|_| ())?;
        while let Some(Ok(entry)) = entries.next() {
            let filename = tsuts!(entry.file_name());
            match entry.file_type().map(|ft| ft.is_dir()) {
                Ok(true) => sf.push(SubFolder::new(&tsuts!(entry.file_name()), root_folder)),
                Ok(false) => {
                    let extension = tsuts!(Path::new(&filename).extension().ok_or(())?);
                    match extension.as_str() {
                        "jpg" | "jpeg" | "png" | "gif" | "jpe" | "webp" | "tiff" | "ico"
                        | "heif" | "heic" | "tif" | "jif" | "jfif" | "svg" => {
                            img.push(Media::new(&filename, root_folder, MediaType::Image))
                        }
                        "mp4" | "webm" | "avi" | "mov" | "mkv" | "mpeg" | "m4v" | "wmv" | "flv" => {
                            vid.push(Media::new(&filename, root_folder, MediaType::Video))
                        }
                        _ => continue,
                    }
                }
                Err(_) => continue,
            }
        }

        let mut rng = WyRand::new();
        rng.shuffle(&mut sf);
        rng.shuffle(&mut img);
        rng.shuffle(&mut vid);

        let recommanded_app_mode = match (sf.is_empty(), img.is_empty(), vid.is_empty()) {
            (true, false, _) => Some(AppMode::Images),
            (true, true, false) => Some(AppMode::Videos),
            _ => None,
        };

        Ok((sf, img, vid, recommanded_app_mode))
    }

    pub fn reset_curr_index(&mut self) {
        self.curr = 0
    }

    pub fn next(&mut self, app_mode: AppMode) -> Result<(), ()> {
        let next_curr = self.curr.checked_add(1).ok_or(())?;
        match app_mode {
            AppMode::SubFolders => {
                if next_curr >= self.subfolders.len() {
                    return Err(());
                }
            }
            AppMode::Images => {
                if next_curr >= self.images.len() {
                    return Err(());
                }
            }
            AppMode::Videos => {
                if next_curr >= self.videos.len() {
                    return Err(());
                }
            }
        }
        self.curr = next_curr;
        Ok(())
    }

    pub fn prev(&mut self, app_mode: AppMode) -> Result<(), ()> {
        let prev_curr = self.curr.checked_sub(1).ok_or(())?;
        match app_mode {
            AppMode::SubFolders => {
                if prev_curr >= self.subfolders.len() {
                    return Err(());
                }
            }
            AppMode::Images => {
                if prev_curr >= self.images.len() {
                    return Err(());
                }
            }
            AppMode::Videos => {
                if prev_curr >= self.videos.len() {
                    return Err(());
                }
            }
        }
        self.curr = prev_curr;
        Ok(())
    }
}

pub fn get_video_thumbnail(filepath: &str) -> Result<String, ()> {
    let mut cache_path = dirs::cache_dir().ok_or(())?;

    cache_path.push("wfns_manager");
    fs::create_dir_all(&cache_path).map_err(|_| ())?;

    let name = {
        let filename = filepath.split("/").last().expect("Should be a path");
        filename.split(".").next().unwrap_or(filename)
    };

    let outname = format!("{name}_thumb.jpg");
    let out_path = tsuts!(cache_path.join(&outname));
    // check if already cached
    if let Ok(true) = fs::exists(cache_path.join(&outname)) {
        println!("Cached!");
        return Ok(out_path);
    };

    let cmd_status = Command::new("ffmpeg")
        .args([
            "-i",
            &filepath.replace(" ", r"\ "),
            "-vf",
            "scale=600:-1", // thumb of 600px in width, because the gtk::Image is of width 600
            "-frames:v",
            "1",
            "-q:v",
            "2",
            &out_path,
        ])
        .output()
        .map_err(|_| ())?
        .status;

    if !cmd_status.success() {
        return Err(());
    }

    Ok(out_path)
}
