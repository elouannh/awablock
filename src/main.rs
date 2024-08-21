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

use actix_web::{App, HttpServer};
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.service(routes::greet::greet)
			.service(routes::user::user)
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
