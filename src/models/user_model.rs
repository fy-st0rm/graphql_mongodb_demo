use mongodb::bson::oid::ObjectId;
use serde::{ Serialize, Deserialize };

#[derive(juniper::GraphQLInputObject, Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
	pub username: String,
	pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub username: String,
	pub password: String
}

#[juniper::graphql_object(description = "User")]
impl User {
	pub fn username(&self) -> &str {
		self.username.as_str()
	}

	pub fn password(&self) -> &str {
		self.password.as_str()
	}
}
