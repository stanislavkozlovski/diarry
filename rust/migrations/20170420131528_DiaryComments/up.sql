-- Your SQL goes here
CREATE TABLE diary_comments (
  id SERIAL PRIMARY KEY,
  entry_id integer REFERENCES diary_entries (id) NOT NULL,
  body TEXT NOT NULL,
  creation_date DATE NOT NULL DEFAULT CURRENT_DATE,
  creation_time TIME NOT NULL DEFAULT CURRENT_TIME
);
