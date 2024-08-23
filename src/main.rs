use actix_web::{ web, App, HttpServer };
use mongodb::Client;

mod routes;
mod services;
mod structs;

use crate::routes::health as health_route;
use crate::routes::login as login_route;
use crate::services::env_manager as env_manager_service;

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    let uri: String = env_manager_service::get_env_var_as_str("MONGODB_URI", "empty");
    let client: Client = Client::with_uri_str(uri).await.unwrap();
	HttpServer::new(move || {
		App::new()
			.app_data(web::Data::new(client.clone()))
			.route("/health", web::get().to(health_route::get_health))
			.route("/login", web::post().to(login_route::post_login))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
