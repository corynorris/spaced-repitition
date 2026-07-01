# Quickstart

## Clone the repository

git clone <your-repo>
cd <your-repo>

## Setup development environment

chmod +x ./scripts/setup.sh
./scripts/setup.sh

## Run the application

cargo run

# Deploying to Dokku

dokku postgres:create spaced_repetition_db
dokku postgres:link spaced_repetition_db your-app-name

dokku config:set your-app-name HMAC_KEY=your_production_key_here
