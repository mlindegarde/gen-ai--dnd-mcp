# Tasks: Character Sheet Field Population Bug Fixes

**Input**: Design documents from `/specs/005-fix-bugs-i/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are OPTIONAL per user request - focusing on implementation only

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Verify embedded PDF template is working in src/mcp_server.rs
- [x] T002 [P] Create test character data with all 5 bug fix fields in tests/fixtures/

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T003 Add FeaturesTraits struct to src/character_model.rs
- [x] T004 Update CharacterData struct with features_traits field in src/character_model.rs
- [x] T005 [P] Update personality_traits field mapping from "Personality" to "PersonalityTraits" in src/field_mapper.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Character Narrative Display (Priority: P1) 🎯 MVP

**Goal**: Personality traits from character narrative data appear in PersonalityTraits PDF field

**Independent Test**: Generate PDF with personality traits data, verify PersonalityTraits field contains the text

### Implementation for User Story 1

- [x] T006 [US1] Verify PersonalityTraits field mapping exists in src/field_mapper.rs
- [x] T007 [US1] Confirm personality traits population works in existing get_field_values() method in src/pdf_filler.rs

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Passive Perception Calculation (Priority: P2)

**Goal**: Passive Wisdom (Perception) automatically calculated and displayed in Passive field

**Independent Test**: Generate PDF with Wisdom score and perception proficiency, verify Passive field shows correct calculation

### Implementation for User Story 2

- [x] T008 [P] [US2] Add passive_perception field mapping to src/field_mapper.rs
- [x] T009 [US2] Add passive perception calculation logic to get_field_values() method in src/pdf_filler.rs
- [x] T009a [US2] Add Wisdom score validation (1-30 range) before passive perception calculation in src/pdf_filler.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Hit Dice Field Population (Priority: P3)

**Goal**: Hit dice information displayed in separate HDTotal and HD fields based on class and level

**Independent Test**: Generate PDF with character class and level, verify HDTotal shows level and HD shows correct die type

### Implementation for User Story 3

- [x] T010 [P] [US3] Add hit_dice_total and hit_dice_type field mappings to src/field_mapper.rs
- [x] T011 [US3] Add hit dice calculation logic with class-to-die-type mapping to get_field_values() method in src/pdf_filler.rs

**Checkpoint**: User Stories 1, 2, AND 3 should all work independently

---

## Phase 6: User Story 4 - Currency Field Population (Priority: P4)

**Goal**: Currency amounts (CP, SP, EP, GP, PP) appear in respective PDF fields

**Independent Test**: Generate PDF with currency data, verify each currency field displays correct amount

### Implementation for User Story 4

- [x] T012 [US4] Add currency field population logic to get_field_values() method in src/pdf_filler.rs (currency fields already mapped)

**Checkpoint**: User Stories 1-4 should all work independently

---

## Phase 7: User Story 5 - Features & Traits Field Population (Priority: P5)

**Goal**: Character features and racial traits displayed in Features & Traits field

**Independent Test**: Generate PDF with features and traits data, verify Features & Traits field contains combined text

### Implementation for User Story 5

- [x] T013 [P] [US5] Add features_traits field mapping to src/field_mapper.rs
- [x] T014 [US5] Add features & traits combination logic to get_field_values() method in src/pdf_filler.rs

**Checkpoint**: All 5 user stories should now be independently functional

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T015 [P] Build and test complete implementation with cargo build --release
- [x] T016 [P] Test MCP server with all 5 bug fixes using fill_dnd_character_sheet tool
- [x] T017 Validate all PDF fields populate correctly with comprehensive test character data

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3 → P4 → P5)
- **Polish (Phase 8)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Independent of other stories
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Independent of other stories
- **User Story 4 (P4)**: Can start after Foundational (Phase 2) - Independent of other stories
- **User Story 5 (P5)**: Can start after Foundational (Phase 2) - Independent of other stories

### Within Each User Story

- Field mappings before PDF population logic
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- Field mapping tasks marked [P] can run in parallel within their story

---

## Parallel Example: User Story 2

```bash
# Launch field mapping and calculation logic together:
Task: "Add passive_perception field mapping to src/field_mapper.rs"
Task: "Add passive perception calculation logic to get_field_values() method in src/pdf_filler.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (Personality Traits)
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Add User Story 5 → Test independently → Deploy/Demo
7. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Personality Traits)
   - Developer B: User Story 2 (Passive Perception)
   - Developer C: User Story 3 (Hit Dice)
   - Developer D: User Story 4 (Currency)
   - Developer E: User Story 5 (Features & Traits)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- No tests included per user request (hobby project focus)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Currency field mappings already exist in field_mapper.rs (copper_pieces, silver_pieces, etc.)
- Focus on minimal code changes per constitutional Simplicity First principle
