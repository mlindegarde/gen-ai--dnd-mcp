# Spell Slot Calculation Contract

**Date**: 2025-10-15  
**Type**: Internal API Contract

## Function Interface

### `calculate_spell_slots(character: &CharacterData) -> HashMap<String, String>`

**Purpose**: Calculate maximum spell slots for a D&D 5e character and return as PDF field mappings

**Input**: 
- `character`: Reference to CharacterData containing class and level information

**Output**:
- HashMap with PDF field names as keys and spell slot counts as string values
- Empty HashMap for non-spellcasting characters

**Example Output**:
```rust
{
    "Spell1stLevel": "4",
    "Spell2ndLevel": "3", 
    "Spell3rdLevel": "3",
    "Spell4thLevel": "1",
    "Spell5thLevel": "0",
    "Spell6thLevel": "0",
    "Spell7thLevel": "0", 
    "Spell8thLevel": "0",
    "Spell9thLevel": "0"
}
```

## Integration Points

### PDF Field Mapping
- Integrates with existing `FieldMapper` system
- Adds spell slot field names to field mapping table
- Fields populated during `get_field_values()` execution

### Character Data Requirements
- Requires `character.class` and `character.level` fields
- Optional `character.multiclass` for multiclass calculations
- No additional data structures needed

### Calculation Rules
- Follows D&D 5e spell slot progression tables
- Handles multiclass spellcaster level calculation
- Returns "0" for spell levels not available to character
- Non-spellcasters return empty HashMap (no fields populated)

## Error Handling

### Invalid Class
- Unknown class names default to non-spellcaster
- Log warning but continue processing

### Invalid Level  
- Levels outside 1-20 range clamp to valid range
- Log warning but continue processing

### Multiclass Errors
- Invalid multiclass data falls back to primary class only
- Log warning but continue processing

**Design Principle**: Graceful degradation - never fail character sheet generation due to spell slot calculation errors
