/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ehosta <ehosta@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/08/21 20:18:07 by ehosta            #+#    #+#             */
/*   Updated: 2024/08/21 20:18:07 by ehosta           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use actix_web::{ App, HttpServer };
use mongodb::{ bson::{ doc, Document }, Client, Collection };
mod routes;

fn get_env_var_as_str<'a>(var_name: &'a str, default: &'a str) -> String {
    match std::env::var(var_name) {
        Ok(value) => {
            String::from(value)
        },
        Err(_) => {
            String::from(default)
        },
    }
}

async fn db() -> mongodb::error::Result<()>
{
    let uri: String = get_env_var_as_str("MONGODB_URI", "empty");
    let client: Client = Client::with_uri_str(uri).await?;
    let database: mongodb::Database = client.database("sample_mflix");
    let my_coll: Collection<Document> = database.collection("movies");
    let my_movie: Option<Document> = my_coll.find_one(doc! { "title": "The Perils of Pauline" }).await?;
    println!("Found a movie:\n{:#?}", my_movie);
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	let _ = db().await;
	HttpServer::new(|| {
		App::new()
			.service(routes::greet::greet)
			.service(routes::user::user)
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
