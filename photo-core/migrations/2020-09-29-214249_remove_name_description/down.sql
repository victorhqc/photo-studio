CREATE TABLE photos_bkp (
  id TEXT PRIMARY KEY NOT NULL,
  album_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  index_in_album INTEGER NOT NULL DEFAULT 0,
  s3_id TEXT NOT NULL,
  src TEXT NOT NULL,
  main_color TEXT NOT NULL,
  title TEXT NOT NULL,
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

INSERT INTO photos_bkp
  SELECT id, album_id, user_id, index_in_album, s3_id, src, main_color, "" as title, null as description, created_at, updated_at, deleted
  FROM photos;

DROP TABLE photos;

ALTER TABLE photos_bkp RENAME TO photos;
