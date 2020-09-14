# photo-api

Photo API

## Requirements

- Rust >= 1.46.0
- Docker (For development)

## Development

Duplicate the `.env.example` and rename it as `.env`

_Note: Make sure you followed the instructions in the core first, as it shows how to setup DB_

Run in development mode.

```bash
cargo run

# Or with cargo-watch for updating after changes
cargo watch -x "run"
```

For watching changes install `cargo-watch`

```bash
cargo install cargo-watch
```
