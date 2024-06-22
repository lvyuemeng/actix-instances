use std::{clone, collections::HashMap, hash::Hash, sync::Mutex};

use actix_web::{
    web::{self, delete},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

struct AppState {
    users: Mutex<HashMap<u32, User>>,
}

async fn create_user(appdata: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut users = appdata.users.lock().unwrap();
    let new_id = users.len() as u32 + 1;
    let new_user = User {
        id: new_id,
        name: user.name.to_owned(),
        email: user.email.to_owned(),
    };
    users.insert(new_id, new_user.clone());
    HttpResponse::Created().json(new_user)
}

async fn delete_user()
pub async fn run() -> std::io::Result<()> {
    let appdata = web::Data::new(AppState {
        users: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(appdata.clone())
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
