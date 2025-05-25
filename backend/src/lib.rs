use std::path::{Path, PathBuf};
use std::sync::Arc;

use rocket::fs::{FileServer, NamedFile};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::{catch, catchers, get, post, routes, Request, State};

use rocket::tokio;
use tokio::time::{interval, Duration};

pub mod slideshow;
use slideshow::{AppState, Settings};

async fn background_task(state: Arc<AppState>) {
    loop {
        let interval_duration = {
            let settings = state.settings.lock().unwrap();
            settings.interval
        };

        let mut interval = interval(Duration::from_secs(interval_duration as u64));

        interval.tick().await;
        // Your code here
        state.increment();
    }
}

#[get("/test")]
fn test(state: &State<Arc<AppState>>) -> String {
    let count = state.get();
    format!("Counter: {}", count)
}

// GET endpoint to retrieve all settings
#[get("/settings")]
fn get_settings(state: &State<Arc<AppState>>) -> Json<Settings> {
    let items = state.settings.lock().unwrap();
    Json(items.clone())
}

// POST endpoint to add a new item
#[post("/settings", format = "json", data = "<new_settings>")]
fn update_settings(new_settings: Json<Settings>, state: &State<Arc<AppState>>) -> String {
    let mut settings = state.settings.lock().unwrap();
    *settings = new_settings.into_inner();
    "Ok".to_string()
}

#[get("/image/<num>")]
pub async fn image(num: i32, state: &State<Arc<AppState>>) -> NamedFile {
    let image = state.get_image(num);
    println!("{}", image.display());

    NamedFile::open(image).await.ok().unwrap()
}

#[get("/image/<num>/metadata")]
pub async fn image_metadata(num: i32, state: &State<Arc<AppState>>) -> String {
    let metadata = state.get_image_metadata(num).unwrap_or_default();
    println!("Metadata: {}", metadata);
    metadata
}

#[catch(404)]
async fn not_found(_: &Request<'_>) -> Result<NamedFile, NotFound<String>> {
    NamedFile::open(Path::new("static/dist/index.html"))
        .await
        .map_err(|e| NotFound(e.to_string()))
}

pub async fn rocket() -> Result<(), rocket::Error> {
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

    let app_state = Arc::new(AppState::new());
    let state_clone = app_state.clone();
    let state_clone2 = app_state.clone();

    let rocket = rocket::build();
    let config: Config = rocket.figment().extract().unwrap_or_default();

    let rocket = rocket
        .manage(app_state)
        .mount(
            "/api",
            routes![image, image_metadata, get_settings, update_settings],
        )
        .mount("/", FileServer::from(config.dist))
        .register("/", catchers![not_found])
        .ignite()
        .await?;

    let pictures_base = std::env::var_os("PICTURES_BASE")
        .map(|s| PathBuf::from(s))
        .unwrap_or_else(|| PathBuf::from("rust-hw"));
    state_clone2.set_path(pictures_base);

    // Spawn the background task
    rocket::tokio::spawn(async move {
        background_task(state_clone).await;
    });

    rocket.launch().await?;

    Ok(())
}
