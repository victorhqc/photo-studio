CREATE TABLE users (
  id TEXT PRIMARY KEY NOT NULL,
  email TEXT UNIQUE NOT NULL,
  created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL
);

CREATE TABLE albums (
  id TEXT PRIMARY KEY NOT NULL,
  user_id TEXT NOT NULL,
  name TEXT NOT NULL,
  description TEXT NULL,
  created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  deleted BOOLEAN NOT NULL DEFAULT false,
  FOREIGN KEY (user_id)
    REFERENCES albums (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE photos (
  id TEXT PRIMARY KEY NOT NULL,
  album_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  index_in_album INTEGER NOT NULL DEFAULT 0,
  src TEXT NOT NULL,
  main_color TEXT NOT NULL,
  description TEXT,
  created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  deleted BOOLEAN NOT NULL DEFAULT false,
  FOREIGN KEY (album_id)
    REFERENCES albums (id)
      ON DELETE CASCADE
      ON UPDATE CASCADE,
  FOREIGN KEY (user_id)
    REFERENCES users (id)
      ON DELETE CASCADE
      ON UPDATE CASCADE
);
