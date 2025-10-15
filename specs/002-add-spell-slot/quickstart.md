# Quickstart: Spell Slot Tracking Implementation

**Date**: 2025-10-15  
**Estimated Time**: 2-3 hours

## Prerequisites

Before starting, review these detailed reference files:
- **[spell-slot-tables.md](spell-slot-tables.md)** - Complete D&D 5e progression tables with exact values
- **[pdf-field-discovery.md](pdf-field-discovery.md)** - PDF field names (already implemented)
- **[data-model.md](data-model.md)** - SpellSlots entity structure
- **[contracts/spell-slots.md](contracts/spell-slots.md)** - API contract details

## Implementation Steps

### 🚨 CRITICAL: Fix Field Mappings First (10 minutes)

**File**: `src/field_mapper.rs`

**REQUIRED**: Replace incorrect field mappings (lines 86-94) with validated names from [pdf-field-discovery.md](pdf-field-discovery.md):

```rust
// Replace existing WRONG mappings:
// field_map.insert("spell_slots_1".to_string(), "L1".to_string()); // ❌

// With CORRECT mappings:
field_map.insert("spell_slots_1".to_string(), "SlotsTotal 19".to_string());
field_map.insert("spell_slots_2".to_string(), "SlotsTotal 20".to_string());
field_map.insert("spell_slots_3".to_string(), "SlotsTotal 21".to_string());
field_map.insert("spell_slots_4".to_string(), "SlotsTotal 22".to_string());
field_map.insert("spell_slots_5".to_string(), "SlotsTotal 23".to_string());
field_map.insert("spell_slots_6".to_string(), "SlotsTotal 24".to_string());
field_map.insert("spell_slots_7".to_string(), "SlotsTotal 25".to_string());
field_map.insert("spell_slots_8".to_string(), "SlotsTotal 26".to_string());
field_map.insert("spell_slots_9".to_string(), "SlotsTotal 27".to_string());
```

**Reference**: See [pdf-field-discovery.md](pdf-field-discovery.md) for validation details using debug-sheet.pdf.

### 1. Implement Spell Slot Calculation (45 minutes)

**File**: `src/spell_system.rs`

Add new method using the progression tables from [spell-slot-tables.md](spell-slot-tables.md):

```rust
impl SpellSystem {
    pub fn calculate_spell_slots(&self, character: &CharacterData) -> HashMap<String, u8> {
        let mut spell_slots = HashMap::new();
        
        // 1. Determine spellcaster type from class (simplified - no multiclass)
        let caster_type = self.get_caster_type(&character.class);
        
        // 2. Look up spell slots from appropriate progression table
        let slots = self.get_spell_slots_for_single_class(character.level, caster_type);
        
        // 3. Return HashMap with level -> count mapping
        for level in 1..=9 {
            if let Some(count) = slots.get(&level) {
                spell_slots.insert(format!("spell_slots_{}", level), *count);
            }
        }
        
        spell_slots
    }
    
    fn get_caster_type(&self, class: &str) -> CasterType {
        match class.to_lowercase().as_str() {
            "wizard" | "sorcerer" | "cleric" | "druid" | "bard" => CasterType::Full,
            "paladin" | "ranger" => CasterType::Half,
            "fighter" | "rogue" => CasterType::Third, // Assumes subclass casting
            _ => CasterType::None,
        }
    }
    
    fn get_spell_slots_for_single_class(&self, level: u8, caster_type: CasterType) -> HashMap<u8, u8> {
        // Use progression tables from spell-slot-tables.md
        // Much simpler - just use level directly with appropriate table
    }
}

enum CasterType {
    Full,
    Half, 
    Third,
    None,
}
```

**Reference**: See [spell-slot-tables.md](spell-slot-tables.md) for exact progression values. No multiclass calculations needed.

### 2. Integrate with PDF Filler (30 minutes)

**File**: `src/pdf_filler.rs`

Extend `get_field_values()` method using existing field mappings from [pdf-field-discovery.md](pdf-field-discovery.md):

```rust
// In get_field_values()
let spell_slots = self.spell_system.calculate_spell_slots(character_data);
for (logical_name, count) in spell_slots {
    if let Some(field_name) = self.field_mapper.get_pdf_field_name(&logical_name) {
        if count > 0 {  // Only show non-zero slots
            fields.insert(field_name.clone(), count.to_string());
        }
    }
}
```

