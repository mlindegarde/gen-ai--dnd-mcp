#!/bin/bash
echo "Starting D&D Character Sheet Filler MCP Server..."
echo "Server will listen on stdin/stdout for JSON-RPC messages"
echo "Press Ctrl+C to stop"
echo ""
cd "$(dirname "$0")"
cargo run
