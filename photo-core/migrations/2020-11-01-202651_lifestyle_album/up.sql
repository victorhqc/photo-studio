UPDATE albums
SET name = 'weddings', description = 'Wedding pictures'
WHERE name = 'web';

CREATE TABLE custom_migrations (
  id TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT current_timestamp NOT NULL
);
