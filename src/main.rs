use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer, Responder};
use argon2::{Config};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{ Mutex};
type Storage = Mutex<Vec<User>>;

#[derive(Serialize, Deserialize, PartialEq)]
struct User {
    name: String,
    password: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let database: Storage = Mutex::new(Vec::new()); // this is for the type checker to chill out
        App::new()
            .data(database)
            .route("/register", web::post().to(register))
            .route("/login", web::get().to(authenticate))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn authenticate(_json: web::Json<User>) -> impl Responder {
    "You tried to authenticate something.".to_string()
}

pub fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config)
        .unwrap()
}

async fn register(json: web::Json<User>, database: web::Data<Storage>) -> impl Responder {
    let mut database = database.lock().unwrap();
    if !database.contains(&json.0) {
        database.push(User {
            name: json.0.name,
            password: hash(json.0.password.as_bytes()),
        });
        HttpResponse::new(StatusCode::CREATED)
    } else {
        HttpResponse::new(StatusCode::BAD_REQUEST)
    }
}
