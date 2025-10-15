# Data Model: Spell Slot Tracking

**Date**: 2025-10-15  
**Feature**: Display-only spell slot calculation for D&D 5e characters

## Entities

### SpellSlots (Calculated Value)
**Purpose**: Represents calculated spell slot maximums for display on character sheet

**Fields**:
- `level_1`: u8 - Maximum 1st level spell slots
- `level_2`: u8 - Maximum 2nd level spell slots  
- `level_3`: u8 - Maximum 3rd level spell slots
- `level_4`: u8 - Maximum 4th level spell slots
- `level_5`: u8 - Maximum 5th level spell slots
- `level_6`: u8 - Maximum 6th level spell slots
- `level_7`: u8 - Maximum 7th level spell slots
- `level_8`: u8 - Maximum 8th level spell slots
- `level_9`: u8 - Maximum 9th level spell slots

**Validation Rules**:
- All values must be 0-9 (D&D 5e maximum)
- Non-spellcasters have all values as 0
- Spell slot progression must follow D&D 5e rules

**Relationships**:
- Calculated from Character.class and Character.level
- No persistence - computed on demand
- Included in calculated_fields HashMap for PDF display

### Character (Existing - Extended)
**Purpose**: Source data for spell slot calculations

**Relevant Fields**:
- `class`: String - Character class (determines spellcasting type)
- `level`: u8 - Character level (determines spell slot progression)

**Spell Slot Calculation Logic (Simplified)**:
1. Determine if character is a spellcaster based on class
2. Use character level directly with appropriate progression table
3. Look up spell slots from progression table
4. Return SpellSlots struct

## State Transitions

**N/A** - Display-only feature with no state changes

## Validation Rules

### Character Class Validation
- Must be valid D&D 5e class name
- Determines spellcasting progression type:
  - Full casters: Wizard, Sorcerer, Cleric, Druid, Bard
  - Half casters: Paladin, Ranger  
  - Third casters: Eldritch Knight Fighter, Arcane Trickster Rogue
  - Non-casters: Barbarian, Fighter, Monk, Rogue (base classes)

### Level Validation  
- Must be 1-20 (D&D 5e level range)
- Determines spell slot availability and quantity
