use argon2::{
	password_hash::{
		rand_core::OsRng,
		PasswordHash,
		PasswordHasher,
		PasswordVerifier,
		SaltString
	},
	Argon2,
};
use mongodb::{
	bson::{
		doc,
		Bson::Document
	},
	Client,
	Collection
};
use std::error::Error;
use crate::structs::user as user_struct;

pub async fn verify_hash(
	client: Client,
	user: user_struct::User,
) -> Result<bool, Box<dyn Error>>
{
	let collection: Collection<user_struct::User> = client.database("awablock").collection("user");

	let filter: Document = doc! { "username": user.username };
	let result: Option<user_struct::User> = collection.find_one(filter).await?;

	if result.is_none()
	{
		return Ok(false);
	}

	let user_doc: user_struct::User = result.unwrap();
	let hashed_password: String = user_doc.password;

	let argon_instance: PasswordHash = PasswordHash::new(&hashed_password)?;
	let algorithms: &[&dyn PasswordVerifier] = &[&Argon2::default()];

	Ok(argon_instance.verify_password(algorithms, user.password.as_bytes()).is_ok())
}

pub async fn hash_pw(pw: &str) -> Result<String, Box<dyn Error>>
{
	let salt: SaltString = SaltString::generate(&mut OsRng);
	let argon2: Argon2 = Argon2::default();

	let password_hash: String = argon2.hash_password(pw.as_bytes(), &salt)?.to_string();
	Ok(password_hash.to_string())
}

pub async fn pw_match(pw1: &str, hashed_pw: &str) -> Result<bool, Box<dyn Error>>
{

}