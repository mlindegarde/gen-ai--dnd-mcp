# PDF Field Discovery: Spell Slot Fields

**Date**: 2025-10-15  
**Source**: Validated using debug-sheet.pdf with populated LX values

## ✅ VALIDATED: Actual Spell Slot Field Names

### Correct Field Mappings (Discovered from debug-sheet.pdf)

```rust
// CORRECT spell slot field mappings
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

### ❌ INCORRECT: Previous Field Mappings in field_mapper.rs

The existing mappings in `src/field_mapper.rs` are **WRONG**:

```rust
// WRONG - These don't exist in the PDF
field_map.insert("spell_slots_1".to_string(), "L1".to_string());  // ❌
field_map.insert("spell_slots_2".to_string(), "L2".to_string());  // ❌
// etc.
```

## Validated PDF Field Name Mapping Table

| Logical Name | **CORRECT** PDF Field Name | Spell Level | Validated Value |
|--------------|----------------------------|-------------|-----------------|
| `spell_slots_1` | `SlotsTotal 19` | 1st level | L1 |
| `spell_slots_2` | `SlotsTotal 20` | 2nd level | L2 |
| `spell_slots_3` | `SlotsTotal 21` | 3rd level | L3 |
| `spell_slots_4` | `SlotsTotal 22` | 4th level | L4 |
| `spell_slots_5` | `SlotsTotal 23` | 5th level | L5 |
| `spell_slots_6` | `SlotsTotal 24` | 6th level | L6 |
| `spell_slots_7` | `SlotsTotal 25` | 7th level | L7 |
| `spell_slots_8` | `SlotsTotal 26` | 8th level | L8 |
| `spell_slots_9` | `SlotsTotal 27` | 9th level | L9 |

## Discovery Method

1. **Debug Sheet Analysis**: Used `docs/debug-sheet.pdf` populated with `LX` values
2. **Field Extraction**: `cargo run read docs/debug-sheet.pdf | grep SlotsTotal`
3. **Validation**: Confirmed each field contains the expected `LX` value

## Required Changes

### 1. Update field_mapper.rs
**File**: `src/field_mapper.rs`  
**Action**: Replace lines 86-94 with correct field names

```rust
// Replace existing WRONG mappings with CORRECT ones
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

### 2. Update quickstart.md validation
**File**: `quickstart.md`  
**Action**: Update validation checklist to use correct field names

## Conclusion

**Critical Fix Required**: The existing field mappings in `field_mapper.rs` are incorrect and must be updated before implementing spell slot calculation. The actual PDF field names are `SlotsTotal X` format, not simple `LX` format.

### Next Steps
1. **FIRST**: Fix field mappings in `field_mapper.rs` 
2. **THEN**: Implement spell slot calculation logic
3. **TEST**: Verify spell slots appear correctly on PDF using corrected field names
