/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   greet.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ehosta <ehosta@student.42lyon.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/08/21 20:17:52 by ehosta            #+#    #+#             */
/*   Updated: 2024/08/21 20:17:52 by ehosta           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn greet() -> impl Responder {
	HttpResponse::Ok().body("Hello, world!")
}
