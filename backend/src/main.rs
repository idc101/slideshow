use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rocket::fs::{FileServer, NamedFile};
use rocket::serde::Deserialize;
use rocket::{get, routes, State};

use rocket::tokio;
use tokio::time::{interval, Duration};

// #[macro_use]
// extern crate rocket;

// Your state structure
struct AppState {
    counter: Mutex<i32>,
    // other fields...
}

async fn background_task(state: Arc<AppState>) {
    let mut interval = interval(Duration::from_secs(2)); // 5 minutes = 300 seconds

    loop {
        interval.tick().await;
        // Your code here
        println!("Running background task...");
        *state.counter.lock().unwrap() += 1;
    }
}

#[get("/test")]
fn test(state: &State<Arc<AppState>>) -> String {
    let count = state.counter.lock().unwrap();
    format!("Counter: {}", count)
}

#[get("/")]
fn index() -> String {
    let paths: Vec<Result<fs::DirEntry, std::io::Error>> = fs::read_dir("./").unwrap().collect();

    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }

    // let x = paths
    //     .last()
    //     .as_mut()
    //     .unwrap()
    //     .unwrap()
    //     .path()
    //     .display()
    //     .to_string();
    // x
    //String::from("Hello, world!")
    // let t: Option<&Result<fs::DirEntry, std::io::Error>> = paths.get(1);
    // let t2: Result<&fs::DirEntry, &std::io::Error> = t.unwrap().as_ref();
    // t2.unwrap().path().display().to_string()
    paths[1].as_ref().unwrap().path().display().to_string()
}

#[get("/image")]
pub async fn image() -> NamedFile {
    let images: Vec<Result<fs::DirEntry, std::io::Error>> =
        fs::read_dir("/Users/iain/Mylio/Apple Photos/Iains iPhone Library")
            .unwrap()
            .filter(|x| x.as_ref().unwrap().path().extension().unwrap() == "jpg")
            .collect();
    let image = images[2].as_ref().unwrap().path();
    println!("{}", image.display());

    NamedFile::open(image).await.ok().unwrap()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    #[derive(Deserialize)]
    #[serde(crate = "rocket::serde")]
    struct Config {
        dist: PathBuf,
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                dist: "static/dist".into(),
            }
        }
    }

    let app_state = Arc::new(AppState {
        counter: Mutex::new(0),
    });
    let state_clone = app_state.clone();

    let rocket = rocket::build();
    let config: Config = rocket.figment().extract().unwrap_or_default();

    let rocket = rocket
        .manage(app_state)
        .mount("/", routes![image, test])
        .mount("/", FileServer::from(config.dist))
        .ignite()
        .await?;

    // Spawn the background task
    rocket::tokio::spawn(async move {
        background_task(state_clone).await;
    });

    rocket.launch().await?;

    Ok(())
}
