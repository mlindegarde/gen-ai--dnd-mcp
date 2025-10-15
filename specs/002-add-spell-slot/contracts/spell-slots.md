# Spell Slot Calculation Contract

**Date**: 2025-10-15  
**Type**: Internal API Contract

## Function Interface

### `calculate_spell_slots(character: &CharacterData) -> HashMap<String, u8>`

**Purpose**: Calculate maximum spell slots for a single-class D&D 5e character and return as logical field mappings

**Input**: 
- `character`: Reference to CharacterData containing class and level information

**Output**:
- HashMap with logical field names as keys and spell slot counts as u8 values
- Empty HashMap for non-spellcasting characters

**Example Output**:
```rust
{
    "spell_slots_1": 4,
    "spell_slots_2": 3, 
    "spell_slots_3": 2,
    "spell_slots_4": 0,
    "spell_slots_5": 0,
    "spell_slots_6": 0,
    "spell_slots_7": 0, 
    "spell_slots_8": 0,
    "spell_slots_9": 0
}
```

**PDF Integration**: These logical names map to PDF fields via FieldMapper:
- `spell_slots_1` → `SlotsTotal 19`
- `spell_slots_2` → `SlotsTotal 20`
- etc.

## Integration Points

### PDF Field Mapping
- Integrates with existing `FieldMapper` system
- Uses corrected spell slot field names (`SlotsTotal X` format)
- Fields populated during `get_field_values()` execution in `pdf_filler.rs`

### Character Data Requirements
- Requires `character.class` and `character.level` fields
- **No multiclass support** - single class only
- No additional data structures needed

### Calculation Rules
- Follows D&D 5e spell slot progression tables for single classes
- **Simplified logic**: Use character level directly with appropriate caster type table
- Returns 0 for spell levels not available to character
- Non-spellcasters return empty HashMap (no fields populated)

## Caster Type Classification

### Full Casters (use full progression table)
- Wizard, Sorcerer, Cleric, Druid, Bard

### Half Casters (use half progression table)  
- Paladin, Ranger

### Third Casters (use third progression table)
- Fighter (Eldritch Knight subclass)
- Rogue (Arcane Trickster subclass)

### Non-Casters (return empty HashMap)
- Barbarian, Fighter (base), Monk, Rogue (base)

## Error Handling

### Invalid Class
- Unknown class names default to non-spellcaster
- Log warning but continue processing

### Invalid Level  
- Levels outside 1-20 range clamp to valid range
- Log warning but continue processing

### No Multiclass Support
- **Scope limitation**: Only single-class characters supported
- Future enhancement can add multiclass calculations

**Design Principle**: Graceful degradation - never fail character sheet generation due to spell slot calculation errors
