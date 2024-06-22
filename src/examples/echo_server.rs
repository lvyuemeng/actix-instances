use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn hello() -> impl Responder {
    "Hello, World!"
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/echo", web::post().to(echo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}