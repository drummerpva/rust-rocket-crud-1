use rocket::{get, routes};

#[get("/")]
fn hello() -> &'static str {
    "Hello, Mundo!\n"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", routes![hello]).launch().await;
}
