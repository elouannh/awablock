use actix_web::{ web, HttpResponse, Result };
use mongodb::{bson::{doc, Document}, Client, Collection};
use jsonwebtoken::{ encode, EncodingKey, Header };
use std::env::var;

use crate::structs::claims as claims_struct;
use crate::structs::user as user_struct;

pub async fn post_login(
	client: web::Data<Client>,
	user: web::Json<user_struct::User>
) -> Result<HttpResponse, Box<dyn std::error::Error>>
{
	let collection: Collection<user_struct::User> = client.database("awablock").collection("user");

	let filter: Document = doc! { "username": &user.username };
	let found_user = collection.find_one(filter).await;

	if let Ok(Some(found_user)) = found_user
	{
		return if found_user.password == user.password
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
