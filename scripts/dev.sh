#!/bin/bash

# Start API in the background (in api directory)
cd api
docker compose up -d db
cargo run &
API_PID=$!

# Start frontend (in web directory)
cd ../web
pnpm dev

# When frontend is stopped, stop the API
kill $API_PID