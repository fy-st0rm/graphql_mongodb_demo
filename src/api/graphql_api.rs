use actix_web::{ get, post, web, HttpResponse, Error };
use juniper::http::{GraphQLRequest, graphiql::graphiql_source};

use crate::repository::graphql_repo::{ Schema, Context };
use crate::repository::mongodb_repo::MongoRepo;

#[post("/graphql")]
pub async fn graphql(
	schema: web::Data<Schema>,
	mongo_repo: web::Data<MongoRepo>,
	payload: web::Json<GraphQLRequest>
) -> Result<HttpResponse, Error> {
	let context = Context {
		db: mongo_repo.into_inner()
	};
	let res = payload.execute(&schema, &context).await;

	match serde_json::to_string(&res) {
		Ok(data) => {
			Ok(HttpResponse::Ok()
				.content_type("application/json")
				.body(data))
		}
		Err(err) => {
			Ok(HttpResponse::BadRequest()
				.body(err.to_string()))
		}
	}
}

#[get("/graphiql")]
pub async fn graphiql() -> HttpResponse {
	let html = graphiql_source("http://127.0.0.1:3000/graphql", Some("/subscription"));
	HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(html)
}
