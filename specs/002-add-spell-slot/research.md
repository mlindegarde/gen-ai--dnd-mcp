# Research: Add Spell Slot Tracking

**Date**: 2025-10-15  
**Feature**: Spell slot display for D&D 5e character sheets

## D&D 5e Spell Slot Progression

**Decision**: Use official D&D 5e spell slot tables from SRD  
**Rationale**: Ensures accuracy and compatibility with standard D&D rules  
**Alternatives considered**: Custom progression tables (rejected - would break compatibility)

### Spell Slot Tables by Class Level

Full casters (Wizard, Sorcerer, Cleric, Druid, Bard):
- Level 1: 2 first-level slots
- Level 2: 3 first-level slots  
- Level 3: 4 first-level, 2 second-level slots
- [Continue through level 20]

Half casters (Paladin, Ranger):
- Start at character level 2
- Slower progression (half-caster table)

Third casters (Eldritch Knight, Arcane Trickster):
- Start at character level 3
- Slowest progression (third-caster table)

## Multiclass Spellcasting

**Decision**: Implement multiclass spell slot calculation per D&D 5e rules  
**Rationale**: Required by spec, common use case in D&D  
**Alternatives considered**: Single-class only (rejected - spec requirement)

### Calculation Method
1. Determine spellcaster level for each class
2. Sum total spellcaster levels
3. Use combined level on full caster table
4. Round down half/third caster levels appropriately

## Integration Approach

**Decision**: Extend existing `calculate_derived_values()` method  
**Rationale**: Minimal code changes, consistent with current architecture  
**Alternatives considered**: 
- New spell slot service (rejected - over-engineering)
- Separate calculation module (rejected - adds complexity)

### Implementation Points
- Add spell slot calculation to `spell_system.rs`
- Extend `field_mapper.rs` with spell slot PDF field names
- Add calculated spell slots to derived values HashMap
- No new data structures needed - use existing pattern

## PDF Field Mapping

**Decision**: Map to existing D&D 5e character sheet spell slot fields  
**Rationale**: Standard character sheets have dedicated spell slot areas  
**Alternatives considered**: Custom field layout (rejected - breaks standard sheets)

### Field Names (to be confirmed from PDF)
- `Spell1stLevel`, `Spell2ndLevel`, etc.
- Or `SpellSlots1`, `SpellSlots2`, etc.
- Actual field names need extraction from PDF template
