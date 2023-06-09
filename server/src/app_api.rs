use axum::{extract::Path, http::StatusCode, Json};
use diesel::{
	result::Error::NotFound, BelongingToDsl, BoolExpressionMethods, ExpressionMethods, QueryDsl,
	SelectableHelper,
};
use diesel_async::RunQueryDsl;
use serde::Serialize;

use crate::{
	auth::LoggedIn,
	models::{Flashcard, Question, Quiz, User},
	schema,
	util::{create_blank_error, internal_error, DatabaseConnection},
	Rejection,
};

#[derive(Serialize)]
struct UserInfo {
	username: String,
}

pub async fn user_info(LoggedIn(user): LoggedIn) -> Result<Json<impl Serialize>, Rejection> {
	Ok(Json(UserInfo {
		username: user.uname,
	}))
}

#[derive(Serialize)]
struct ApiQuiz {
	id: i32,
	name: String,
	username: String,
}

#[derive(Serialize)]
struct ApiQuizes {
	quizes: Vec<ApiQuiz>,
}

pub async fn quizes(
	user: Option<LoggedIn>,
	DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<impl Serialize>, Rejection> {
	use schema::quizes::dsl::*;
	let quizes_r: Vec<(Quiz, User)> = if let Some(LoggedIn(user)) = user {
		quizes
			.inner_join(schema::users::table)
			.filter(public.eq(true).or(user_id.eq(user.id)))
			.load(&mut conn)
			.await
			.map_err(internal_error)?
	} else {
		quizes
			.inner_join(schema::users::table)
			.filter(public.eq(true))
			.load(&mut conn)
			.await
			.map_err(internal_error)?
	};
	let api_quizes = quizes_r
		.into_iter()
		.map(|(q, u)| ApiQuiz {
			id: q.id,
			name: q.quiz_name,
			username: u.uname,
		})
		.collect();
	Ok(Json(ApiQuizes { quizes: api_quizes }))
}

pub async fn quiz(
	logged: Option<LoggedIn>,
	Path(id): Path<i32>,
	DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<impl Serialize>, Rejection> {
	use schema::quizes::dsl::{self, quizes};
	let (quiz, user): (Quiz, User) = match quizes
		.inner_join(schema::users::table)
		.filter(dsl::id.eq(id))
		.first(&mut conn)
		.await
	{
		Ok(s) => s,
		Err(NotFound) => return Err(create_blank_error(StatusCode::NOT_FOUND)),
		Err(e) => return Err(internal_error(e)),
	};
	if !quiz.public && logged.map(|x| x.0.id) != Some(quiz.user_id) {
		return Err(create_blank_error(StatusCode::UNAUTHORIZED));
	}
	Ok(Json(ApiQuiz {
		id: quiz.id,
		name: quiz.quiz_name,
		username: user.uname,
	}))
}

#[derive(Serialize)]
struct ApiQuestion {
	question: String,
	correct: String,
	wrong: Vec<String>,
}

#[derive(Serialize)]
struct ApiQuestions {
	name: String,
	username: String,
	questions: Vec<ApiQuestion>,
}

pub async fn quiz_questions(
	logged: Option<LoggedIn>,
	Path(id): Path<i32>,
	DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<impl Serialize>, Rejection> {
	use schema::quizes::dsl::{self, quizes};

	let (quiz, user): (Quiz, User) = match quizes
		.inner_join(schema::users::table)
		.filter(dsl::id.eq(id))
		.first(&mut conn)
		.await
	{
		Ok(s) => s,
		Err(NotFound) => return Err(create_blank_error(StatusCode::NOT_FOUND)),
		Err(e) => return Err(internal_error(e)),
	};
	if !quiz.public && logged.map(|x| x.0.id) != Some(quiz.user_id) {
		return Err(create_blank_error(StatusCode::UNAUTHORIZED));
	}
	let questions_r: Vec<Question> = Question::belonging_to(&quiz)
		.select(Question::as_select())
		.load(&mut conn)
		.await
		.map_err(internal_error)?;
	let questions_r: Vec<ApiQuestion> = questions_r
		.into_iter()
		.map(|x| {
			let mut wrong = vec![x.red_hering];
			if let Some(green_hering) = x.green_hering {
				wrong.push(green_hering)
			};
			if let Some(blue_hering) = x.blue_hering {
				wrong.push(blue_hering)
			};
			ApiQuestion {
				question: x.question,
				correct: x.correct,
				wrong,
			}
		})
		.collect();
	Ok(Json(ApiQuestions {
		name: quiz.quiz_name,
		username: user.uname,
		questions: questions_r,
	}))
}

#[derive(Serialize)]
struct ApiFlashcard {
	front: String,
	back: String,
}

#[derive(Serialize)]
struct ApiFlashcards {
	name: String,
	username: String,
	flashcards: Vec<ApiFlashcard>,
}

pub async fn quiz_flashcards(
	logged: Option<LoggedIn>,
	Path(id): Path<i32>,
	DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<impl Serialize>, Rejection> {
	use schema::quizes::dsl::{self, quizes};

	let (quiz, user): (Quiz, User) = match quizes
		.inner_join(schema::users::table)
		.filter(dsl::id.eq(id))
		.first(&mut conn)
		.await
	{
		Ok(s) => s,
		Err(NotFound) => return Err(create_blank_error(StatusCode::NOT_FOUND)),
		Err(e) => return Err(internal_error(e)),
	};
	if !quiz.public && logged.map(|x| x.0.id) != Some(quiz.user_id) {
		return Err(create_blank_error(StatusCode::UNAUTHORIZED));
	}
	let questions_r: Vec<Flashcard> = Flashcard::belonging_to(&quiz)
		.select(Flashcard::as_select())
		.load(&mut conn)
		.await
		.map_err(internal_error)?;
	let flashcards_r: Vec<ApiFlashcard> = questions_r
		.into_iter()
		.map(|x| ApiFlashcard {
			front: x.front,
			back: x.back,
		})
		.collect();
	Ok(Json(ApiFlashcards {
		name: quiz.quiz_name,
		username: user.uname,
		flashcards: flashcards_r,
	}))
}
