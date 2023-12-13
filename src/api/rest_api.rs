use crate::models::user_model::{ NewUser, User };
use crate::repository::mongodb_repo::MongoRepo;

use actix_web::{ post, get, web, HttpResponse };

#[post("/user")]
pub async fn create_user(db: web::Data<MongoRepo>, new_user: web::Json<NewUser>) -> HttpResponse {
	let data = User {
		id: None,
		username: new_user.username.to_owned(),
		password: new_user.password.to_owned(),
	};

	let user_details = db.create_user(data).await;
	match user_details {
		Ok(user) => HttpResponse::Ok().json(user),
		Err(err) => HttpResponse::InternalServerError().body(err.to_string())
	}
}

#[get("/user/{id}")]
pub async fn get_user(db: web::Data<MongoRepo>, path: web::Path<String>) -> HttpResponse {
	let id = path.into_inner();
	if id.is_empty() {
		return HttpResponse::BadRequest().body("Invalid ID");
	}
	let user_details = db.get_user(&id).await;
	match user_details {
		Ok(user) => HttpResponse::Ok().json(user),
		Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
	}
}
