#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Kill any existing process on port 3000
echo "Cleaning up any existing process on port 3000..."
lsof -i :3000 | grep LISTEN | awk '{print $2}' | xargs kill -9 2>/dev/null || true

# Start the server in the background
echo "Starting the server..."
RUST_LOG=info cargo run > server.log 2>&1 &
SERVER_PID=$!

# Wait for server to start
echo "Waiting for server to start..."
sleep 5

# Function to test an endpoint
test_endpoint() {
    local endpoint=$1
    local payload=$2
    local description=$3

    echo -e "\n${GREEN}Testing $description...${NC}"
    echo "Request:"
    echo "curl -X POST http://127.0.0.1:3000$endpoint -H \"Content-Type: application/json\" -H \"x-api-key: your_api_key_here\" -d '$payload'"
    
    echo -e "\nResponse:"
    curl -s -X POST http://127.0.0.1:3000$endpoint \
        -H "Content-Type: application/json" \
        -H "x-api-key: your_api_key_here" \
        -d "$payload" | jq '.' || echo "Failed to parse JSON response"
}

# Test embedding endpoint
test_endpoint "/api/embed" '{"text": "Hello, this is a test message"}' "Embedding Endpoint"

# Test chat endpoint
test_endpoint "/api/chat" '{"message": "What is the capital of France?"}' "Chat Endpoint"

# Cleanup
echo -e "\n${GREEN}Tests completed. Cleaning up...${NC}"
kill $SERVER_PID 2>/dev/null || true

# Show server logs
echo -e "\n${GREEN}Server logs:${NC}"
cat server.log
rm server.log 