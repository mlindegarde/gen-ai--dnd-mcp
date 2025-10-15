# Implementation Plan: Spellcasting Page Header Population

**Branch**: `003-add-spellcasting-page` | **Date**: 2025-10-15 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/003-add-spellcasting-page/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add minimal spellcasting header calculations to existing PDF filler. Use existing character data to populate 4 header fields with simple D&D 5e formulas.

## Technical Context

**Language/Version**: Rust 1.75+  
**Primary Dependencies**: lopdf (PDF manipulation), serde_json (JSON handling)  
**Storage**: N/A (calculated values only, no persistence)  
**Testing**: cargo test  
**Target Platform**: Local filesystem for PDF output files
**Project Type**: single - extends existing D&D character sheet filler  
**Performance Goals**: <1 second for spellcasting header calculations  
**Constraints**: Must integrate with existing PDF field mapping system  
**Scale/Scope**: Single character processing, 4 header fields per character

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Initial Check**: No specific constitutional violations identified. Feature extends existing system without architectural changes.

**Post-Design Check**: ✅ PASSED
- Single module addition maintains simplicity
- Extends existing PDF filling system without breaking changes  
- No new dependencies beyond existing lopdf/serde_json
- Follows established patterns for D&D rule calculations

## Project Structure

### Documentation (this feature)

```
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```
src/
├── character_model.rs   # Existing - Character data structures
├── field_mapper.rs      # MODIFY - Add 4 spellcasting header field mappings
└── pdf_filler.rs        # MODIFY - Add spellcasting header calculations inline

tests/
└── unit/
    └── pdf_filler_tests.rs  # MODIFY - Add spellcasting header test cases
```

**Structure Decision**: Minimal changes to existing files. No new modules needed - add calculations directly to pdf_filler.rs where character data is already available.

## Complexity Tracking

*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
