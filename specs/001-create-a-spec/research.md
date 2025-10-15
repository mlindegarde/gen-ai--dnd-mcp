# Research: MCP Server with PDF Form Filling Tool

**Date**: 2025-10-14  
**Purpose**: Resolve technical unknowns for implementation planning

## Language & Framework Selection

### Decision: Rust with lopdf and pdf-writer crates

**Research Task**: Evaluate Rust PDF manipulation crates for D&D character sheet filling

**Rust PDF Crates Evaluated**:
- `lopdf`: PDF parsing and manipulation, actively maintained, good for form field access
- `pdf-writer`: Low-level PDF creation, excellent for form filling operations
- `printpdf`: High-level PDF creation, less suitable for existing form modification
- `pdf`: Read-only PDF parsing, insufficient for form filling

**Rationale**: 
- Rust `lopdf` + `pdf-writer` combination provides comprehensive PDF form manipulation
- Rust aligns with constitutional requirements (Rust Idiomatic principle)
- Performance benefits for local PDF processing
- Memory safety for PDF parsing operations
- Active maintenance and good documentation

**Alternatives Considered**: 
- Node.js `pdf-lib` has mature form filling but violates Rust preference
- Python `PyPDF2` good for prototyping but not aligned with tech stack

## MCP Protocol Implementation

### Decision: Custom MCP server using tokio and serde_json

**Research Task**: Implement MCP JSON-RPC protocol for D&D character sheet tool

**Implementation Approach**:
- JSON-RPC 2.0 over stdio using `serde_json` for serialization
- `tokio` async runtime for handling MCP client connections
- Custom tool schema validation for D&D character data
- Error handling following MCP specification

**Rationale**:
- MCP protocol is straightforward JSON-RPC, suitable for custom implementation
- Full control over D&D-specific validation and error messages
- Aligns with Simplicity & Maintainability principle
- No external MCP library dependencies needed

## D&D 5e Data Model

### Decision: Comprehensive character model with spell system

**Research Task**: Define complete D&D 5e character data structure

**Core Entities Identified**:
- Character (basic info, class, level, race, background)
- AbilityScores (6 core abilities with derived modifiers)
- Skills (18 skills with proficiency tracking)
- SavingThrows (6 saves with proficiency indicators)
- Combat (AC, HP, initiative, speed)
- Spells (cantrips and leveled spells with prepared state)
- Spellcasting (attack bonus, save DC, spellcasting ability)
- Proficiencies (weapons, armor, languages, tools)
- Equipment (weapons, armor, currency, gear)
- CharacterNarrative (personality, backstory, appearance, traits)

**Validation Rules**:
- Ability scores: 1-20 range
- Proficiency bonus: +2 to +6 based on level
- Spell levels: 0 (cantrips) through 9
- Derived calculations: modifiers, saves, skills, spell bonuses

## PDF Field Mapping Strategy

### Decision: Hardcoded mapping with D&D terminology translation

**Research Task**: Map D&D character data to PDF form fields

**Mapping Categories**:
- Basic character info → PDF header fields
- Ability scores → STR, DEX, CON, INT, WIS, CHA fields
- Skills → Individual skill fields with proficiency circles
- Spells → Cantrip section and spell level sections (1st-9th)
- Narrative → Personality traits, backstory, appearance text areas
- Equipment → Weapon tables and equipment lists

**Field Discovery**:
- Analyze D&D 5e character sheet PDF structure
- Map common D&D terms to exact PDF field names
- Handle proficiency indicators (filled circles)
- Organize spells by level in appropriate PDF sections

**Rationale**:
- Single PDF focus allows optimized hardcoded mapping
- Better user experience with familiar D&D terminology
- Efficient for hobby project scope
- Reduces complexity compared to generic PDF handling

## Spell System Implementation

### Decision: Level-based spell organization with prepared state tracking

**Research Task**: Handle D&D 5e spell system requirements

**Spell Data Structure**:
- Spell name (string)
- Base level (0 for cantrips, 1-9 for leveled spells)
- Prepared state (boolean, defaults to false)
- PDF placement by spell level

**Spellcasting Mechanics**:
- Spell attack bonus = spellcasting ability modifier + proficiency bonus
- Spell save DC = 8 + spellcasting ability modifier + proficiency bonus
- Spellcasting ability determined from input (Wisdom, Intelligence, or Charisma)

**PDF Integration**:
- Cantrips go in dedicated cantrip section
- Leveled spells organized by spell level (1st through 9th)
- Prepared spells marked with filled circles
- Spell attack and save DC calculated and populated

**Rationale**:
- Matches D&D 5e spell system exactly
- Focuses on character preparation, not active play tracking
- Efficient organization for PDF form structure
