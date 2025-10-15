# Tasks: PDF File Saving Fix

**Input**: Design documents from `/specs/002-my-program-no/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are not explicitly requested in the feature specification, so test tasks are omitted.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root
- Paths assume single project structure per plan.md

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Verify existing project structure matches plan.md requirements
- [x] T002 [P] Update Cargo.toml dependencies for enhanced error handling
- [x] T003 [P] Configure cargo clippy and fmt for code quality

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Create enhanced error types in src/lib/errors.rs
- [x] T005 [P] Implement SaveError enum with specific error variants
- [x] T006 [P] Create OutputPathConfig struct in src/models/path_config.rs
- [x] T007 [P] Create SaveOperationResult struct in src/models/save_result.rs
- [x] T008 Implement path validation utilities in src/lib/path_validator.rs
- [x] T009 [P] Add filesystem permission checking functions
- [x] T010 [P] Add disk space validation functions

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Successful PDF File Output (Priority: P1) 🎯 MVP

**Goal**: Fix core PDF saving functionality with reliable file output and proper validation

**Independent Test**: Run character sheet filler with valid character data and verify properly formatted PDF file is created at expected location

### Implementation for User Story 1

- [ ] T011 [P] [US1] Enhance pdf_filler.rs with pre-flight path validation
- [ ] T012 [P] [US1] Implement directory auto-creation logic in src/services/pdf_filler.rs
- [ ] T013 [US1] Add PDF integrity verification after file write operations
- [ ] T014 [US1] Update MCP tool handler to use enhanced error types
- [ ] T015 [US1] Implement atomic file write operations for PDF output
- [ ] T016 [US1] Add file size and format validation post-write
- [ ] T017 [US1] Update success response with file metadata

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Error Handling and Diagnostics (Priority: P2)

**Goal**: Provide clear, actionable error messages when PDF saving fails

**Independent Test**: Simulate various failure conditions and verify appropriate error messages are displayed

### Implementation for User Story 2

- [ ] T018 [P] [US2] Create user-friendly error message mapping in src/lib/error_messages.rs
- [ ] T019 [P] [US2] Implement specific error handlers for each SaveError variant
- [ ] T020 [US2] Add error suggestion generation based on error type
- [ ] T021 [US2] Update MCP error responses with detailed diagnostics
- [ ] T022 [US2] Add error logging for troubleshooting support
- [ ] T023 [US2] Implement error recovery suggestions for common issues

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - File Overwrite Protection (Priority: P3)

**Goal**: Handle file conflicts gracefully with auto-overwrite behavior

**Independent Test**: Create existing files at target locations and verify system overwrites them consistently

### Implementation for User Story 3

- [ ] T024 [P] [US3] Implement file existence checking in path validation
- [ ] T025 [P] [US3] Add overwrite confirmation logic to pdf_filler.rs
- [ ] T026 [US3] Update file write operations to handle existing files
- [ ] T027 [US3] Add logging for file overwrite operations
- [ ] T028 [US3] Ensure consistent behavior across multiple save operations

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T029 [P] Update documentation in README.md with new error handling
- [ ] T030 [P] Add performance monitoring for 2-second save target
- [ ] T031 Code cleanup and refactoring across PDF pipeline
- [ ] T032 [P] Validate quickstart.md implementation guide
- [ ] T033 Run cargo clippy and fix any warnings
- [ ] T034 [P] Update MCP tool schema documentation

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
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Integrates with US1 error handling but independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Uses US1 file operations but independently testable

### Within Each User Story

- Path validation before file operations
- Error handling before user feedback
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- Models and utilities within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch parallel tasks for User Story 1:
Task: "Enhance pdf_filler.rs with pre-flight path validation"
Task: "Implement directory auto-creation logic in src/services/pdf_filler.rs"

# Then sequential tasks:
Task: "Add PDF integrity verification after file write operations"
Task: "Update MCP tool handler to use enhanced error types"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Core PDF saving)
   - Developer B: User Story 2 (Error handling)
   - Developer C: User Story 3 (File overwrite)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Focus on existing pdf_filler.rs module as primary implementation target
- Maintain MCP protocol compliance throughout
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
