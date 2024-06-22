use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Info {
    name: String,
    age: u32,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

async fn greet(info: web::Json<Info>) -> impl Responder {
    let response = Response {
        message: format!("Hello, {}! You are {} years old.", info.name, info.age),
    };
    HttpResponse::Ok().json(response)
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/greet", web::post().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}