# Data Model: Spellcasting Page Header Population

**Feature**: 003-add-spellcasting-page  
**Date**: 2025-10-15

## Minimal Changes

No new data structures needed. Use existing Character model:

```rust
pub struct Character {
    pub class: String,        // Single class (e.g., "Wizard")
    pub level: u8,           // Character level for proficiency bonus
    // ... other fields
}

pub struct AbilityScores {
    pub intelligence: u8,
    pub wisdom: u8, 
    pub charisma: u8,
    // ... other abilities
}
```

## Calculations (inline in pdf_filler.rs)

```rust
// Simple class-to-ability mapping
let spellcasting_ability = match character.class.as_str() {
    "Wizard" | "Eldritch Knight" | "Arcane Trickster" => "intelligence",
    "Cleric" | "Druid" | "Ranger" => "wisdom", 
    "Sorcerer" | "Bard" | "Paladin" => "charisma",
    _ => return, // Non-spellcaster
};

// Use existing functions
let ability_modifier = ability_scores.modifier(spellcasting_ability);
let proficiency_bonus = dnd_validator::proficiency_bonus(character.level);
let spell_save_dc = 8 + proficiency_bonus + ability_modifier;
let spell_attack_bonus = proficiency_bonus + ability_modifier;
```

## PDF Fields to Populate

✅ **All 4 field names discovered using debug-sheet.pdf method:**

- ✅ Spellcasting Class field: `"Spellcasting Class 2"`
- ✅ Spellcasting Ability field: `"SpellcastingAbility 2"`  
- ✅ Spell Save DC field: `"SpellSaveDC  2"` (note: extra space)
- ✅ Spell Attack Bonus field: `"SpellAtkBonus 2"`

**Next Step**: Add these 2 new mappings to `field_mapper.rs` (save DC and attack bonus already exist but with different field names)
