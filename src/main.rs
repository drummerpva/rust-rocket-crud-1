use rocket::{
    get, routes,
    serde::json::{json, Value},
};

#[get("/")]
fn hello() -> Value {
    json!({"message":"Hello, Mundo!"})
}

#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", routes![hello]).launch().await;
}
