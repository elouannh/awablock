/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   user.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ehosta <ehosta@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/08/21 20:18:03 by ehosta            #+#    #+#             */
/*   Updated: 2024/08/21 20:18:03 by ehosta           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use actix_web::{get, HttpResponse, Responder};

#[get("/user")]
async fn user() -> impl Responder {
	let mongodb_uri: Result<String, std::env::VarError> = std::env::var("MONGODB_URI");
	println!("{:?}", std::env::var("MONGODB_URI"));

	match mongodb_uri {
		Ok(uri) => {
			HttpResponse::Ok().body(uri)
		},
		Err(_) => {
			HttpResponse::InternalServerError().body("Error.")
		}
	}
}
