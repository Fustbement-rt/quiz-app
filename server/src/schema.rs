// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        atempt_id -> Int4,
        question_id -> Int4,
        created -> Timestamp,
        correct -> Bool,
    }
}

diesel::table! {
    attempts (id) {
        id -> Int4,
        user_id -> Int4,
        quiz_id -> Int4,
    }
}

diesel::table! {
    flashchards (id) {
        id -> Int4,
        quiz_id -> Int4,
        front -> Varchar,
        back -> Varchar,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        quiz_id -> Int4,
        correct -> Varchar,
        red_hering -> Varchar,
        blue_hering -> Nullable<Varchar>,
        green_hering -> Nullable<Varchar>,
        question -> Varchar,
    }
}

diesel::table! {
    quizes (id) {
        id -> Int4,
        user_id -> Int4,
        public -> Bool,
        quiz_name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        uname -> Varchar,
        passwd -> Varchar,
    }
}

diesel::joinable!(answers -> questions (question_id));
diesel::joinable!(answers -> users (atempt_id));
diesel::joinable!(attempts -> quizes (quiz_id));
diesel::joinable!(attempts -> users (user_id));
diesel::joinable!(flashchards -> quizes (quiz_id));
diesel::joinable!(questions -> quizes (quiz_id));
diesel::joinable!(quizes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    attempts,
    flashchards,
    questions,
    quizes,
    users,
);
