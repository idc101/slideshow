use std::sync::Mutex;

use chrono::NaiveDateTime;
use chrono::TimeZone;
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
    pictures_base: Mutex<PathBuf>, // Added pictures_base
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
            pictures_base: Mutex::new(PathBuf::new()), // Initialize pictures_base
        };
        new
    }

    pub fn set_seed(&self) {
        let mut rng = self.rng.lock().unwrap();
        *rng = StdRng::seed_from_u64(0);
    }

    pub fn set_path(&self, pictures_base: PathBuf) {
        let mut images = self.all_images.lock().unwrap();
        let mut base = self.pictures_base.lock().unwrap(); //save pictures_base
        *base = pictures_base.clone();

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

        let description =
            Self::filename_to_description(path.clone(), self.pictures_base.lock().unwrap().clone());

        match exifreader.read_from_container(&mut bufreader) {
            Ok(exif) => {
                let date = exif
                    .get_field(Tag::DateTimeOriginal, In::PRIMARY)
                    .or(exif.get_field(Tag::DateTime, In::PRIMARY))
                    .map(|field| field.display_value().to_string())
                    .map(|s| {
                        println!("Date: {}", s);
                        let datetime =
                            NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap();
                        format!("{}", datetime.format("%a %d %h %Y"))
                    });

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

    fn filename_to_description(filename: PathBuf, pictures_base: PathBuf) -> Option<String> {
        let base_name = filename.strip_prefix(pictures_base).unwrap().to_str();

        if base_name.is_none() {
            return None;
        }

        let unwrapped = base_name.unwrap();
        let no_extension = regex::Regex::new(r"\..+$")
            .unwrap()
            .replace_all(&unwrapped, "");
        let no_date = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}[ -]")
            .unwrap()
            .replace_all(&no_extension, "");
        let no_year_month = regex::Regex::new(r"^\d{4}-\d{2}[ -]")
            .unwrap()
            .replace_all(&no_date, "");
        let no_datetime = regex::Regex::new(r"\d{8}T\d{6}[-]")
            .unwrap()
            .replace_all(&no_year_month, "");
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
        assert_eq!(metadata0.date.unwrap(), "Mon 28 Oct 2024");
        assert_eq!(metadata1.date.unwrap(), "Sun 16 Feb 2025");
    }

    #[test]
    fn test_filename_to_description() {
        let pictures_base = PathBuf::from_str("/base/path").unwrap();
        assert_eq!(
            AppState::filename_to_description(
                PathBuf::from_str("/base/path/2025-02 Skiing - La Rosiere/IMG_9969.jpg").unwrap(),
                pictures_base.clone()
            ),
            Some("Skiing - La Rosiere".to_string())
        );
        assert_eq!(
            AppState::filename_to_description(
                PathBuf::from_str("/base/path/2024-12 December-20241224T100546-019.jpg").unwrap(),
                pictures_base.clone()
            ),
            Some("December".to_string())
        );
        assert_eq!(
            AppState::filename_to_description(
                PathBuf::from_str("/base/path/2024-12-25 Christmas-20241224T100546-019.jpg")
                    .unwrap(),
                pictures_base.clone()
            ),
            Some("Christmas".to_string())
        );
    }

    #[test]
    fn test_get_image() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");

        let state = AppState::new();
        state.set_path(d);

        let image0 = state.get_image(0);
        let image1 = state.get_image(1);

        assert!(image0
            .to_string_lossy()
            .ends_with("2024-10 Rome/20241028T100833-061.jpg"));
        assert!(image1
            .to_string_lossy()
            .ends_with("2025-02 Skiing -IMG_0325.jpg"));
    }
}