**Reference**: Field mappings `spell_slots_1` → `SlotsTotal 19`, etc. are documented in [pdf-field-discovery.md](pdf-field-discovery.md).

### 3. Add Test Data (15 minutes)

**File**: `tests/fixtures/spell-slot-characters.json`

Create test characters using values from [spell-slot-tables.md](spell-slot-tables.md):

```json
{
  "wizard_level_5": {
    "character": {
      "name": "Test Wizard",
      "class": "Wizard", 
      "level": 5,
      "race": "Human"
    },
    "expected_slots": {
      "SlotsTotal 19": "4", "SlotsTotal 20": "3", "SlotsTotal 21": "2", 
      "SlotsTotal 22": "0", "SlotsTotal 23": "0", "SlotsTotal 24": "0", 
      "SlotsTotal 25": "0", "SlotsTotal 26": "0", "SlotsTotal 27": "0"
    }
  },
  "paladin_level_5": {
    "character": {
      "name": "Test Paladin",
      "class": "Paladin",
      "level": 5,
      "race": "Human"
    },
    "expected_slots": {
      "SlotsTotal 19": "4", "SlotsTotal 20": "2", "SlotsTotal 21": "0",
      "SlotsTotal 22": "0", "SlotsTotal 23": "0", "SlotsTotal 24": "0", 
      "SlotsTotal 25": "0", "SlotsTotal 26": "0", "SlotsTotal 27": "0"
    }
  },
  "barbarian_level_5": {
    "character": {
      "name": "Test Barbarian",
      "class": "Barbarian",
      "level": 5,
      "race": "Human"
    },
    "expected_slots": {}
  }
}
```

### 4. Write Tests (45 minutes)

**File**: `tests/integration/spell_slot_tests.rs`

Test cases based on [spell-slot-tables.md](spell-slot-tables.md) values:

```rust
#[test]
fn test_wizard_level_5_spell_slots() {
    // Expected: 4/3/2/0/0/0/0/0/0 (from full caster table)
}

#[test] 
fn test_paladin_level_5_spell_slots() {
    // Expected: 4/2/0/0/0/0/0/0/0 (from half caster table)
}

#[test]
fn test_barbarian_no_spell_slots() {
    // Expected: No spell slot fields (non-caster)
}

#[test]
fn test_eldritch_knight_fighter_level_7() {
    // Expected: 4/2/0/0/0/0/0/0/0 (from third caster table)
}
```

## Testing Commands

```bash
# Run spell slot specific tests
cargo test spell_slot

# Test with sample character  
cargo run test

# Verify PDF output
cargo run --bin dnd-character-sheet-filler
```

## Validation Checklist

Using exact values from [spell-slot-tables.md](spell-slot-tables.md):

- [ ] **FIRST**: Fix field mappings in field_mapper.rs (SlotsTotal X format)
- [ ] Wizard level 5 shows correct spell slots (4/3/2/0/0/0/0/0/0)
- [ ] Paladin level 5 shows correct spell slots (4/2/0/0/0/0/0/0/0)  
- [ ] Eldritch Knight Fighter level 7 shows correct spell slots (4/2/0/0/0/0/0/0/0)
- [ ] Barbarian shows no spell slot fields on PDF
- [ ] All spell slot fields map to correct PDF field names (SlotsTotal 19-27)
- [ ] No errors in character sheet generation

## Reference Files

- **[spell-slot-tables.md](spell-slot-tables.md)** - All D&D 5e progression tables and multiclass rules
- **[pdf-field-discovery.md](pdf-field-discovery.md)** - PDF field mappings (L1-L9)
- **[data-model.md](data-model.md)** - SpellSlots entity and validation rules  
- **[contracts/spell-slots.md](contracts/spell-slots.md)** - Function interface and integration points

## Common Issues

**Issue**: Spell slots not appearing on PDF  
**Solution**: Verify field mappings in [pdf-field-discovery.md](pdf-field-discovery.md) - use SlotsTotal 19-27, not L1-L9

**Issue**: Multiclass calculation incorrect  
**Solution**: Check formulas in [spell-slot-tables.md](spell-slot-tables.md) multiclass section

**Issue**: Wrong spell slot counts  
**Solution**: Verify against progression tables in [spell-slot-tables.md](spell-slot-tables.md)

**Issue**: Field mappings still wrong  
**Solution**: Ensure field_mapper.rs uses SlotsTotal format, not L format
