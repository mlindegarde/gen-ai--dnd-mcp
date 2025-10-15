# Implementation Plan: Add Spell Slot Tracking

**Branch**: `002-add-spell-slot` | **Date**: 2025-10-15 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/002-add-spell-slot/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add display-only spell slot tracking to D&D 5e character sheets. Calculate maximum spell slots based on character class and level according to D&D 5e rules for single-class characters. Display spell slot counts for levels 1-9 on PDF character sheets, hiding the section for non-spellcasting characters.

## Technical Context

**Language/Version**: Rust 1.75+  
**Primary Dependencies**: lopdf (PDF manipulation), serde_json (JSON handling)  
**Storage**: N/A (calculated values only, no persistence)  
**Testing**: cargo test  
**Target Platform**: Local binary (MCP server)
**Project Type**: Single project (extends existing D&D character sheet filler)  
**Performance Goals**: Calculate spell slots in <100ms  
**Constraints**: Must integrate with existing PDF field mapping system  
**Scale/Scope**: Support all official D&D 5e single-class characters

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Initial Check**: ✅ PASS - No constitution file found, proceeding with standard practices  
**Post-Design Check**: ✅ PASS - Design maintains simplicity and extends existing architecture without violations

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
├── character_model.rs   # Existing - may need spell slot fields
├── field_mapper.rs      # Existing - add spell slot PDF field mappings
├── pdf_filler.rs        # Existing - add spell slot calculation logic
├── spell_system.rs      # Existing - extend for spell slot calculations
└── dnd_validator.rs     # Existing - may need spell slot validation

tests/
├── fixtures/
│   └── spell-slot-characters.json  # New test data
└── integration/
    └── spell_slot_tests.rs         # New tests
```

**Structure Decision**: Extend existing single project structure. Spell slot calculation integrates into existing `calculate_derived_values()` method in `pdf_filler.rs`. New spell slot logic added to `spell_system.rs`. Minimal changes to existing codebase.

## Complexity Tracking

*No violations detected - complexity tracking not required*
