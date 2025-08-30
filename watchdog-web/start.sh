#!/bin/bash

# Build script for Watchdog Web UI

# Check if node is installed
if ! command -v node &> /dev/null
then
    echo "Node.js is not installed. Please install Node.js to run this application."
    exit 1
fi

# Install dependencies if node_modules doesn't exist
if [ ! -d "node_modules" ]; then
    echo "Installing dependencies..."
    npm install
fi

# Start the server
echo "Starting Watchdog Web UI..."
echo "Open your browser and navigate to http://localhost:3000"
npm start