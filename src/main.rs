mod api;
mod models;
mod repository;

use actix_web::{ web, App, HttpServer, middleware::Logger };

use repository::graphql_repo::create_schema;
use repository::mongodb_repo::MongoRepo;

use api::graphql_api::{ graphql, graphiql };
use api::rest_api::{ get_user, create_user };


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	if std::env::var_os("RUST_LOG").is_none() {
		std::env::set_var("RUST_LOG", "info");
	}
	env_logger::init();

	let schema = create_schema();
	let sd = web::Data::new(schema);
	let mongo_repo = MongoRepo::init().await;
	let md = web::Data::new(mongo_repo);

	HttpServer::new( move || {
		App::new()
			.wrap(Logger::default())
			.app_data(sd.clone())
			.app_data(md.clone())
			.service(graphql)
			.service(graphiql)
			.service(create_user)
			.service(get_user)
	})
	.bind(("127.0.0.1", 3000))?
	.run()
	.await
}
