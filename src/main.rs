mod examples {
    pub mod count_show;
    pub mod echo_server;
    pub mod json_work;
    pub mod user_api;
}

mod error_mid {
    pub mod errors;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //examples::count_show::run().await
    //examples::echo_server::run().await
    //examples::json_work::run().await
    examples::user_api::run().await
}
