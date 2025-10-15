# Tasks: Spellcasting Page Header Population

**Feature**: `003-add-spellcasting-page`  
**Branch**: `003-add-spellcasting-page`  
**Created**: 2025-10-15

## Overview

Implement automatic population of spellcasting page header fields (class, ability, spell save DC, spell attack bonus) using existing character data and D&D 5e calculations.

## Task Summary

- **Total Tasks**: 8
- **User Story 1 (P1)**: 4 tasks - Core spellcasting header population
- **User Story 2 (P2)**: 2 tasks - Non-spellcaster handling  
- **Setup/Integration**: 2 tasks

## Dependencies

```
Phase 1 (Setup) → Phase 2 (US1) → Phase 3 (US2)
```

**User Story Dependencies**:
- US1 (P1): Independent - Core spellcasting functionality
- US2 (P2): Depends on US1 - Extends US1 with non-spellcaster handling

## Implementation Strategy

**MVP Scope**: User Story 1 only (automatic spellcasting header population for spellcasting classes)

**Incremental Delivery**:
1. **Phase 2**: Complete spellcasting header population (US1) - Delivers immediate value
2. **Phase 3**: Add non-spellcaster handling (US2) - Completes feature robustness

## Phase 1: Setup & Integration

### Goal
Prepare existing codebase for spellcasting header functionality by adding required PDF field mappings.

- [ ] T001 Add spellcasting class field mapping to src/field_mapper.rs
- [ ] T002 Add spellcasting ability field mapping to src/field_mapper.rs

**Completion Criteria**: Field mappings added for all 4 spellcasting header fields

## Phase 2: User Story 1 - Automatic Spellcasting Header Population (P1)

### Goal
Implement core spellcasting header calculations for spellcasting classes (Wizard, Cleric, Sorcerer, etc.)

### Independent Test Criteria
- Character with Wizard class and Intelligence 16 produces: class="Wizard", ability="Intelligence", save DC=13, attack bonus=+5
- Character with Cleric class and Wisdom 14 produces: class="Cleric", ability="Wisdom", save DC=12, attack bonus=+4  
- Character with Sorcerer class and Charisma 18 produces: class="Sorcerer", ability="Charisma", save DC=14, attack bonus=+6

### Tasks

- [ ] T003 [US1] Implement class-to-ability mapping function in src/pdf_filler.rs
- [ ] T004 [US1] Implement spellcasting header calculations in src/pdf_filler.rs
- [ ] T005 [US1] Integrate spellcasting header population into PDF filling workflow in src/pdf_filler.rs
- [ ] T006 [US1] Add unit tests for spellcasting header calculations in tests/unit/pdf_filler_tests.rs

**Completion Criteria**: All spellcasting classes correctly populate header fields with accurate D&D 5e calculations

## Phase 3: User Story 2 - Non-Spellcaster Handling (P2)

### Goal  
Ensure non-spellcasting classes (Fighter, Rogue, etc.) leave spellcasting header fields empty without errors.

### Independent Test Criteria
- Character with Fighter class leaves all spellcasting header fields empty
- Character with Rogue class leaves all spellcasting header fields empty
- No errors or exceptions thrown for non-spellcasting classes

### Tasks

- [ ] T007 [US2] Extend class-to-ability mapping to handle non-spellcasting classes in src/pdf_filler.rs
- [ ] T008 [US2] Add unit tests for non-spellcaster handling in tests/unit/pdf_filler_tests.rs

**Completion Criteria**: Non-spellcasting classes gracefully handled with empty header fields

## Parallel Execution Opportunities

**Phase 1**: Sequential (field mappings must be added in order)
**Phase 2**: Tasks T003-T005 sequential (dependencies), T006 can be parallel after T004
**Phase 3**: Tasks T007-T008 can be parallel

## Technical Implementation Notes

### Key Files Modified
- `src/field_mapper.rs`: Add 2 new PDF field mappings
- `src/pdf_filler.rs`: Add ~15 lines of spellcasting calculations  
- `tests/unit/pdf_filler_tests.rs`: Add test cases

### PDF Field Mappings (Discovered)
- `spellcasting_class` → `"Spellcasting Class 2"`
- `spellcasting_ability` → `"SpellcastingAbility 2"`
- `spell_save_dc` → `"SpellSaveDC  2"` (note: extra space)
- `spell_attack_bonus` → `"SpellAtkBonus 2"`

### D&D 5e Calculations
- **Spell Save DC**: 8 + proficiency_bonus + ability_modifier
- **Spell Attack Bonus**: proficiency_bonus + ability_modifier
- **Ability Modifier**: Use existing `AbilityScores::modifier()` method
- **Proficiency Bonus**: Use existing `dnd_validator::proficiency_bonus()` function

### Spellcasting Classes Supported
- **Intelligence**: Wizard, Eldritch Knight, Arcane Trickster
- **Wisdom**: Cleric, Druid, Ranger  
- **Charisma**: Sorcerer, Bard, Paladin
