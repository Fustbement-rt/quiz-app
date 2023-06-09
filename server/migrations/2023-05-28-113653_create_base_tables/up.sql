CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	uname VARCHAR NOT NULL UNIQUE,
	passwd VARCHAR NOT NULL
);

CREATE TABLE quizes (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL REFERENCES users,
	public BOOLEAN NOT NULL
);

CREATE TABLE questions (
	id SERIAL PRIMARY KEY,
	quiz_id INTEGER NOT NULL REFERENCES quizes,
	correct VARCHAR NOT NULL,
	red_hering VARCHAR NOT NULL,
	blue_hering VARCHAR,
	green_hering VARCHAR
);

CREATE TABLE flashchards (
	id SERIAL PRIMARY KEY,
	quiz_id INTEGER NOT NULL REFERENCES quizes,
	front VARCHAR NOT NULL,
	back VARCHAR NOT NULL
);

CREATE TABLE attempts (
	id SERIAL NOT NULL PRIMARY KEY,
	user_id INTEGER NOT NULL REFERENCES users,
	quiz_id INTEGER NOT NULL REFERENCES quizes
);

CREATE TABLE answers (
  id SERIAL NOT NULL PRIMARY KEY,
  atempt_id INTEGER NOT NULL REFERENCES users,
  question_id INTEGER NOT NULL REFERENCES questions,
  created TIMESTAMP NOT NULL,
  correct BOOLEAN NOT NULL
);
