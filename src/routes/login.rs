use actix_web::{
	web,
	HttpResponse,
	Result
};
use mongodb::{
	bson::{
		doc,
		Document
	},
	error::Result as MongoResult,
	Client,
	Collection
};
use jsonwebtoken::{
	encode,
	EncodingKey,
	Header
};
use std::env::var;

use crate::services::pw_hash as pw_hash_service;
use crate::structs::claims as claims_struct;
use crate::structs::user as user_struct;

pub async fn post_login(
	client: web::Data<Client>,
	user: web::Json<user_struct::User>
) -> Result<HttpResponse, Box<dyn std::error::Error>>
{
	let collection: Collection<user_struct::User> = client.database("awablock").collection("user");

	let filter: Document = doc! { "username": &user.username };
	let found_user: MongoResult<Option<user_struct::User>> = collection.find_one(filter).await;

	if let Ok(Some(_found_user)) = found_user
	{
		let user_instance: user_struct::User = user_struct::User {
			username: **user.username,
			password: **user.password
		};
		let client_instance: Client = Client::from(****client);
		return if pw_hash_service::verify_hash(client_instance, user_instance)
		{
			let my_claims: claims_struct::Claims = claims_struct::Claims { sub: user.username.clone(), exp: 10000000000 };
			let key: String = var("SECRET_KEY").expect("SECRET_KEY must be set");
			let token: String = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key.as_ref()))?;
			Ok(HttpResponse::Ok().json(token))
		}
		else
		{
			Ok(HttpResponse::Unauthorized().body("Invalid credentials."))
		}
	}

	Ok(HttpResponse::Unauthorized().body("Invalid credentials"))
}
