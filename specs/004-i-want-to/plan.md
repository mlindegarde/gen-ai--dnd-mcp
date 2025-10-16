# Implementation Plan: Character Sheet PDF Population (Saves, Skills, Spells)

**Branch**: `004-i-want-to` | **Date**: 2025-10-16 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/004-i-want-to/spec.md`

## Summary

Extend the existing D&D character sheet PDF filler to populate saving throw and skills sections on page 1, plus spell preparation checkboxes. Calculate bonuses using ability modifiers and proficiency rules (excluding expertise), mark visual indicators in PDF form fields using systematically verified checkbox mappings, and handle errors gracefully while maintaining D&D 5e rule accuracy.

## Technical Context

**Language/Version**: Rust 1.75+  
**Primary Dependencies**: lopdf (PDF manipulation), serde_json (JSON handling)  
**Storage**: N/A (calculated values only, no persistence)  
**Testing**: cargo test  
**Target Platform**: Local filesystem for PDF output files
**Project Type**: single (command-line tool with MCP server)  
**Performance Goals**: PDF processing under 1 second for character data  
**Constraints**: <2 seconds for PDFs under 5MB, <100MB memory usage  
**Scale/Scope**: Single character processing, 6 saving throws + 18 skills + spell preparation checkboxes per character

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Initial Check (Pre-Phase 0)
✅ **Local-First Processing**: All calculations performed locally, no cloud dependencies  
✅ **Minimal Dependencies**: Uses existing lopdf and serde_json, no new heavy dependencies  
✅ **D&D 5e Rule Accuracy**: Implements official proficiency bonus table and ability modifier calculations  
✅ **Rust Idiomatic Code**: Extends existing codebase following established patterns  
✅ **Simplicity First**: Adds focused functionality to existing PDF filling system  
✅ **Single Responsibility**: Extends existing field mapping and calculation modules  

**Performance Standards**: ✅ Under 1 second target aligns with <2 second constitution requirement  
**Error Handling**: ✅ Graceful degradation specified for invalid data  
**Testing Requirements**: ✅ Unit tests required for all calculation functions  

### Post-Design Check (After Phase 1)
✅ **Local-First Processing**: Design maintains local-only calculations, no external dependencies introduced  
✅ **Minimal Dependencies**: No new dependencies added beyond existing lopdf/serde_json  
✅ **D&D 5e Rule Accuracy**: Calculation API enforces exact D&D 5e formulas and progression tables  
✅ **Rust Idiomatic Code**: API design uses Result types, proper error handling, follows naming conventions  
✅ **Simplicity First**: Extends existing modules rather than creating new abstractions  
✅ **Single Responsibility**: Each calculation function has single, clear purpose  

**All Constitution requirements maintained through design phase.**

## Project Structure

### Documentation (this feature)

```
specs/004-i-want-to/
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
├── character_model.rs   # Extend with saving throw and skill data structures
├── dnd_validator.rs     # Add saving throw and skill validation rules
├── field_mapper.rs      # Add PDF field mappings for saves/skills/spells (UPDATED with verified checkbox mappings)
├── pdf_filler.rs        # Extend with checkbox/circle field filling
├── spell_system.rs      # (existing, no changes needed)
├── narrative_handler.rs # (existing, no changes needed)
├── mcp_server.rs        # (existing, no changes needed)
└── errors.rs            # Extend with saves/skills error types

tests/
├── contract/            # MCP tool contract tests
├── integration/         # End-to-end PDF filling tests
└── unit/                # Calculation and validation tests
```

**Structure Decision**: Single project structure maintained, extending existing modules rather than creating new ones to follow Simplicity First principle. The field_mapper.rs has been updated with systematically verified checkbox mappings for spell preparation.

## Complexity Tracking

*No Constitution violations - all requirements align with established principles.*
