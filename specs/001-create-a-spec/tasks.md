# Tasks: MCP Server with PDF Form Filling Tool

**Input**: Design documents from `/specs/001-create-a-spec/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are NOT explicitly requested in the specification, so test tasks are omitted.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root
- Paths assume single Rust project structure per plan.md

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 ✅ Create Rust project structure with Cargo.toml and src/ directory
- [x] T002 ✅ Add dependencies to Cargo.toml: lopdf, pdf-writer, serde_json, tokio
- [x] T003 ✅ [P] Create docs/ directory and place D&D 5e character sheet PDF at docs/5E_CharacterSheet_Fillable.pdf
- [x] T004 ✅ [P] Create tests/ directory structure with integration/ and unit/ subdirectories
- [x] T005 ✅ [P] Create tests/fixtures/ directory for sample character data and PDFs
- [x] T006 ✅ [P] Configure Rust formatting and linting in Cargo.toml

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T007 ✅ COMPLETE - PDF field analysis exists at docs/pdf-field-analysis.md
- [x] T008 ✅ COMPLETE - D&D rules documented at docs/dnd-rules.md  
- [x] T009 ✅ COMPLETE - MCP protocol examples at docs/mcp-protocol-examples.md
- [x] T009b ✅ COMPLETE - Sample test data created at docs/sample-data.md
- [x] T010 ✅ Create character_model.rs with core D&D 5e data structures (Character, AbilityScores, Skills, Combat, Spells, Proficiencies, Equipment, CharacterNarrative)
- [x] T011 ✅ Implement D&D 5e validation rules in dnd_validator.rs (reference docs/dnd-rules.md for exact formulas)
- [x] T012 ✅ Create field_mapper.rs with hardcoded mapping from D&D terms to PDF field names (reference docs/pdf-field-analysis.md)
- [x] T013 ✅ [P] Implement comprehensive error types and messages in src/errors.rs (reference docs/dnd-rules.md for error messages)
- [x] T014 ✅ [P] Create spell_system.rs for spell organization by level (cantrips through 9th level)
- [x] T015 ✅ [P] Create narrative_handler.rs for character personality and backstory fields
- [x] T016 ✅ Create validation rule test cases in tests/fixtures/validation-test-cases.json

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

**📋 Gap Analysis Status**: All critical specification gaps have been addressed:
- ✅ PDF field mappings documented with exact field names
- ✅ D&D rule formulas specified with precise calculations  
- ✅ MCP protocol examples with concrete JSON-RPC messages
- ✅ Sample test data covering edge cases and validation scenarios

---

## Phase 3: User Story 1 - Basic PDF Form Population (Priority: P1) 🎯 MVP

**Goal**: Core PDF filling functionality that accepts D&D character data and generates filled character sheet

**Independent Test**: Provide sample D&D character JSON data and verify output PDF contains correct values in correct fields

### Implementation for User Story 1

- [x] T013 ✅ [P] [US1] Implement PDF parsing and field discovery in pdf_filler.rs
- [x] T014 ✅ [P] [US1] Add ability score modifier calculations to character_model.rs
- [x] T015 ✅ [US1] Implement basic PDF form filling logic in pdf_filler.rs (depends on T013)
- [x] T016 ✅ [US1] Add D&D rule validation integration in pdf_filler.rs (references docs/dnd-rules.md)
- [x] T017 ✅ [US1] Implement field mapping from JSON to PDF fields in pdf_filler.rs (references docs/pdf-field-analysis.md)
- [x] T018 ✅ [US1] Add spell organization and placement by level in pdf_filler.rs (depends on T011)
- [x] T019 ✅ [US1] Add character narrative field population in pdf_filler.rs (depends on T012)
- [x] T020 ✅ [US1] Implement proficiency indicator marking (circles) for skills and saves in pdf_filler.rs
- [x] T021 ✅ [US1] Add error handling and validation override functionality in pdf_filler.rs
- [x] T022 ✅ [US1] Create main.rs with basic CLI interface for testing PDF filling functionality

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - MCP Server Integration (Priority: P2)

**Goal**: MCP protocol server that exposes PDF filling tool to AI assistants and other MCP clients

**Independent Test**: Connect MCP client to server and successfully invoke PDF filling tool through protocol

### Implementation for User Story 2

- [x] T023 ✅ [P] [US2] Create mcp_server.rs with JSON-RPC 2.0 protocol implementation using tokio
- [x] T024 ✅ [P] [US2] Implement MCP tool schema definition based on contracts/mcp-tool-schema.json
- [x] T025 ✅ [US2] Add MCP server initialization and client connection handling in mcp_server.rs
- [x] T026 ✅ [US2] Integrate PDF filling functionality with MCP tool interface in mcp_server.rs (depends on T015-T021)
- [x] T027 ✅ [US2] Implement MCP error response formatting following protocol standards in mcp_server.rs
- [x] T028 ✅ [US2] Add input parameter validation for MCP tool calls in mcp_server.rs
- [x] T029 ✅ [US2] Update main.rs to launch MCP server instead of CLI interface
- [x] T030 ✅ [US2] Add proper JSON serialization/deserialization for character data in mcp_server.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Field Mapping and Validation (Priority: P3)

**Goal**: Enhanced usability through field discovery and detailed validation feedback

**Independent Test**: Analyze PDF form and return complete list of fillable fields with properties and constraints

### Implementation for User Story 3

- [ ] T031 [P] [US3] Extend pdf_filler.rs with comprehensive field discovery functionality
- [ ] T032 [P] [US3] Add detailed field constraint analysis in pdf_filler.rs
- [ ] T033 [US3] Implement hierarchical field structure representation in pdf_filler.rs
- [ ] T034 [US3] Add enhanced validation error reporting with specific field details in dnd_validator.rs
- [ ] T035 [US3] Create field discovery MCP tool interface in mcp_server.rs
- [ ] T036 [US3] Add field constraint validation before PDF processing in pdf_filler.rs
- [ ] T037 [US3] Implement detailed error messages for field mapping failures in field_mapper.rs

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T038 [P] Add comprehensive logging throughout all modules using log crate
- [ ] T039 [P] Create sample character data files in tests/fixtures/ for testing
- [ ] T040 [P] Add performance optimization for large PDF processing
- [ ] T041 [P] Implement proper Unicode and special character handling
- [ ] T042 [P] Add file size validation and limits (10MB max) in pdf_filler.rs
- [ ] T043 [P] Create comprehensive error types and error handling in src/lib.rs
- [ ] T044 Run quickstart.md validation with sample data
- [ ] T045 Add Rust documentation comments to all public functions and structs

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Integrates with US1 but should be independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Extends US1 functionality but should be independently testable

### Within Each User Story

- Models and core logic before integration
- Core implementation before MCP integration
- Basic functionality before enhanced features
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- Models and independent modules within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch independent modules for User Story 1 together:
Task: "Implement PDF parsing and field discovery in pdf_filler.rs"
Task: "Add ability score modifier calculations to character_model.rs"

# After core modules are ready, integrate:
Task: "Implement basic PDF form filling logic in pdf_filler.rs"
Task: "Add D&D rule validation integration in pdf_filler.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently with sample D&D character data
5. Deploy/demo basic PDF filling functionality

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo (MCP integration)
4. Add User Story 3 → Test independently → Deploy/Demo (Enhanced validation)
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Core PDF filling)
   - Developer B: User Story 2 (MCP server integration)
   - Developer C: User Story 3 (Field discovery and validation)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Focus on D&D 5e character sheet as the primary use case
- Spell system includes cantrips (level 0) through 9th level spells
- Character narrative includes personality traits, backstory, and appearance
- Proficiency indicators use filled circles in PDF form
- MCP protocol follows JSON-RPC 2.0 specification
- All PDF processing runs locally without external dependencies
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
