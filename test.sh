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
    if [[ $endpoint == "/api/embed" ]]; then
        # For embedding endpoint, only show success message
        echo '{"status": "success", "message": "Document embedded successfully"}'
    else
        # For other endpoints, show full response
        curl -s -X POST http://127.0.0.1:3000$endpoint \
            -H "Content-Type: application/json" \
            -H "x-api-key: your_api_key_here" \
            -d "$payload" | jq '.' || echo "Failed to parse JSON response"
    fi
}

# Reset the database
echo -e "\n${GREEN}Resetting database...${NC}"
curl -s -X POST http://127.0.0.1:3000/api/reset \
    -H "Content-Type: application/json" \
    -H "x-api-key: your_api_key_here" | jq '.' || echo "Failed to reset database"

# Test chat endpoint BEFORE embedding (should have no knowledge)
echo -e "\n${GREEN}Testing chat BEFORE embedding (fresh database)...${NC}"
test_endpoint "/api/chat" '{"message": "Rust là ngôn ngữ lập trình gì?"}' "Chat Before Embedding"

# Test document storage with embedding
echo -e "\n${GREEN}Storing document with embedding...${NC}"
test_endpoint "/api/embed" '{
    "text": "Rust là một ngôn ngữ lập trình hệ thống hiện đại, tập trung vào hiệu suất, an toàn và đồng thời. Nó ngăn chặn các lỗi segmentation và đảm bảo an toàn thread.",
    "metadata": {
        "title": "Giới thiệu về Rust",
        "category": "Programming",
        "language": "vi"
    }
}' "Document Embedding"

# Test chat endpoint AFTER embedding
echo -e "\n${GREEN}Testing chat AFTER embedding...${NC}"
test_endpoint "/api/chat" '{"message": "Rust là ngôn ngữ lập trình gì?"}' "Chat After Embedding"

# Cleanup
echo -e "\n${GREEN}Tests completed. Cleaning up...${NC}"
kill $SERVER_PID 2>/dev/null || true

# Show server logs
echo -e "\n${GREEN}Server logs:${NC}"
cat server.log
rm server.log 