-- Your SQL goes here
CREATE TABLE scores (
    id SERIAL PRIMARY KEY,
    userid INTEGER NOT NULL REFERENCES users(id),
    movieid INTEGER NOT NULL REFERENCES movies(id),
    score FLOAT NOT NULL
)