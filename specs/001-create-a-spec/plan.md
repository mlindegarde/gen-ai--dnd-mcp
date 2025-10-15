# Implementation Plan: MCP Server with PDF Form Filling Tool

**Branch**: `001-create-a-spec` | **Date**: 2025-10-14 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-create-a-spec/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

MCP server with D&D 5e character sheet PDF filling tool. Accepts nested JSON character data including spells, proficiencies, and narrative fields. Validates against D&D rules, maps to PDF fields, auto-calculates derived values, and returns completed character sheet PDF with proper spell organization and proficiency indicators.

## Technical Context

**Language/Version**: Rust 1.75+  
**Primary Dependencies**: lopdf (PDF manipulation), pdf-writer (PDF creation), serde_json (JSON handling), tokio (async runtime)  
**Storage**: Files (local PDF templates and character data validation rules)  
**Testing**: cargo test (unit and integration tests)  
**Target Platform**: Local development environment (macOS/Linux/Windows)
**Project Type**: Single project (MCP server with tool)  
**Performance Goals**: <2 seconds response time for PDF forms under 5MB  
**Constraints**: <10MB PDF file size limit, local-only operation, D&D 5e rule compliance  
**Scale/Scope**: Single-user hobby tool, complete D&D 5e character sheet with spells and narrative

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

✅ **Local-First**: PDF processing runs locally, no cloud dependencies  
✅ **Hobby-Focused**: D&D character sheet filling serves genuine hobby use case  
✅ **Rust Idiomatic**: Using lopdf/pdf-writer crates with proper Rust patterns  
✅ **MCP Protocol Compliance**: Custom MCP implementation following JSON-RPC specification  
✅ **Simplicity & Maintainability**: Single binary with focused PDF filling responsibility

**Status**: PASS - All constitutional principles satisfied after design phase

## Project Structure

### Documentation (this feature)

```
specs/001-create-a-spec/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   └── mcp-tool-schema.json
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```
src/
├── main.rs              # MCP server entry point
├── mcp_server.rs        # MCP protocol implementation  
├── pdf_filler.rs        # PDF form filling logic
├── dnd_validator.rs     # D&D 5e rule validation
├── field_mapper.rs      # JSON to PDF field mapping
├── character_model.rs   # Character data structures
├── spell_system.rs      # Spell organization and validation
└── narrative_handler.rs # Character narrative processing

docs/
└── 5E_CharacterSheet_Fillable.pdf  # D&D character sheet template

tests/
├── integration/         # MCP protocol compliance tests
├── unit/               # Component unit tests
└── fixtures/           # Sample character data and PDFs

Cargo.toml              # Rust dependencies and metadata
```

**Structure Decision**: Single Rust project structure selected. The MCP server, PDF processing, D&D validation, spell system, and narrative handling are tightly coupled components that benefit from being in a single binary. This aligns with the Simplicity & Maintainability constitutional principle while supporting the complete D&D 5e character sheet feature set.

## Complexity Tracking

*No constitutional violations - this section intentionally left empty*

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
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure: feature modules, UI flows, platform tests]
```

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## Complexity Tracking

*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
