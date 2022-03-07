pub mod mongo;
pub use crate::mongo::{connect, find, Product};

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{price}")]
async fn index(params: web::Path<u32>) -> impl Responder {
	let price = params.into_inner();
	let collection = connect();
	let docs = find(&collection, String::from("price"), price);
	return format!("{:#?}!", docs);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| App::new().service(index))
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}
