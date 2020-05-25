-- Your SQL goes here
CREATE TABLE scores (
    id SERIAL PRIMARY KEY,
    userid INTEGER NOT NULL REFERENCES users(id),
    bookid VARCHAR NOT NULL REFERENCES books(id),
    score FLOAT NOT NULL
)