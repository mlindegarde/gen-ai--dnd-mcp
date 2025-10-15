#!/usr/bin/env python3

import json
import subprocess
import base64
import sys

def test_pdf_return():
    # Sample character data
    character_data = {
        "character": {
            "name": "Test Character",
            "class": "Fighter",
            "level": 5,
            "race": "Human"
        },
        "abilities": {
            "strength": 16,
            "dexterity": 14,
            "constitution": 15,
            "intelligence": 12,
            "wisdom": 13,
            "charisma": 10
        }
    }
    
    # MCP request to get PDF content back
    request = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "fill_dnd_character_sheet",
            "arguments": {
                "character_data": character_data,
                "return_pdf_content": True,
                "output_path": "temp_character_sheet.pdf"
            }
        }
    }
    
    # Start MCP server
    process = subprocess.Popen(
        ['./target/release/dnd-character-sheet-filler'],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    # Send initialize request first
    init_request = {
        "jsonrpc": "2.0",
        "id": 0,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "test-client", "version": "1.0.0"}
        }
    }
    
    process.stdin.write(json.dumps(init_request) + '\n')
    process.stdin.flush()
    
    # Read initialize response
    init_response = process.stdout.readline()
    print("Initialize response:", init_response.strip())
    
    # Send the actual request
    process.stdin.write(json.dumps(request) + '\n')
    process.stdin.flush()
    
    # Read response
    response_line = process.stdout.readline()
    process.stdin.close()
    
    try:
        response = json.loads(response_line)
        print("Response received successfully")
        
        if "result" in response and "pdf_content" in response["result"]:
            pdf_data = response["result"]["pdf_content"]
            if isinstance(pdf_data, dict) and "data" in pdf_data:
                pdf_base64 = pdf_data["data"]
                pdf_size = pdf_data.get("size", 0)
            else:
                # Fallback for old format
                pdf_base64 = pdf_data
                pdf_size = 0
                
            pdf_bytes = base64.b64decode(pdf_base64)
            
            # Save the PDF to a new location
            output_path = "returned_character_sheet.pdf"
            with open(output_path, 'wb') as f:
                f.write(pdf_bytes)
            
            print(f"✅ PDF successfully returned and saved to: {output_path}")
            print(f"📄 PDF size: {len(pdf_bytes)} bytes (reported: {pdf_size})")
            return True
        else:
            print("❌ No PDF content in response")
            print("Response:", json.dumps(response, indent=2))
            return False
            
    except json.JSONDecodeError as e:
        print(f"❌ Failed to parse response: {e}")
        print("Raw response:", response_line)
        return False
    except Exception as e:
        print(f"❌ Error: {e}")
        return False
    finally:
        process.terminate()

if __name__ == "__main__":
    success = test_pdf_return()
    sys.exit(0 if success else 1)
