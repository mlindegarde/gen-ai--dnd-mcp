# Implementation Tasks: Character Sheet PDF Population (Saves, Skills, Spells)

**Feature**: Character Sheet PDF Population (Saves, Skills, Spells)  
**Branch**: `004-i-want-to`  
**Generated**: 2025-10-16  
**Source**: [spec.md](./spec.md) | [plan.md](./plan.md)

## Task Overview

**Total Tasks**: 17  
**User Stories**: 4 (P1: Saving Throws, P2: Skills & Spells, P3: Proficiency Indicators)  
**Parallel Opportunities**: 6 tasks can run in parallel  
**MVP Scope**: User Story 1 (Saving Throws Population)

## Phase 1: Setup

**Goal**: Initialize project structure and verify existing components

- [ ] T001 Verify existing codebase structure matches plan.md requirements AND confirm calculate_saving_throw_bonus() and calculate_skill_bonus() functions exist in current implementation
- [ ] T002 Confirm field_mapper.rs contains systematically verified checkbox mappings

## Phase 2: Foundational

**Goal**: Extend core modules with shared functionality and error handling needed by all user stories

- [ ] T003 [P] Add checkbox marking capability to pdf_filler.rs
- [ ] T004 [P] Extend errors.rs with saving throw and skill error types
- [ ] T004.1 [P] Add ability score validation functions to dnd_validator.rs
- [ ] T004.2 [P] Add negative modifier handling to calculation functions in pdf_filler.rs
- [ ] T004.3 [P] Add error indicator display logic for invalid calculations in pdf_filler.rs

## Phase 3: User Story 1 - Basic Saving Throws Population (P1)

**Goal**: Calculate and populate saving throw bonuses in PDF  
**Independent Test**: Provide character with ability scores and saving throw proficiencies, verify PDF shows correct calculated bonuses

- [ ] T005 [US1] Implement saving throw bonus population in pdf_filler.rs (verify calculate_saving_throw_bonus() exists first, create if missing)
- [ ] T006 [US1] Add saving throw field mapping integration using verified checkbox fields (Check Box 11, 18-22)
- [ ] T007 [US1] Add basic unit test for saving throw bonus calculations

## Phase 4: User Story 2 - Skills Population with Proficiency (P2)

**Goal**: Calculate and populate skill bonuses in PDF  
**Independent Test**: Provide character with skill proficiencies, verify PDF shows correct skill bonuses

- [ ] T008 [US2] Implement skill bonus population in pdf_filler.rs (verify calculate_skill_bonus() exists first, create if missing)
- [ ] T009 [US2] Add skill field mapping integration using verified checkbox fields (Check Box 23-40)

## Phase 5: User Story 4 - Spell Preparation Checkbox Marking (P2)

**Goal**: Mark spell preparation checkboxes for prepared spells  
**Independent Test**: Provide character with prepared spells across levels, verify checkboxes marked in correct spell level sections

- [ ] T010 [US4] Implement spell preparation checkbox marking in pdf_filler.rs using spell.prepared field
- [ ] T011 [US4] Add spell preparation field mapping using verified checkbox mappings (levels 1-9)
- [ ] T011.1 [US4] Add spell data validation in pdf_filler.rs before checkbox marking
- [ ] T011.2 [US4] Add spell overflow handling when more spells than available checkboxes

## Phase 6: User Story 3 - Proficiency Indicator Marking (P3)

**Goal**: Mark proficiency indicators for saving throws and skills  
**Independent Test**: Provide character with specific proficiencies, verify appropriate visual markers shown

- [ ] T012 [US3] Integrate proficiency checkbox marking with saving throw and skill population logic

## Dependencies

```
Phase 1 (Setup) → Phase 2 (Foundational) → Phase 3 (US1)
                                        → Phase 4 (US2)  
                                        → Phase 5 (US4)
                                        → Phase 6 (US3)
```

**User Story Dependencies**: All user stories are independent after foundational phase completion

## Parallel Execution Opportunities

**Within Phase 2**: T003, T004, T004.1, T004.2, T004.3 can run in parallel (different files/functions)  
**Across User Stories**: US1, US2, US4 can be implemented in parallel after foundational phase  
**US3 Integration**: Must wait for US1 and US2 completion (integrates their logic)  
**Phase 5 Validation**: T011.1-T011.2 can run in parallel with T010-T011

## Implementation Strategy

**MVP Approach**: Implement User Story 1 first for immediate value  
**Incremental Delivery**: Each user story provides independent, testable functionality  
**Risk Mitigation**: Foundational checkbox marking capability built first to support all stories

## File Modification Summary

**Primary Files**:
- `src/pdf_filler.rs` - Core implementation (T003, T004.2, T004.3, T005, T008, T010, T011.1, T011.2, T012)
- `src/errors.rs` - Error handling (T004)
- `src/dnd_validator.rs` - Validation logic (T004.1)

**Existing Files Used**:
- `src/field_mapper.rs` - Contains verified checkbox mappings (no changes needed)
- `src/character_model.rs` - Contains calculation functions (no changes needed)

## Testing Strategy

**Unit Tests**: Basic calculation validation (T007)  
**Manual Testing**: Sample character data verification per user story  
**No Integration Tests**: Simplified for hobby project scope

## Success Criteria

- [ ] All saving throw bonuses calculated and populated correctly
- [ ] All skill bonuses calculated and populated correctly  
- [ ] All spell preparation checkboxes marked for prepared spells
- [ ] All proficiency indicators marked appropriately
- [ ] PDF maintains original formatting and layout
- [ ] Error handling allows partial success when some data invalid
