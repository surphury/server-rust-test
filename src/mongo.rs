use mongodb::{bson::doc, sync::Client};
use serde::{Deserialize, Serialize};

pub fn connect() -> mongodb::sync::Collection<Product> {
	let client = Client::with_uri_str("mongodb://localhost:27017").expect("no database found");
	let database = client.database("market");
	let collection = database.collection::<Product>("products");
	return collection;
}

pub fn insert(docs: Vec<Product>, collection: &mongodb::sync::Collection<Product>) {
	collection
		.insert_many(docs, None)
		.expect("Could not insert data to mongoDB");
}

pub fn find(
	collection: &mongodb::sync::Collection<Product>,
	parametre: String,
	value: u32,
) -> mongodb::sync::Cursor<Product> {
	let cursor = collection
		.find(doc! {parametre.to_string(): value}, None)
		.expect("Error on connection");
	return cursor;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
	name: String,
	price: u32,
}
