use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer, Responder};
use argon2::{Config};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{ Mutex};

struct Storage {
    data: Mutex<Vec<User>>
}
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    password: String,
}

impl PartialEq<User> for User {
    fn eq(&self, other: &User) -> bool {
        self.name == other.name
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let database: web::Data<Storage> = web::Data::new(Storage { data: Mutex::new(Vec::new()) }); // this is for the type checker to chill out
    HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .route("/register", web::post().to(register))
            .route("/login", web::get().to(authenticate))
            .route("/list", web::get().to(list))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn list(database: web::Data<Storage>) -> impl Responder {
    let database = database.data.lock().unwrap();
    database.iter().fold(String::new(), |acc, user| acc + &format!("User: {} and Password:{}\n", user.name, user.password))
}

async fn authenticate() -> impl Responder {
    "You tried to authenticate something.".to_string()
}

async fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config)
        .unwrap()
}

async fn register(json: web::Json<User>, database: web::Data<Storage>) -> impl Responder {
    let mut database = database.data.lock().unwrap();
    if !database.contains(&json.0) {
        let new_user = User {
            name: json.0.name,
            password: hash(json.0.password.as_bytes()).await,
        };
        database.push(new_user);
        HttpResponse::new(StatusCode::CREATED)
    } else {
        HttpResponse::new(StatusCode::CONFLICT)
    }
}
