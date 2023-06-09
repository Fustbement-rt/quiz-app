use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
	pub id: i32,
	pub uname: String,
	pub passwd: String,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::quizes)]
pub struct Quiz {
	pub id: i32,
	pub user_id: i32,
	pub public: bool,
	pub quiz_name: String,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Associations)]
#[diesel(belongs_to(Quiz, foreign_key = quiz_id))]
#[diesel(table_name = crate::schema::questions)]
pub struct Question {
	pub id: i32,
	pub quiz_id: i32,
	pub correct: String,
	pub red_hering: String,
	pub blue_hering: Option<String>,
	pub green_hering: Option<String>,
	pub question: String,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Associations)]
#[diesel(belongs_to(Quiz, foreign_key = quiz_id))]
#[diesel(table_name = crate::schema::flashchards)]
pub struct Flashcard {
	pub id: i32,
	pub quiz_id: i32,
	pub front: String,
	pub back: String,
}
