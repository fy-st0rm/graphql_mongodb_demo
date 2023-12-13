use juniper::{ RootNode, FieldResult, EmptySubscription };
use mongodb::results::InsertOneResult;
use std::sync::Arc;

use crate::models::user_model::{ NewUser, User };
use crate::repository::mongodb_repo::MongoRepo;

pub struct Context {
	pub db: Arc<MongoRepo>
}
impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
	pub async fn apiVersion() -> &str {
		"1.0"
	}

	async fn get_all_users(context: &Context) -> FieldResult<Vec<User>> {
		let db = &context.db;
		Ok(db.get_all_user().await?)
	}
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
	async fn create_user(new_user: NewUser, context: &Context) -> FieldResult<User> {
		let db = &context.db;
		let user = User {
			id: None,
			username: new_user.username.to_owned(),
			password: new_user.password.to_owned()
		};

		let inserted: InsertOneResult = db.create_user(user).await.unwrap();
		let id = inserted.inserted_id.as_object_id().unwrap().to_string();
		let user = db.get_user(&id).await?;
		Ok(user)
	}
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
	Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
