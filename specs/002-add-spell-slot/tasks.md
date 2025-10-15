# Tasks: Add Spell Slot Tracking

**Input**: Design documents from `/specs/002-add-spell-slot/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are NOT requested in the feature specification - implementation only.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Validate existing project structure matches plan.md requirements
- [x] T002 [P] Create test fixtures directory tests/fixtures/ for spell slot test data
- [x] T003 [P] Create integration test directory tests/integration/ for spell slot tests

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 🚨 CRITICAL: Fix incorrect spell slot field mappings in src/field_mapper.rs (replace L1-L9 with SlotsTotal 19-27)
- [x] T005 [P] Add spell slot progression tables to src/spell_system.rs (full/half/third caster tables from research.md)
- [x] T006 [P] Add CasterType enum to src/spell_system.rs (Full, Half, Third, None variants)
- [x] T007 Implement get_caster_type() method in src/spell_system.rs for class classification

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Display Available Spell Slots (Priority: P1) 🎯 MVP

**Goal**: Display spell slot counts for levels 1-9 on character sheets, hide for non-casters

**Independent Test**: Generate character sheet with spellcaster and verify spell slot fields appear with correct counts

### Implementation for User Story 1

- [x] T008 [P] [US1] Implement calculate_spell_slots() method in src/spell_system.rs returning HashMap<String, u8>
- [x] T009 [P] [US1] Implement get_spell_slots_for_single_class() helper method in src/spell_system.rs
- [x] T010 [US1] Integrate spell slot calculation into get_field_values() method in src/pdf_filler.rs
- [x] T011 [US1] Add spell slot field population logic to pdf_filler.rs (only populate non-zero slots)
- [x] T012 [P] [US1] Create spell slot test fixtures in tests/fixtures/spell-slot-characters.json
- [x] T013 [P] [US1] Create spell slot integration tests in tests/integration/spell_slot_tests.rs
- [x] T014 [US1] Add validation for spell slot display (non-casters show no fields, casters show correct counts)

**Checkpoint**: At this point, User Story 1 should be fully functional - spell slots display correctly for all character types

---

## Phase 4: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect the spell slot feature

- [x] T015 [P] Add error handling for invalid character classes in spell slot calculation
- [x] T016 [P] Add logging for spell slot calculation operations
- [x] T017 Validate implementation against quickstart.md test scenarios
- [x] T018 [P] Update documentation with spell slot feature details

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS user story
- **User Story 1 (Phase 3)**: Depends on Foundational phase completion
- **Polish (Phase 4)**: Depends on User Story 1 completion

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories

### Within User Story 1

- T008, T009 (spell system methods) can run in parallel
- T010, T011 (PDF integration) depend on T008 completion
- T012, T013 (test fixtures and tests) can run in parallel with implementation
- T014 (validation) depends on all implementation tasks

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- T005, T006 in Foundational can run in parallel after T004 completes
- T008, T009 in User Story 1 can run in parallel
- T012, T013 can run in parallel with implementation tasks
- All Polish tasks marked [P] can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch spell system implementation in parallel:
Task: "Implement calculate_spell_slots() method in src/spell_system.rs"
Task: "Implement get_spell_slots_for_single_class() helper method in src/spell_system.rs"

# Launch test creation in parallel with implementation:
Task: "Create spell slot test fixtures in tests/fixtures/spell-slot-characters.json"
Task: "Create spell slot integration tests in tests/integration/spell_slot_tests.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - especially T004 field mapping fix)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test spell slot display with wizard, paladin, and barbarian characters
5. Deploy/demo if ready

### Critical Path

The most important tasks for immediate functionality:

1. **T004** (Fix field mappings) - BLOCKS everything
2. **T005** (Add progression tables) - Required for calculations
3. **T008** (Calculate spell slots method) - Core functionality
4. **T010** (PDF integration) - Makes it visible to users

### Validation Checklist

Using exact values from data-model.md and contracts/:

- [ ] **FIRST**: Verify field mappings use SlotsTotal 19-27 format (not L1-L9)
- [ ] Wizard level 5 shows spell slots: 4/3/2/0/0/0/0/0/0
- [ ] Paladin level 5 shows spell slots: 4/2/0/0/0/0/0/0/0
- [ ] Eldritch Knight Fighter level 7 shows spell slots: 4/2/0/0/0/0/0/0/0
- [ ] Barbarian shows no spell slot fields on PDF
- [ ] All spell slot fields map to correct PDF field names
- [ ] No errors in character sheet generation

---

## Notes

- [P] tasks = different files, no dependencies
- [US1] label maps task to User Story 1 for traceability
- User Story 1 should be independently completable and testable
- Commit after each task or logical group
- Stop at checkpoint to validate story independently
- **CRITICAL**: T004 must be completed first - incorrect field mappings will break everything
