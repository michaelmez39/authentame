use actix_web::{Responder, web, };
use argon2::hash_encoded;

type Storage = Vec<User>;

struct User {
    name: String,
    password: String
}
fn main() {

    println!("Hello, world!");
}

async fn authenticate(json: web::Json<User>) -> impl Responder {
    "You tried to authenticate something.".to_string()
}