use std::sync::Mutex;

use rand::seq::IndexedRandom;
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
                interval: 10,
            }),
        };
        new.set_path();
        new
    }

    pub fn set_path(&self) {
        //self.all_images.lock().unwrap().clear();
        let mut images = self.all_images.lock().unwrap();
        *images = fs::read_dir("/Users/iain/Mylio/Apple Photos/Iains iPhone Library")
            .unwrap()
            .filter(|x| x.as_ref().unwrap().path().extension().unwrap() == "jpg")
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

    pub fn increment(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
    }

    pub fn get(&self) -> i32 {
        *self.counter.lock().unwrap()
    }
}
