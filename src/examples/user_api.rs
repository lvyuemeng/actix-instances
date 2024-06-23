use actix_web::middleware::Logger;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web, App, HttpResponse, HttpServer,
};
use env_logger::Env;
use futures::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    future::{ready, Ready},
    sync::Mutex,
};

use crate::error_mid::errors::UserError;

#[derive(Deserialize)]
struct PaginationParams {
    page: Option<usize>,
    per_page: Option<usize>,
}

#[derive(Serialize)]
struct PaginatedResponse<T> {
    data: Vec<T>,
    total: usize,
    page: usize,
    per_page: usize,
    total_pages: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct UserRequest {
    name: String,
    email: String,
}

struct AppState {
    users: Mutex<HashMap<u32, User>>,
}

fn populate_sample_data(appdata: &web::Data<AppState>) {
    let mut users = appdata.users.lock().unwrap();

    for i in 1..=100 {
        let user = User {
            id: i,
            name: format!("User {}", i),
            email: format!("user{}@example.com", i),
        };
        users.insert(i, user);
    }
}

async fn create_user(
    appdata: web::Data<AppState>,
    user_req: web::Json<UserRequest>,
) -> Result<HttpResponse, UserError> {
    let mut users = appdata
        .users
        .lock()
        .map_err(|_| UserError::InternalServerError)?;
    let new_id = users.len() as u32 + 1;
    let new_user = User {
        id: new_id,
        name: user_req.name.to_owned(),
        email: user_req.email.to_owned(),
    };
    users.insert(new_id, new_user.clone());
    Ok(HttpResponse::Created().json(new_user))
}
async fn get_users(
    appdata: web::Data<AppState>,
    web::Query(params): web::Query<PaginationParams>,
) -> Result<HttpResponse, UserError> {
    let users = appdata
        .users
        .lock()
        .map_err(|_| UserError::InternalServerError)?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    let total = users.len();
    let total_pages = (total as f64 / per_page as f64).ceil() as usize;

    let start = (page - 1) * per_page;

    let data: Vec<&User> = users.values().skip(start).take(per_page).collect();

    let response = PaginatedResponse {
        data,
        total,
        page,
        per_page,
        total_pages,
    };
    Ok(HttpResponse::Ok().json(response))
}

async fn get_user(
    appdata: web::Data<AppState>,
    path: web::Path<u32>,
) -> Result<HttpResponse, UserError> {
    let user_id = path.into_inner();
    let users = appdata
        .users
        .lock()
        .map_err(|_| UserError::InternalServerError)?;
    match users.get(&user_id) {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(UserError::NotFound),
    }
}

async fn update_user(
    appdata: web::Data<AppState>,
    path: web::Path<u32>,
    user_req: web::Json<UserRequest>,
) -> Result<HttpResponse, UserError> {
    let user_id = path.into_inner();
    let user_req = user_req.into_inner();
    let user = User {
        id: user_id,
        name: user_req.name.to_owned(),
        email: user_req.email.to_owned(),
    };
    let mut users = appdata
        .users
        .lock()
        .map_err(|_| UserError::InternalServerError)?;
    users.insert(user_id, user.clone());
    Ok(HttpResponse::Ok().json(user))
}

async fn delete_user(
    appdata: web::Data<AppState>,
    path: web::Path<u32>,
) -> Result<HttpResponse, UserError> {
    let user_id = path.into_inner();
    let mut users = appdata
        .users
        .lock()
        .map_err(|_| UserError::InternalServerError)?;
    match users.remove(&user_id) {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(UserError::NotFound),
    }
}

// A possible implementation of Logger
pub struct LoggerOwn;

impl<S, B> Transform<S, ServiceRequest> for LoggerOwn
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = LoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggerMiddleware { service }))
    }
}

pub struct LoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Request: {} {}", req.method(), req.path());
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            println!("Response: {}", res.status());
            Ok(res)
        })
    }
}

pub async fn run() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let appdata = web::Data::new(AppState {
        users: Mutex::new(HashMap::new()),
    });

    populate_sample_data(&appdata);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
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
