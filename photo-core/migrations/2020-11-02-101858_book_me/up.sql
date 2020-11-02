CREATE TABLE book_me (
  id TEXT PRIMARY KEY NOT NULL,
  user_id TEXT NOT NULL,
  email TEXT NOT NULL,
  FOREIGN KEY (user_id)
    REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
