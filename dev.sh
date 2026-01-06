#!/bin/bash
# Development script that copies portfolio assets and starts dx serve

# Copy portfolio assets to the expected location after dx builds
copy_assets() {
    sleep 3  # Wait for initial build
    while true; do
        if [ -d "target/dx/pounds-consulting/debug/web/public/assets" ]; then
            mkdir -p target/dx/pounds-consulting/debug/web/public/assets/portfolio
            cp -r assets/portfolio/* target/dx/pounds-consulting/debug/web/public/assets/portfolio/ 2>/dev/null
        fi
        sleep 2
    done
}

# Start the asset copier in the background
copy_assets &
COPY_PID=$!

# Cleanup on exit
cleanup() {
    kill $COPY_PID 2>/dev/null
    exit 0
}
trap cleanup EXIT INT TERM

# Run dx serve
dx serve
