# Quickstart: Character Sheet Field Population Bug Fixes

**Feature**: 005-fix-bugs-i  
**Date**: 2025-10-17

## Overview

This feature fixes five critical bugs in D&D character sheet PDF generation:
1. **Personality Traits** - Now populates from character narrative data
2. **Passive Perception** - Now calculates automatically (10 + Wis mod + prof bonus)  
3. **Hit Dice** - Now displays based on class and level in separate HDTotal and HD fields
4. **Currency Fields** - Now populates CP, SP, EP, GP, PP from equipment data
5. **Features & Traits** - Now populates from character features and racial traits data

## Quick Test

### 1. Test Character Data
```json
{
  "character": {
    "name": "Test Character",
    "class": "Fighter", 
    "level": 3,
    "race": "Human"
  },
  "abilities": {
    "wisdom": 14,
    "strength": 16,
    "dexterity": 12,
    "constitution": 15,
    "intelligence": 10,
    "charisma": 8
  },
  "narrative": {
    "personality_traits": "I am brave and always help others in need."
  },
  "proficiencies": {
    "skills": ["athletics", "perception", "intimidation"]
  },
  "equipment": {
    "currency": {
      "cp": 50,
      "sp": 25, 
      "gp": 100,
      "pp": 5
    }
  },
  "features_traits": {
    "features": ["Action Surge", "Fighting Style"],
    "traits": ["Darkvision", "Fey Ancestry"]
  }
}
```

### 2. Expected PDF Results
- **Personality Traits Field**: "I am brave and always help others in need."
- **Passive Perception Field**: "14" (10 + 2 Wis mod + 2 prof bonus)
- **Hit Dice Fields**: HDTotal="3", HD="d10" (3rd level Fighter)
- **Currency Fields**: CP=50, SP=25, GP=100, PP=5
- **Features & Traits Field**: "Action Surge, Fighting Style, Darkvision, Fey Ancestry"

### 3. Run Test
```bash
# Using MCP tool
fill_dnd_character_sheet character_data='[JSON above]' output_path='test_bugfix.pdf'

# Using binary directly  
cargo run --bin dnd-character-sheet-filler test-docs
```

## Implementation Details

### Files Modified
- `src/character_model.rs` - Add FeaturesTraits struct, update CharacterData
- `src/field_mapper.rs` - Add PDF field mappings in FieldMapper::new()
- `src/pdf_filler.rs` - Add calculation logic in get_field_values() method

### Specific Code Changes

#### 1. character_model.rs
```rust
// Add new struct after CharacterNarrative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesTraits {
    pub features: Option<Vec<String>>,
    pub traits: Option<Vec<String>>,
}

// Update CharacterData struct
pub struct CharacterData {
    // ... existing fields ...
    pub features_traits: Option<FeaturesTraits>, // Add this
}
```

#### 2. field_mapper.rs - In FieldMapper::new()
```rust
// Update existing mapping
field_map.insert("personality_traits".to_string(), "PersonalityTraits".to_string());

// Add new mappings
field_map.insert("passive_perception".to_string(), "Passive".to_string());
field_map.insert("hit_dice_total".to_string(), "HDTotal".to_string());
field_map.insert("hit_dice_type".to_string(), "HD".to_string());
field_map.insert("features_traits".to_string(), "Features & Traits".to_string());
```

#### 3. pdf_filler.rs - In get_field_values() method
```rust
// Passive perception (after ability scores)
let passive_perception = 10 + self.calculate_modifier(abilities.wisdom) + 
    if is_perception_proficient { prof_bonus } else { 0 };

// Hit dice (after proficiency bonus)  
let hit_die_type = match character.class.as_str() {
    "Barbarian" => "d12",
    "Fighter" | "Paladin" | "Ranger" => "d10",
    // ... other classes
};

// Features & traits (after narrative)
let combined_text = features.join(", ") + ", " + traits.join(", ");
```

### Field Mappings Added
```rust
"personality_traits" -> "PersonalityTraits"
"passive_perception" -> "Passive" 
"hit_dice_total" -> "HDTotal"
"hit_dice_type" -> "HD"
"cp" -> "CP"
"sp" -> "SP" 
"ep" -> "EP"
"gp" -> "GP"
"pp" -> "PP"
"features_traits" -> "Features & Traits"
```

## Testing Strategy

### Unit Tests
- Passive perception calculation with/without proficiency
- Hit dice formatting for all D&D classes
- Currency field mapping with missing values
- Personality traits truncation for long text

### Integration Tests  
- Complete character sheet generation with all bug fix fields
- PDF field verification using test character data
- Error handling for missing optional data

### Edge Cases Covered
- Missing narrative data (empty personality traits)
- Invalid wisdom scores (skip passive perception)
- Unknown character classes (default to d8 hit dice)
- Missing currency data (empty currency fields)

## Constitutional Compliance

✅ **Local-First**: All processing remains local, no cloud dependencies  
✅ **Minimal Dependencies**: No new dependencies added  
✅ **D&D 5e Accuracy**: Passive perception and hit dice follow official rules  
✅ **Simplicity First**: Minimal changes to existing architecture  
✅ **Single Responsibility**: Changes isolated to appropriate modules  
✅ **Scope Limitations**: Single-class characters only, no multi-classing

## Performance Impact

- **Calculation overhead**: ~1ms for passive perception and hit dice formatting
- **Memory impact**: Negligible (simple string operations)
- **PDF processing time**: No measurable increase (well within 2-second target)

## Rollback Plan

If issues arise, revert changes to:
- `src/field_mapper.rs` (remove new field mappings)
- `src/pdf_filler.rs` (remove calculation methods)
- Tests remain for future implementation

Original functionality preserved - no breaking changes to existing features.
