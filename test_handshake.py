#!/usr/bin/env python3
import json
import subprocess
import time

def test_full_handshake():
    process = subprocess.Popen(
        ["./target/release/dnd-character-sheet-filler"],
        cwd="/Users/lindegar/learningplace/specify--test",
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    try:
        # 1. Initialize
        init_msg = {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}
        process.stdin.write(json.dumps(init_msg) + "\n")
        process.stdin.flush()
        
        response = process.stdout.readline()
        print("1. Initialize response:", response.strip())
        
        # 2. Initialized notification
        initialized_msg = {"jsonrpc": "2.0", "method": "initialized", "params": {}}
        process.stdin.write(json.dumps(initialized_msg) + "\n")
        process.stdin.flush()
        
        # 3. List tools
        tools_msg = {"jsonrpc": "2.0", "id": 2, "method": "tools/list"}
        process.stdin.write(json.dumps(tools_msg) + "\n")
        process.stdin.flush()
        
        response = process.stdout.readline()
        print("2. Tools list response:", response.strip())
        
        # Check server is alive
        time.sleep(0.5)
        if process.poll() is None:
            print("✅ Server completed handshake and is running")
        else:
            print("❌ Server died during handshake")
            
    finally:
        process.terminate()
        process.wait()

if __name__ == "__main__":
    test_full_handshake()
