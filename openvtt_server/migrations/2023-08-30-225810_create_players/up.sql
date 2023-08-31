CREATE TABLE players (
  id INTEGER PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  hashed_password TEXT NOT NULL
)
