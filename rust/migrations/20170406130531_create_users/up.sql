-- Your SQL goes here
CREATE TABLE diary_owner (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  jwt_token VARCHAR
)