use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn greet() -> impl Responder {
	HttpResponse::Ok().body("Hello, world!")
}
