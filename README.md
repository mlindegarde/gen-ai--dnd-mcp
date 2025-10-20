# D&D 5e Character Sheet Filler - MCP Server

A Model Context Protocol (MCP) server that fills D&D 5e character sheet PDFs with character data including spells, proficiencies, and narrative elements.

## Features

✅ **Complete D&D 5e Support**: Handles all character data including abilities, spells (cantrips through 9th level), proficiencies, combat stats, and narrative elements

✅ **Spell Slot Tracking**: Automatically calculates and displays maximum spell slots for single-class characters based on D&D 5e progression tables

✅ **Rule Validation**: Validates character data against D&D 5e rules with optional override capability

✅ **MCP Protocol**: Full JSON-RPC 2.0 MCP server implementation for AI assistant integration

✅ **Auto-Calculation**: Automatically calculates ability modifiers, proficiency bonuses, spell attack bonuses, and spell save DCs

✅ **Spell Organization**: Properly organizes spells by level with prepared state tracking

✅ **Windows Compatible**: Checkboxes visible, appropriate font sizing, and cross-platform debug logging

## Quick Start

### Test Mode
```bash
cargo run test
```

### MCP Server Mode
```bash
cargo run
```

## Architecture

- **character_model.rs**: Core D&D 5e data structures
- **dnd_validator.rs**: D&D 5e rule validation with exact formulas
- **field_mapper.rs**: Maps JSON character data to PDF field names
- **pdf_filler.rs**: PDF form filling logic with lopdf
- **spell_system.rs**: Spell organization and spellcasting calculations
- **narrative_handler.rs**: Character personality and backstory processing
- **mcp_server.rs**: JSON-RPC 2.0 MCP protocol implementation
- **errors.rs**: Comprehensive error types and messages

## Sample Character Data

See `tests/fixtures/sample-character.json` for a complete example including:
- Basic character info (name, class, level, race)
- Ability scores (strength through charisma)
- Spells organized by level with prepared states
- Combat statistics and proficiencies
- Character narrative (personality, ideals, bonds, flaws)
- Equipment and currency

## MCP Tool: `fill_dnd_character_sheet`

**Parameters:**
- `character_data` (required): Complete D&D 5e character information
- `output_path` (optional): Path for filled PDF (default: "filled_character_sheet.pdf")
- `allow_rule_violations` (optional): Allow rule violations (default: false)

**Returns:**
- Success status and output file path
- Calculated fields (modifiers, bonuses, DCs)
- Validation errors/warnings
- Rule violation override status

## Implementation Status

### ✅ Phase 1: Setup - COMPLETE
- Rust project structure with Cargo.toml
- Dependencies: lopdf, pdf-writer, serde_json, tokio
- Test directory structure

### ✅ Phase 2: Foundational - COMPLETE  
- PDF field analysis with exact field names
- D&D 5e rule specifications with formulas
- MCP protocol examples with JSON-RPC messages
- Core data structures and validation
- Field mapping and spell system

### ✅ Phase 3: User Story 1 (Basic PDF Filling) - COMPLETE
- PDF parsing and form filling logic
- D&D rule validation integration
- Spell organization by level
- Character narrative processing
- Error handling and validation overrides

### ✅ Phase 4: User Story 2 (MCP Integration) - COMPLETE
- JSON-RPC 2.0 protocol implementation
- MCP tool schema and validation
- Server initialization and client handling
- Input parameter validation
- Proper error response formatting

### 🚧 Phase 5: User Story 3 (Field Discovery) - COMPLETE
- ✅ Systematic checkbox field mapping for saving throws (Check Box 11, 18-22)
- ✅ Systematic checkbox field mapping for skills (Check Box 23-40) 
- ✅ Systematic checkbox field mapping for spell preparation (levels 1-9)
- ✅ Enhanced field discovery functionality
- ✅ Detailed validation error reporting
- ✅ Field constraint analysis

## Constitutional Compliance

✅ **Local-First**: All PDF processing runs locally, no cloud dependencies  
✅ **Hobby-Focused**: Serves genuine D&D character sheet filling use case  
✅ **Rust Idiomatic**: Uses lopdf/pdf-writer with proper Rust patterns  
✅ **MCP Protocol Compliance**: Follows JSON-RPC 2.0 specification  
✅ **Simplicity**: Single binary with focused PDF filling responsibility

## Dependencies

- **lopdf**: PDF parsing and manipulation
- **pdf-writer**: PDF form field writing  
- **serde_json**: JSON serialization/deserialization
- **tokio**: Async runtime for MCP server
- **serde**: Data structure serialization

## File Size Limits

- Maximum PDF file size: 10MB
- Response time target: <2 seconds for PDFs under 5MB
