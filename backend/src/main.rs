use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use rocket::fs::{FileServer, NamedFile};
use rocket::serde::Deserialize;
use rocket::{get, routes, State};

use rocket::tokio;
use tokio::time::{interval, Duration};

mod slideshow;
use slideshow::AppState;

async fn background_task(state: Arc<AppState>) {
    let mut interval = interval(Duration::from_secs(2)); // 5 minutes = 300 seconds

    loop {
        interval.tick().await;
        // Your code here
        println!("Running background task...");
        state.increment();
    }
}

#[get("/test")]
fn test(state: &State<Arc<AppState>>) -> String {
    let count = state.get();
    format!("Counter: {}", count)
}

#[get("/image")]
pub async fn image(state: &State<Arc<AppState>>) -> NamedFile {
    let image = state.get_current_image();
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

    let app_state = Arc::new(AppState::new());
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
