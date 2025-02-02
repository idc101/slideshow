use std::sync::Mutex;

use rand::seq::IndexedRandom;
use std::fs;
use std::path::{Path, PathBuf};

pub struct AppState {
    all_images: Mutex<Vec<PathBuf>>,
    counter: Mutex<i32>,
    // other fields...
}

impl AppState {
    pub fn new() -> Self {
        let new = Self {
            all_images: Mutex::new(Vec::new()),
            counter: Mutex::new(0),
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

    pub fn increment(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
    }

    pub fn get(&self) -> i32 {
        *self.counter.lock().unwrap()
    }
}
