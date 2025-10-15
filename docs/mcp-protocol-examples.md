# MCP Protocol Implementation Examples

**Purpose**: Concrete MCP JSON-RPC message examples for implementation reference
**Date**: 2025-10-14

## MCP Server Initialization

### Server Capabilities Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "capabilities": {
      "tools": {}
    },
    "serverInfo": {
      "name": "dnd-character-sheet-filler",
      "version": "1.0.0"
    }
  }
}
```

## Tool Discovery

### List Tools Request
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list"
}
```

### List Tools Response
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "tools": [
      {
        "name": "fill_dnd_character_sheet",
        "description": "Fills a D&D 5e character sheet PDF with provided character data including spells, proficiencies, and narrative elements",
        "inputSchema": {
          "type": "object",
          "properties": {
            "character_data": {
              "type": "object",
              "description": "Complete D&D 5e character information"
            },
            "output_path": {
              "type": "string",
              "description": "Path where filled PDF should be saved"
            },
            "allow_rule_violations": {
              "type": "boolean",
              "default": false
            }
          },
          "required": ["character_data"]
        }
      }
    ]
  }
}
```

## Tool Execution

### Fill Character Sheet Request
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "fill_dnd_character_sheet",
    "arguments": {
      "character_data": {
        "character": {
          "name": "Thorin Ironforge",
          "class": "Fighter",
          "level": 3,
          "race": "Dwarf"
        },
        "abilities": {
          "strength": 16,
          "dexterity": 12,
          "constitution": 15,
          "intelligence": 10,
          "wisdom": 13,
          "charisma": 8
        },
        "spells": {
          "cantrips": [
            {
              "name": "Mage Hand",
              "level": 0,
              "prepared": true
            }
          ],
          "first_level": [
            {
              "name": "Magic Missile", 
              "level": 1,
              "prepared": true
            }
          ]
        }
      },
      "output_path": "thorin_character.pdf"
    }
  }
}
```

### Successful Response
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "success": true,
    "output_file": "thorin_character.pdf",
    "calculated_fields": {
      "strength_modifier": 3,
      "dexterity_modifier": 1,
      "constitution_modifier": 2,
      "spell_attack_bonus": 4,
      "spell_save_dc": 12
    },
    "spells_organized": {
      "cantrips_placed": 1,
      "first_level_placed": 1
    }
  }
}
```

### Error Response (Validation Failure)
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "error": {
    "code": -32602,
    "message": "Invalid character data",
    "data": {
      "validation_errors": [
        {
          "field": "abilities.strength",
          "error": "Strength score 25 exceeds maximum of 20",
          "severity": "error"
        },
        {
          "field": "spells.first_level[0].level",
          "error": "Spell 'Fireball' has level 3 but was placed in first_level array",
          "severity": "error"
        }
      ]
    }
  }
}
```

### Error Response (Rule Override)
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "result": {
    "success": true,
    "output_file": "custom_character.pdf",
    "validation_errors": [
      {
        "field": "abilities.strength",
        "error": "Strength score 25 exceeds maximum of 20",
        "severity": "warning"
      }
    ],
    "rule_violations_overridden": true
  }
}
```

## Protocol Flow

### Connection Sequence
1. Client connects to server via stdio
2. Client sends initialization request
3. Server responds with capabilities
4. Client can now discover and call tools

### Tool Call Sequence  
1. Client lists available tools
2. Server returns tool definitions with schemas
3. Client calls tool with validated parameters
4. Server processes and returns results or errors

### Error Handling
- **-32700**: Parse error (invalid JSON)
- **-32600**: Invalid request (malformed JSON-RPC)
- **-32601**: Method not found
- **-32602**: Invalid params (validation failure)
- **-32603**: Internal error (processing failure)
