use axum::{
	async_trait,
	extract::{FromRef, FromRequestParts},
	http::{request::Parts, StatusCode},
	Json,
};
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

use crate::{Pool, Rejection};

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
pub struct DatabaseConnection(
	pub bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
	S: Send + Sync,
	Pool: FromRef<S>,
{
	type Rejection = Rejection;

	async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let pool = Pool::from_ref(state);

		let conn = pool.get_owned().await.map_err(internal_error)?;

		Ok(Self(conn))
	}
}

pub fn create_blank_error(status_code: StatusCode) -> Rejection {
	(
		status_code,
		Json(crate::JsonError {
			code: status_code.as_u16(),
			canon: status_code.canonical_reason().unwrap().to_string(),
			string: None,
		}),
	)
}

pub fn create_error(status_code: StatusCode, string: impl ToString) -> Rejection {
	let mut error = create_blank_error(status_code);
	error.1.string = Some(string.to_string());
	error
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> Rejection
where
	E: std::error::Error,
{
	create_error(StatusCode::INTERNAL_SERVER_ERROR, err)
}
