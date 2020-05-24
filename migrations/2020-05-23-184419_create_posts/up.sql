-- Your SQL goes here

CREATE TABLE rating (
	id SERIAL PRIMARY KEY,
	username VARCHAR NOT NULL,
	title VARCHAR NOT NULL,
	score FLOAT DEFAULT NULL 
)