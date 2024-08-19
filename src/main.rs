use actix_web::{App, HttpServer};
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.service(routes::greet::greet)
			.service(routes::user::user)
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
