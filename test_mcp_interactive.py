#!/usr/bin/env python3
import json
import subprocess
import time

def test_mcp_interactive():
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
        print("✅ Initialization:", response.strip())
        
        # Wait a bit
        time.sleep(0.1)
        
        # Test tools list
        tools_request = {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list"
        }
        
        process.stdin.write(json.dumps(tools_request) + "\n")
        process.stdin.flush()
        
        response = process.stdout.readline()
        print("✅ Tools list:", response.strip())
        
        # Check if process is still alive
        if process.poll() is None:
            print("✅ Server is still running")
        else:
            print("❌ Server terminated unexpectedly")
            
        # Keep server alive for a bit
        time.sleep(1)
        
        if process.poll() is None:
            print("✅ Server stayed alive")
        else:
            print("❌ Server died")
        
    finally:
        process.terminate()
        process.wait()

if __name__ == "__main__":
    test_mcp_interactive()
