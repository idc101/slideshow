use std::sync::Mutex;

use exif::Reader as ExifReader;
use exif::{In, Tag};
use rand::rngs::StdRng;
use rand::seq::IndexedRandom;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct AppState {
    all_images: Mutex<Vec<PathBuf>>,
    counter: Mutex<i32>,
    rng: Mutex<StdRng>,
    pub settings: Mutex<Settings>,
}

// Define the data structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    slideshow: String,
    pub interval: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageMetadata {
    pub date: Option<String>,
    pub description: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        let new = Self {
            all_images: Mutex::new(Vec::new()),
            counter: Mutex::new(0),
            rng: Mutex::new(StdRng::from_os_rng()),
            settings: Mutex::new(Settings {
                slideshow: "slideshow".to_string(),
                interval: 300,
            }),
        };
        new
    }

    pub fn set_seed(&self) {
        let mut rng = self.rng.lock().unwrap();
        *rng = StdRng::seed_from_u64(0);
    }

    pub fn set_path(&self, pictures_base: PathBuf) {
        let mut images = self.all_images.lock().unwrap();

        images.clear();
        *images = WalkDir::new(pictures_base)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|x| {
                x.path()
                    .extension()
                    .unwrap_or_default()
                    .to_ascii_lowercase()
                    == "jpg"
            })
            .map(|x| x.path().to_path_buf())
            .collect();
    }

    pub fn get_current_image(&self) -> PathBuf {
        let images = self.all_images.lock().unwrap();
        let mut rng = self.rng.lock().unwrap();
        images.choose(&mut rng).unwrap().clone()
    }

    pub fn get_image(&self, num: i32) -> PathBuf {
        let images = self.all_images.lock().unwrap();
        images.get((num as usize) % images.len()).unwrap().clone()
    }

    pub fn get_image_metadata(&self, num: i32) -> ImageMetadata {
        let path = self.get_image(num);
        let file = fs::File::open(path.clone()).unwrap();
        let mut bufreader = BufReader::new(file);
        let exifreader = ExifReader::new();

        let description = Self::filename_to_description(path.clone());

        match exifreader.read_from_container(&mut bufreader) {
            Ok(exif) => {
                let date = exif
                    .get_field(Tag::DateTimeOriginal, In::PRIMARY)
                    .or(exif.get_field(Tag::DateTime, In::PRIMARY))
                    .map(|field| field.display_value().to_string());

                return ImageMetadata { date, description };
            }
            Err(_) => {
                return ImageMetadata {
                    date: None,
                    description,
                };
            }
        }
    }

    pub fn increment(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
    }

    pub fn get(&self) -> i32 {
        *self.counter.lock().unwrap()
    }

    fn filename_to_description(filename: PathBuf) -> Option<String> {
        let without_extension = filename
            .file_stem()
            .map(|f| f.to_str().map(|s| s.to_string()))
            .flatten();

        if without_extension.is_none() {
            return None;
        }

        let unwrapped = without_extension.unwrap();
        let no_date = regex::Regex::new(r"^\d{4}-\d{2}[ -]")
            .unwrap()
            .replace_all(unwrapped.as_str(), "");
        let no_datetime = regex::Regex::new(r"\d{8}T\d{6}[-]")
            .unwrap()
            .replace_all(&no_date, "");
        let no_three_digits = regex::Regex::new(r"-\d{3}$")
            .unwrap()
            .replace_all(&no_datetime, "");
        let no_img = regex::Regex::new(r"IMG_\d{4}")
            .unwrap()
            .replace_all(&no_three_digits, "");
        let no_dcs = regex::Regex::new(r"DCS_\d{4}")
            .unwrap()
            .replace_all(&no_img, "");

        let alnum_regex = regex::Regex::new(r"^[^a-zA-Z0-9]+|[^a-zA-Z0-9]+$").unwrap();
        return Some(alnum_regex.replace_all(&no_dcs, "").to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, str::FromStr};

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

        let metadata0 = state.get_image_metadata(0);
        let metadata1 = state.get_image_metadata(1);
        assert_eq!(metadata0.date.unwrap(), "2024-10-28 10:08:33");
        assert_eq!(metadata1.date.unwrap(), "2025-02-16 11:14:53");
    }

    #[test]
    fn test_filename_to_description() {
        assert_eq!(
            AppState::filename_to_description(
                PathBuf::from_str("2025-02 Skiing - La Rosiere-IMG_9969.jpg").unwrap()
            ),
            Some("Skiing - La Rosiere".to_string())
        );
        assert_eq!(
            AppState::filename_to_description(
                PathBuf::from_str("2024-12 December-20241224T100546-019.jpg").unwrap()
            ),
            Some("December".to_string())
        );
    }
}
