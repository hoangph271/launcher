CREATE TABLE users (
  id VARCHAR(21) NOT NULL PRIMARY KEY,
  email VARCHAR(255) NOT NULL UNIQUE,
  nickname TEXT NOT NULL
)
