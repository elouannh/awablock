use actix_web::{
	web,
	HttpResponse,
	Result
};
use mongodb::{
	bson::{
		doc,
		Bson::Document
	},
	Client,
	Collection,
};

use crate::structs::user as user_struct;

async fn post_register(
	client: web::Data<Client>,
	user: web::Json<user_struct::User>
) -> Result<HttpResponse, Box<dyn std::error::Error>>
{
	let collection: Collection<user_struct::User> = client.database("awablock").collection("user");

	let filter: Document = doc! { "username": &user.username };
	let existing_user = collection.find_one(filter.clone()).await.unwrap();

	if existing_user.is_some()
	{
		return Ok(HttpResponse::BadRequest().body("Username already taken."))
	}

	let salt: [u8; 32] = rand::random();
	let hashed_password = 0;

	Ok(HttpResponse::Ok().body("User registered successfully"))
}