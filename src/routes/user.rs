use actix_web::{get, HttpResponse, Responder};

#[get("/user")]
async fn user() -> impl Responder {
	HttpResponse::Ok().body("You currently are a cool human.")
}
