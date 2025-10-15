#!/usr/bin/env python3
import json
import subprocess
import sys

def test_mcp_server():
    # Start the MCP server
    process = subprocess.Popen(
        ["cargo", "run"],
        cwd="/Users/lindegar/learningplace/specify--test",
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    try:
        # Test initialization
        init_request = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {}
        }
        
        process.stdin.write(json.dumps(init_request) + "\n")
        process.stdin.flush()
        
        # Read response
        response = process.stdout.readline()
        print("Initialization response:", response.strip())
        
        # Test tools list
        tools_request = {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list"
        }
        
        process.stdin.write(json.dumps(tools_request) + "\n")
        process.stdin.flush()
        
        response = process.stdout.readline()
        print("Tools list response:", response.strip())
        
        # Test tool call
        tool_call = {
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": "fill_dnd_character_sheet",
                "arguments": {
                    "character_data": {
                        "character": {
                            "name": "Test Character",
                            "class": "Fighter",
                            "level": 1,
                            "race": "Human"
                        },
                        "abilities": {
                            "strength": 15,
                            "dexterity": 14,
                            "constitution": 13,
                            "intelligence": 12,
                            "wisdom": 10,
                            "charisma": 8
                        }
                    },
                    "output_path": "mcp_test_output.pdf"
                }
            }
        }
        
        process.stdin.write(json.dumps(tool_call) + "\n")
        process.stdin.flush()
        
        response = process.stdout.readline()
        print("Tool call response:", response.strip())
        
    finally:
        process.terminate()
        process.wait()

if __name__ == "__main__":
    test_mcp_server()
