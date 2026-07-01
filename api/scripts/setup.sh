#!/bin/bash
set -e

# Copy environment file if it doesn't exist
if [ ! -f .env ]; then
    cp .env.sample .env
    echo "Created .env file from template. Please review and update the values."
fi

# Ensure migrations directory exists
if [ ! -d migrations ]; then
    mkdir -p migrations
fi

# Move SQL files to migrations if they're in the root
for file in *.sql; do
    if [ -f "$file" ]; then
        mv "$file" migrations/
        echo "Moved $file to migrations/"
    fi
done

# Check if sqlx-cli is installed
if ! command -v sqlx &> /dev/null; then
    echo "Installing sqlx-cli..."
    cargo install sqlx-cli --no-default-features --features native-tls,postgres
fi

# Start PostgreSQL container
docker compose up -d db

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL to be ready..."
until docker compose exec db pg_isready -U postgres > /dev/null 2>&1; do
    sleep 1
done

echo "PostgreSQL is ready!"

# Source the .env file to get DATABASE_URL
source .env

# Run migrations
echo "Running database migrations..."
sqlx migrate run

echo "Setup complete! You can now run: cargo run"