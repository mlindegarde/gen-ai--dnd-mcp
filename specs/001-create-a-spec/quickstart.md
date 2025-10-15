# Quickstart: MCP Server with PDF Form Filling Tool

**Purpose**: Get the D&D 5e character sheet PDF filling tool running locally with complete spell and narrative support

## Prerequisites

- Rust 1.75+ installed
- D&D 5e character sheet PDF (`docs/5E_CharacterSheet_Fillable.pdf`)
- Basic understanding of D&D 5e character creation

## Quick Setup

### 1. Install Dependencies

```bash
# Clone and navigate to project
cd mcp-dnd-server

# Install Rust dependencies
cargo build
```

### 2. Prepare Character Data

Create a JSON file with your complete character data:

```json
{
  "character": {
    "name": "Thorin Ironforge",
    "class": "Fighter",
    "level": 3,
    "race": "Dwarf",
    "background": "Soldier",
    "alignment": "Lawful Good",
    "experience_points": 900
  },
  "abilities": {
    "strength": 16,
    "dexterity": 12,
    "constitution": 15,
    "intelligence": 10,
    "wisdom": 13,
    "charisma": 8
  },
  "skills": {
    "athletics": 5,
    "athletics_proficient": true,
    "intimidation": 1,
    "intimidation_proficient": true,
    "perception": 1,
    "perception_proficient": false
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
      },
      {
        "name": "Shield",
        "level": 1,
        "prepared": false
      }
    ]
  },
  "spellcasting": {
    "spellcasting_ability": "Intelligence",
    "spell_attack_bonus": 4,
    "spell_save_dc": 12
  },
  "proficiencies": {
    "armor": ["Light armor", "Medium armor", "Heavy armor", "Shields"],
    "weapons": ["Simple weapons", "Martial weapons"],
    "tools": ["Smith's tools"],
    "languages": ["Common", "Dwarvish"]
  },
  "combat": {
    "armor_class": 18,
    "hit_point_maximum": 28,
    "current_hit_points": 28,
    "speed": 25,
    "initiative": 1
  },
  "equipment": {
    "weapons": [
      {
        "name": "Battleaxe",
        "attack_bonus": 5,
        "damage": "1d8+3 slashing"
      }
    ],
    "currency": {
      "gold": 150,
      "silver": 23,
      "copper": 45
    }
  },
  "narrative": {
    "personality_traits": "I face problems head-on. A simple, direct solution is the best path to success.",
    "ideals": "The thing that keeps a ship together is mutual respect between captain and crew.",
    "bonds": "I fight for those who cannot fight for themselves.",
    "flaws": "I have trouble trusting in my allies.",
    "character_appearance": "A stout dwarf with braided beard and battle scars.",
    "additional_features_traits": "Second Wind: Regain 1d10+3 hit points as a bonus action.",
    "treasure": "Family heirloom battleaxe passed down through generations.",
    "character_backstory": "Born in the mountain halls, trained as a warrior to defend the clan."
  }
}
```

### 3. Start MCP Server

```bash
# Start the MCP server
cargo run --bin mcp-server

# Server will listen on stdio for MCP protocol messages
```

### 4. Test with MCP Client

Using any MCP-compatible client:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "fill_dnd_character_sheet",
    "arguments": {
      "character_data": {
        // ... your complete character data here
      },
      "output_path": "thorin_character_sheet.pdf"
    }
  }
}
```

## Expected Output

- ✅ Filled PDF saved to specified output path
- ✅ Auto-calculated ability modifiers, saves, and skills
- ✅ Spells organized by level (cantrips, 1st-9th level sections)
- ✅ Proficiency circles marked for skills and saving throws
- ✅ Spell prepared states indicated with filled circles
- ✅ Character narrative fields populated (personality, backstory, etc.)
- ✅ D&D 5e rule validation with clear error messages
- ✅ Field mapping from friendly names to PDF field names

## Common Issues

### PDF Template Not Found
```
Error: PDF template not found at docs/5E_CharacterSheet_Fillable.pdf
Solution: Ensure the D&D 5e character sheet PDF is in the correct location
```

### Rule Validation Errors
```
Error: Strength score 25 exceeds maximum of 20
Solution: Use valid ability scores (1-20) or set allow_rule_violations: true
```

### Missing Required Fields
```
Error: Character name is required
Solution: Ensure character.name is provided in the input data
```

### Spell Level Mismatch
```
Error: Spell "Fireball" has level 3 but was placed in first_level array
Solution: Ensure spells are in the correct level array or let the system auto-organize
```

### Invalid Spellcasting Ability
```
Error: Spellcasting ability "Strength" is not valid
Solution: Use "Wisdom", "Intelligence", or "Charisma" for spellcasting_ability
```

## Feature Highlights

### Spell System
- **Cantrips**: Level 0 spells, always prepared
- **Leveled Spells**: 1st through 9th level organization
- **Prepared State**: Circles filled for prepared spells
- **Auto-calculation**: Spell attack bonus and save DC from spellcasting ability

### Proficiency Tracking
- **Skills**: Proficiency circles marked automatically
- **Saving Throws**: Proficiency indicators based on class
- **Equipment**: Weapon, armor, tool, and language proficiencies listed

### Character Narrative
- **Personality**: Traits, ideals, bonds, and flaws
- **Appearance**: Physical description and character details
- **Backstory**: Character history and background
- **Features**: Class features, racial traits, and special abilities

### D&D 5e Validation
- **Ability Scores**: 1-20 range validation
- **Proficiency Bonus**: Level-appropriate bonuses
- **Derived Values**: Auto-calculated modifiers and bonuses
- **Rule Override**: Option to proceed with invalid data

## Next Steps

1. **Customize Field Mapping**: Modify field mappings in `src/pdf_mapper.rs`
2. **Add Custom Rules**: Extend validation in `src/dnd_validator.rs`
3. **Spell Database**: Add spell validation against official D&D spell lists
4. **Test Integration**: Use with your preferred MCP client or AI assistant

## File Structure

```
src/
├── main.rs              # MCP server entry point
├── mcp_server.rs        # MCP protocol implementation
├── pdf_filler.rs        # PDF form filling logic
├── dnd_validator.rs     # D&D 5e rule validation
├── field_mapper.rs      # JSON to PDF field mapping
├── character_model.rs   # Character data structures
├── spell_system.rs      # Spell organization and validation
└── narrative_handler.rs # Character narrative processing

docs/
└── 5E_CharacterSheet_Fillable.pdf  # D&D character sheet template

tests/
├── integration/         # MCP protocol tests
├── unit/               # Component unit tests
└── fixtures/           # Sample character data and PDFs
```

## Development Mode

For development and testing:

```bash
# Run with debug logging
RUST_LOG=debug cargo run --bin mcp-server

# Run tests
cargo test

# Test spell system specifically
cargo test spell_system

# Test narrative fields
cargo test narrative

# Check code quality
cargo clippy
cargo fmt
```
