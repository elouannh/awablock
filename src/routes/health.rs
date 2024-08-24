use actix_web::{
	HttpResponse
};

pub async fn get_health() -> actix_web::Result<HttpResponse, Box<dyn std::error::Error>>
{
	Ok(HttpResponse::Ok().body("Server is alive."))
}