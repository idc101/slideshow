use std::f32::consts::PI;
use std::sync::Mutex;

use exif::Reader as ExifReader;
use exif::Tag;
use rand::seq::IndexedRandom;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

pub struct AppState {
    all_images: Mutex<Vec<PathBuf>>,
    counter: Mutex<i32>,
    pub settings: Mutex<Settings>,
}

// Define the data structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    slideshow: String,
    pub interval: i32,
}

impl AppState {
    pub fn new() -> Self {
        let new = Self {
            all_images: Mutex::new(Vec::new()),
            counter: Mutex::new(0),
            settings: Mutex::new(Settings {
                slideshow: "slideshow".to_string(),
                interval: 300,
            }),
        };
        new
    }

    pub fn set_path(&self, pictures_base: PathBuf) {
        let mut images = self.all_images.lock().unwrap();

        images.clear();
        *images = fs::read_dir(pictures_base)
            .unwrap()
            .filter(|x| {
                x.as_ref()
                    .unwrap()
                    .path()
                    .extension()
                    .unwrap_or_default()
                    .to_ascii_lowercase()
                    == "jpg"
            })
            .map(|x| x.unwrap().path())
            .collect();
    }

    pub fn get_current_image(&self) -> PathBuf {
        let images = self.all_images.lock().unwrap();
        images.choose(&mut rand::rng()).unwrap().clone()
    }

    pub fn get_image(&self, num: i32) -> PathBuf {
        let images = self.all_images.lock().unwrap();
        images.get((num as usize) % images.len()).unwrap().clone()
    }

    pub fn get_image_metadata(&self, num: i32) -> Option<String> {
        let path = self.get_image(num);
        let file = fs::File::open(path.clone()).unwrap();
        let mut bufreader = BufReader::new(file);
        let exifreader = ExifReader::new();
        if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
            if let Some(field) = exif.get_field(Tag::DateTimeOriginal, exif::In::PRIMARY) {
                return Some(field.display_value().to_string());
            }
            if let Some(field) = exif.get_field(Tag::DateTime, exif::In::PRIMARY) {
                return Some(field.display_value().to_string());
            }
        }
        return path
            .file_name()
            .map(|f| f.to_str().map(|s| s.to_string()))
            .unwrap();
    }

    pub fn increment(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
    }

    pub fn get(&self) -> i32 {
        *self.counter.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_counter() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");

        let state = AppState::new();
        state.set_path(d);
        assert_eq!(state.get(), 0);
        state.increment();
        assert_eq!(state.get(), 1);
    }

    #[test]
    fn test_metadata() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");

        let state = AppState::new();
        state.set_path(d);

        let metadata0 = state.get_image_metadata(0).expect("not found 0");
        let metadata1 = state.get_image_metadata(1).expect("not found 1");
        if metadata0 == "2024-10-28 10:08:33" {
            assert_eq!(metadata0, "2024-10-28 10:08:33");
            assert_eq!(metadata1, "2025-02-16 11:14:53");
        } else {
            assert_eq!(metadata0, "2025-02-16 11:14:53");
            assert_eq!(metadata1, "2024-10-28 10:08:33");
        }
    }
}
