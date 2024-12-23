docker run --rm \
    -v "$(pwd)":/app \
    -w /app \
    rust:latest \
    cargo test
