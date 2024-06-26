use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::{
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
};

#[derive(Clone)]
struct AppState {
    local_count: Cell<usize>,
    global_count: Arc<AtomicUsize>,
}

#[get("/")]
async fn show_count(data: web::Data<AppState>) -> impl Responder {
    let res = format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    );

    HttpResponse::Ok().body(res)
}

#[get("/add")]
async fn add_one(data: web::Data<AppState>) -> impl Responder {
    data.global_count.fetch_add(1, Ordering::Relaxed);

    let local_count = data.local_count.get();
    data.local_count.set(local_count + 1);

    let res = format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    );

    HttpResponse::Ok().body(res)
}

pub async fn run() -> std::io::Result<()> {
    let data = AppState {
        local_count: Cell::new(0),
        global_count: Arc::new(AtomicUsize::new(0)),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .service(show_count)
            .service(add_one)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
