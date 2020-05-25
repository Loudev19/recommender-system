-- Your SQL goes here
CREATE TABLE scores (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  movie_id INTEGER NOT NULL REFERENCES movies(id),
  score FLOAT NOT NULL
)