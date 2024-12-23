docker run --rm \
    -v "$(pwd)":/app \
    -w /app \
    rust:latest \
    cargo run --release

# now test with the python script:

docker run --rm \
    -v "$(pwd)":/app \
    -w /app \
    python:latest \
    python test_set.py