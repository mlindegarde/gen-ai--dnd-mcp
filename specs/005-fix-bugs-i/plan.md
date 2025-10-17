# Implementation Plan: Character Sheet Field Population Bug Fixes

**Branch**: `005-fix-bugs-i` | **Date**: 2025-10-17 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/005-fix-bugs-i/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Fix five critical PDF field population bugs: personality traits not displaying, passive Wisdom (Perception) not calculating, hit dice not showing, currency fields (CP, SP, EP, GP, PP) not populating, and Features & Traits field not displaying. Technical approach involves updating field mapping system and adding missing calculation logic within existing PDF filler architecture.

## Technical Context

**Language/Version**: Rust 1.75+  
**Primary Dependencies**: lopdf (PDF manipulation), serde_json (JSON handling), pdf-writer (PDF creation)  
**Storage**: Local filesystem for PDF output files  
**Testing**: cargo test (unit tests), integration tests for PDF generation  
**Target Platform**: Local desktop (cross-platform via Rust)  
**Project Type**: Single project (library + MCP server binary)  
**Performance Goals**: PDF processing under 2 seconds for files under 5MB  
**Constraints**: Local-only processing, memory usage under 100MB, embedded PDF template  
**Scale/Scope**: Single-user tool, ~1000 lines of existing code, 5 specific bug fixes

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

✅ **I. Local-First Processing**: Bug fixes maintain local PDF processing, no cloud dependencies  
✅ **II. Minimal Dependencies**: Uses existing lopdf/serde_json, no new dependencies required  
✅ **III. D&D 5e Rule Accuracy**: Passive Perception and Hit Dice follow official D&D 5e rules  
✅ **IV. Rust Idiomatic Code**: Bug fixes will follow existing code patterns and error handling  
✅ **V. Simplicity First**: Minimal changes to existing field mapping and calculation logic  
✅ **VI. Single Responsibility**: Changes isolated to appropriate modules (field_mapper, pdf_filler)  
✅ **VII. Scope Limitations**: Explicitly excludes multi-classing and expert skills per constitution

**Gate Status**: ✅ PASSED - All constitutional principles satisfied

**Post-Design Re-evaluation**: ✅ CONFIRMED - Design artifacts maintain constitutional compliance:
- Research.md confirms no new dependencies or complexity
- Data-model.md shows single-class only approach  
- Contracts maintain existing MCP tool interface
- Quickstart demonstrates local-only processing
- Features & Traits addition follows same simple field mapping pattern

## Project Structure

### Documentation (this feature)

```
specs/005-fix-bugs-i/
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
├── character_model.rs   # Existing - may need hit dice calculation
├── field_mapper.rs      # PRIMARY - add missing field mappings (including Features & Traits)
├── pdf_filler.rs        # PRIMARY - add personality traits, passive perception, currency, features & traits logic
├── narrative_handler.rs # Existing - personality traits handling
├── spell_system.rs      # Existing - no changes needed
├── dnd_validator.rs     # Existing - no changes needed
├── errors.rs           # Existing - no changes needed
└── mcp_server.rs       # Existing - no changes needed

tests/
├── integration/        # Add tests for bug fixes
├── unit/              # Add tests for new calculations
└── fixtures/          # Test character data with all fields
```

**Structure Decision**: Single project structure maintained. Changes focused on field_mapper.rs and pdf_filler.rs modules to add missing field mappings and calculation logic. Features & Traits field follows same pattern as personality traits. No new modules needed per Simplicity First principle.

## Complexity Tracking

*No constitutional violations - this section not needed*
