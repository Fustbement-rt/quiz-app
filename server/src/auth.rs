use axum::{
	async_trait,
	extract::{FromRef, FromRequestParts},
	http::{header::AUTHORIZATION, request::Parts, StatusCode},
};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel_async::RunQueryDsl;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{
	models::User,
	schema,
	util::{create_blank_error, create_error, internal_error},
	Pool, Rejection,
};

#[derive(Serialize, Deserialize)]
pub struct Claims {
	uid: i32,
}

type Keys = (EncodingKey, DecodingKey);

static KEYS: Lazy<Keys> = Lazy::new(|| {
	let jwt_secret = std::env::var("JWT_SECRET").unwrap();
	(
		EncodingKey::from_secret(jwt_secret.as_bytes()),
		DecodingKey::from_secret(jwt_secret.as_bytes()),
	)
});

pub struct LoggedIn(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for LoggedIn
where
	S: Send + Sync,
	Pool: FromRef<S>,
{
	type Rejection = Rejection;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let mut conn = &mut Pool::from_ref(state)
			.get_owned()
			.await
			.map_err(internal_error)?;
		let Some(token) = parts.headers.get(AUTHORIZATION) else {return Err(create_blank_error(StatusCode::UNAUTHORIZED));};
		let Ok(token) = token.to_str() else {return Err(create_error(StatusCode::BAD_REQUEST, "invalid token"));};
		if !token.starts_with("Bearer ") {
			return Err(create_error(StatusCode::BAD_REQUEST, "invalid token"));
		}
		let token = &token[7..];
		let mut val = Validation::default();
		val.set_required_spec_claims::<&str>(&[]);
		let claims: Claims = match decode(token, &KEYS.1, &val) {
			Err(_) => return Err(create_error(StatusCode::BAD_REQUEST, "invalid token")),
			Ok(c) => c.claims,
		};
		let user: User = match schema::users::dsl::users
			.filter(schema::users::dsl::id.eq(claims.uid))
			.first(&mut conn)
			.await
		{
			Ok(u) => u,
			Err(_) => return Err(create_error(StatusCode::BAD_REQUEST, "invalid token")),
		};

		Ok(Self(user))
	}
}

pub fn generate_login_jwt(uid: i32) -> Result<String, Rejection> {
	encode(&Header::default(), &Claims { uid }, &KEYS.0).map_err(internal_error)
}
