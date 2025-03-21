use std::f32::consts::PI;
use std::sync::Mutex;

use rand::seq::IndexedRandom;
use regex::Regex;
use rexiv2::Metadata;
use serde::{Deserialize, Serialize};
use std::fs;
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
    interval: i32,
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
        new.set_path();
        new
    }

    pub fn set_path(&self) {
        //self.all_images.lock().unwrap().clear();
        let mut images = self.all_images.lock().unwrap();
        *images = fs::read_dir(std::env::var("PICTURES_BASE").unwrap())
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
        let metadata = Metadata::new_from_path(path.clone()).unwrap();
        // Get XMP subject
        if let Ok(subjects) = metadata.get_tag_multiple_strings("Xmp.dc.subject") {
            let re = Regex::new(r"\d+-.*").unwrap();
            if let Some(subject) = subjects.iter().filter(|x| re.is_match(x)).next() {
                return Some(subject.clone());
            }
        }
        if let Ok(date_time) = metadata.get_tag_string("Exif.Photo.DateTimeOriginal") {
            if !date_time.is_empty() {
                return Some(date_time);
            }
        }
        if let Ok(date_time) = metadata.get_tag_string("Exif.Image.DateTime") {
            if !date_time.is_empty() {
                return Some(date_time);
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
