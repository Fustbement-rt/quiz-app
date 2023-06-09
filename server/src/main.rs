use axum::{
	http::StatusCode,
	routing::{get, post},
	Json, Router,
};

use argon2::{
	password_hash::{rand_core::OsRng, SaltString},
	Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use diesel::prelude::*;
use diesel_async::{
	pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use unicode_normalization::UnicodeNormalization;
use util::{internal_error, DatabaseConnection};

use crate::{auth::generate_login_jwt, models::User, util::create_blank_error};

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

mod app_api;
mod auth;
mod models;
mod schema;
mod util;

#[derive(Serialize)]
pub struct JsonError {
	pub code: u16,
	pub canon: String,
	pub string: Option<String>,
}

type Rejection = (StatusCode, Json<JsonError>);

#[tokio::main]
async fn main() {
	let _ = dotenv();
	tracing_subscriber::registry()
		.with(tracing_subscriber::fmt::layer())
		.init();
	// build our application with a single route

	let db_url = std::env::var("DATABASE_URL").unwrap();

	// set up connection pool
	let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
	let pool = bb8::Pool::builder().build(config).await.unwrap();
	let app = Router::new()
		.route("/register", post(register))
		.route("/login", post(login))
		.route("/user_info", get(app_api::user_info))
		.route("/quizes", get(app_api::quizes))
		.route("/quizes/:id", get(app_api::quiz))
		.route("/quizes/:id/questions", get(app_api::quiz_questions))
		.route("/quizes/:id/flashcards", get(app_api::quiz_flashcards))
		.layer(CorsLayer::very_permissive())
		.with_state(pool);

	// run it with hyper on localhost:3000
	axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
		.serve(app.into_make_service())
		.await
		.unwrap();
}

#[derive(Deserialize)]
struct UserDataForm {
	username: String,
	password: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
struct NewUser<'a> {
	uname: &'a str,
	passwd: &'a str,
}

fn hash_password(password: &str) -> Result<String, Rejection> {
	let normalized: String = password.nfd().collect();
	let salt = SaltString::generate(&mut OsRng);
	let argon2 = Argon2::default();
	argon2
		.hash_password(normalized.as_bytes(), &salt)
		.map_err(internal_error)
		.map(|x| x.to_string())
}

async fn register(
	DatabaseConnection(mut conn): DatabaseConnection,
	Json(register_form): Json<UserDataForm>,
) -> Result<(), Rejection> {
	let password_hash = hash_password(&register_form.password)?;

	let new_user = NewUser {
		uname: &register_form.username,
		passwd: &password_hash,
	};

	if diesel::insert_into(schema::users::table)
		.values(new_user)
		.returning(models::User::as_returning())
		.get_result(&mut conn)
		.await
		.is_err()
	{
		Err(create_blank_error(StatusCode::CONFLICT))
	} else {
		Ok(())
	}
}

async fn login(
	DatabaseConnection(mut conn): DatabaseConnection,
	Json(login_form): Json<UserDataForm>,
) -> Result<Json<String>, Rejection> {
	let user: User = match schema::users::dsl::users
		.filter(schema::users::dsl::uname.eq(login_form.username))
		.first(&mut conn)
		.await
	{
		Ok(u) => u,
		Err(_) => return Err(create_blank_error(StatusCode::UNAUTHORIZED)),
	};
	let Ok(hash) = PasswordHash::new(&user.passwd) else {return Err(create_blank_error(StatusCode::BAD_REQUEST));};
	if Argon2::default()
		.verify_password(login_form.password.as_bytes(), &hash)
		.is_err()
	{
		return Err(create_blank_error(StatusCode::UNAUTHORIZED));
	}
	let token = generate_login_jwt(user.id)?;
	Ok(Json(token))
}
