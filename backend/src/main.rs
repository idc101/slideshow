mod slideshow;

use env_logger::Env;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let env = Env::default().filter_or("LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    backend::rocket().await
}
