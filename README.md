# rust-server

A tiny, dependency-free HTTP server written in Rust. It serves static files from `public/` and includes a couple of example routes.

## Features

- Pure Rust, no external crates
- Serves static files from a configurable `PUBLIC_PATH`
- Minimal HTTP request parsing/response writing (for learning and experiments)

## Running

```bash
cargo run
```

Then open:

- http://127.0.0.1:8080/ (serves `public/index.html`)
- http://127.0.0.1:8080/about (simple HTML string)
- http://127.0.0.1:8080/style.css (static file from `public/`)

## Configuration

`PUBLIC_PATH` controls where static files are loaded from.

By default it uses: `<repo>/public`

Example:

```bash
PUBLIC_PATH=/absolute/path/to/your/public cargo run
```

## Notes

This project is intentionally minimal and currently focuses on `GET` requests and simple file serving.
