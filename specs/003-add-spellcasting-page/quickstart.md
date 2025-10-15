# Quickstart: Spellcasting Page Header Population

**Feature**: 003-add-spellcasting-page  
**Date**: 2025-10-15

## Overview

Minimal addition to populate 4 spellcasting header fields using existing character data and simple D&D 5e calculations.

## Implementation

**Total changes**: ~30 lines across 2 existing files

1. **field_mapper.rs**: Add 4 PDF field name mappings
2. **pdf_filler.rs**: Add inline calculations (match statement + 2 formulas)

## Calculations

```rust
// Class → Ability mapping (9 lines)
let spellcasting_ability = match class_name.as_str() {
    "Wizard" | "Eldritch Knight" | "Arcane Trickster" => "Intelligence",
    "Cleric" | "Druid" | "Ranger" => "Wisdom", 
    "Sorcerer" | "Bard" | "Paladin" => "Charisma",
    _ => return, // Non-spellcaster
};

// D&D 5e formulas (2 lines)
let spell_save_dc = 8 + proficiency_bonus + ability_modifier;
let spell_attack_bonus = proficiency_bonus + ability_modifier;
```

## Testing

Add test cases to existing `pdf_filler_tests.rs`:
- Wizard with Intelligence 16 → save DC 13, attack +5
- Fighter → no spellcasting fields populated
