use std::fs;

use rocket::fs::{relative, NamedFile};
use std::path::{Path, PathBuf};

use rocket::{get, launch, routes};

use rocket::serde::Deserialize;

use rocket::fs::FileServer;

#[macro_use]
extern crate rocket;

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

#[launch]
fn rocket() -> _ {
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

    let rocket = rocket::build();
    let config: Config = rocket.figment().extract().unwrap_or_default();

    rocket
        .mount("/", routes![image])
        .mount("/", FileServer::from(config.dist))
}
