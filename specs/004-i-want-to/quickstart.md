# Quickstart: Character Sheet PDF Population

**Feature**: Character Sheet PDF Population (Saves, Skills, Spells)  
**Generated**: 2025-10-16  
**Prerequisites**: Rust 1.75+, existing D&D character sheet PDF filler codebase

## Overview

This feature extends the existing D&D character sheet PDF filler to populate:
- Saving throw bonuses with proficiency indicators
- Skill bonuses with proficiency and expertise indicators  
- Spell preparation checkboxes for levels 1-9

All checkbox field mappings have been systematically verified through testing.

## Quick Start

### 1. Basic Usage (MCP Tool)

```json
{
  "character_data": {
    "basic_info": {
      "name": "Erilon",
      "level": 6,
      "class": "Wizard"
    },
    "ability_scores": {
      "strength": 10,
      "dexterity": 14,
      "constitution": 12,
      "intelligence": 18,
      "wisdom": 12,
      "charisma": 12
    },
    "proficiencies": {
      "saving_throws": ["intelligence", "wisdom"],
      "skills": ["arcana", "history", "investigation"],
      "expertise": ["arcana"]
    },
    "spells": {
      "prepared_spells": {
        "1": ["magic_missile", "shield", "detect_magic"],
        "2": ["misty_step", "web"],
        "3": ["fireball", "counterspell"]
      }
    }
  },
  "output_path": "erilon_character_sheet.pdf"
}
```

### 2. Command Line Usage

```bash
# Test mode with sample character
cargo run test

# MCP server mode
cargo run
```

### 3. Expected Output

The tool will:
1. Calculate saving throw bonuses: `modifier + proficiency_bonus (if proficient)`
2. Calculate skill bonuses: `modifier + proficiency_bonus * (2 if expertise, 1 if proficient, 0 otherwise)`
3. Mark proficiency checkboxes for saves and skills
4. Mark spell preparation checkboxes for prepared spells
5. Generate filled PDF with all calculations and indicators

## Key Calculations

### Proficiency Bonus by Level
- Levels 1-4: +2
- Levels 5-8: +3  
- Levels 9-12: +4
- Levels 13-16: +5
- Levels 17-20: +6

### Ability Modifiers
- Score 10-11: +0
- Score 12-13: +1
- Score 14-15: +2
- Score 16-17: +3
- Score 18-19: +4
- Score 20-21: +5

### Example Calculations

For a 6th level character (proficiency bonus +3):
- **Strength Save**: Strength 10 (+0), not proficient → +0
- **Intelligence Save**: Intelligence 18 (+4), proficient → +7 (+4 + 3)
- **Arcana Skill**: Intelligence 18 (+4), expertise → +10 (+4 + 6)
- **History Skill**: Intelligence 18 (+4), proficient → +7 (+4 + 3)

## Checkbox Field Mappings

### Saving Throws (Verified)
- Strength: Check Box 11
- Dexterity: Check Box 18
- Constitution: Check Box 19
- Intelligence: Check Box 20
- Wisdom: Check Box 21
- Charisma: Check Box 22

### Skills (Verified)
- Acrobatics: Check Box 23
- Animal Handling: Check Box 24
- Arcana: Check Box 25
- Athletics: Check Box 26
- [... continues through Check Box 40 for Survival]

### Spell Preparation (Verified)
- **Level 1**: Check Box 251, 309, 3010-3019 (12 slots)
- **Level 2**: Check Box 313, 310, 3020-3030 (13 slots)
- **Level 3**: Check Box 315, 314, 3031-3041 (13 slots)
- **Level 4**: Check Box 317, 316, 3042-3052 (13 slots)
- **Level 5**: Check Box 319, 318, 3053-3059 (9 slots)
- **Level 6**: Check Box 321, 320, 3060-3066 (9 slots)
- **Level 7**: Check Box 323, 322, 3067-3073 (9 slots)
- **Level 8**: Check Box 325, 324, 3074-3078 (7 slots)
- **Level 9**: Check Box 327, 326, 3079-3083 (7 slots)

## Error Handling

The system gracefully handles:
- Missing ability scores (skips affected calculations)
- Invalid proficiency selections (ignores invalid entries)
- Overflow spell preparation (marks available checkboxes only)
- PDF field mapping failures (continues with valid fields)

## Testing

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test saving_throws
cargo test skills
cargo test spell_preparation
cargo test checkbox_mapping
```

## Implementation Status

✅ **Saving Throw Calculations**: Complete with proficiency bonus support  
✅ **Skill Calculations**: Complete with expertise support  
✅ **Checkbox Field Mappings**: Systematically verified for all categories  
✅ **Spell Preparation Checkboxes**: Complete for levels 1-9  
✅ **Error Handling**: Graceful degradation implemented  
✅ **MCP Integration**: Tool schema and validation complete

## Next Steps

1. Run tests to verify all calculations
2. Test with sample character data
3. Verify PDF output shows correct bonuses and checkboxes
4. Add additional character data as needed

For detailed implementation information, see [data-model.md](./data-model.md) and [contracts/](./contracts/).
