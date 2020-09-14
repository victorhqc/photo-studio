# photo-core

Photo Core

## Requirements

- Rust >= 1.46.0

## Development

Duplicate the `.env.example` and rename it as `.env`

Run migrations

```bash
# When bootstraping the project (so schema file doesn't get overwritten)
diesel migration run --locked-schema

# Whenever a new migration gets added.
diesel migration run

```

For windows

```powershell
$env:DATABASE_URL=".\photos.db"
diesel migration run --locked-schema
```
