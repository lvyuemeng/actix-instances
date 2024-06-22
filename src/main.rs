mod examples{
	pub mod echo_server;
	pub mod count_show;
	pub mod json_work;
	pub mod user_api;
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	//examples::count_show::run().await
	//examples::echo_server::run().await
	examples::json_work::run().await
}