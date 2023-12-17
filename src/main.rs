mod auth;
use auth::BasicAuth;
use rocket::{
    catch, catchers, delete, get, post, put,
    response::status,
    routes,
    serde::json::{json, Value},
};

#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth) -> Value {
    json!([
        {
            "id": 1,
            "name": "John Doe"
        },
        {
            "id": 2,
            "name": "Jane Doe"
        }
    ])
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!([
        {
            "id": id,
            "name": "John Doe",
            "email": "john@doe.com"
        }
    ])
}

#[post("/rustaceans", format = "json")]
fn create_rustacean(_auth: BasicAuth) -> Value {
    json!([
        {
            "id": 3,
            "name": "John Connor",
            "email": "john@connor.com"
        }
    ])
}
#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!([
        {
            "id": id,
            "name": "John Alterado",
            "email": "john@connor.com"
        }
    ])
}
#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}
#[catch(404)]
fn not_found() -> Value {
    json!("Not Found!")
}
#[catch(401)]
fn unauthorized() -> Value {
    json!("Unauthorized!")
}
#[catch(422)]
fn invalid_param() -> Value {
    json!("Unprocessable Entity!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .register("/", catchers![not_found, invalid_param, unauthorized])
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .launch()
        .await;
}
