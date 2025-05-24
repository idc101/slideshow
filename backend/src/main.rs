mod slideshow;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    backend::rocket().await
}
