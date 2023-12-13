use dotenv::dotenv;
use mongodb::{
	bson::{ oid::ObjectId, doc },
	results::InsertOneResult,
	Client, Collection
};
use futures::stream::StreamExt;
use actix_web::Error;

use crate::models::user_model::User;

#[derive(Debug, Clone)]
pub struct MongoRepo {
	pub col: Collection<User>
}

impl MongoRepo {
	pub async fn init() -> Self {
		dotenv().ok();
		let uri = std::env::var("MONGOURI")
			.expect("MONGOURI env should be set.");

		let client = Client::with_uri_str(uri).await.unwrap();
		let db = client.database("database");
		let col: Collection<User> = db.collection("User");
		MongoRepo { col }
	}

	pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
		let user = self
			.col
			.insert_one(new_user, None)
			.await
			.ok()
			.expect("Failed to create user");

		Ok(user)
	}

	pub async fn get_user(&self, id: &String) -> Result<User, Error> {
		let filter = doc! {
			"_id": ObjectId::parse_str(id).unwrap()
		};
		let user_details = self
			.col
			.find_one(filter, None)
			.await
			.ok()
			.expect("Error getting user's detail");

		Ok(user_details.unwrap())
	}

	pub async fn get_all_user(&self) -> Result<Vec<User>, Error> {
		let mut user_details: mongodb::Cursor<User> = self
			.col
			.find(None, None)
			.await
			.ok()
			.expect("Error getting all user's detail");

		let mut users = Vec::new();
		while let Some(user) = user_details.next().await {
			users.push(user.unwrap());
		}

		Ok(users)
	}
}
