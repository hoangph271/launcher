CREATE TABLE auths (
  id TEXT NOT NULL PRIMARY KEY,
  auth_type TEXT NOT NULL,
  email TEXT NOT NULL,
  password_hash TEXT NOT NULL,
  FOREIGN KEY(email) REFERENCES users(email)
)
