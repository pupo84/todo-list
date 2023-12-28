# Run the development server
dev:
    cargo run

# Run tests
test:
    cargo test

# Format code using rustfmt
format:
    cargo fmt

# Check code formatting
check-format:
    cargo fmt -- --check

# Run clippy (Rust linter)
clippy:
    cargo clippy

# Build the project
build:
    cargo build --release

# Run migrations
migrate:
    diesel migration run --database-url $(DATABASE_URL)

# Rollback the last migration
rollback:
    diesel migration revert --database-url $(DATABASE_URL)

# Seed the database (if applicable)
seed:
    # Add your custom seed command here

# Clean up compiled files
clean:
    cargo clean

# Alias for commonly used tasks during development
start: dev