# Research: Spellcasting Page Header Population

**Feature**: 003-add-spellcasting-page  
**Date**: 2025-10-15

## Technical Decisions

### Decision: Inline Calculations in PDF Filler
**Rationale**: Character data already available in pdf_filler.rs. Simple calculations (4 lines) don't warrant separate module.

**Alternatives considered**: 
- Separate spellcasting module - rejected due to over-engineering
- Complex class hierarchy - rejected due to unnecessary complexity

### Decision: Hardcoded Class-to-Ability Mapping
**Rationale**: D&D 5e spellcasting abilities are fixed rules. Simple match statement is sufficient.

**Alternatives considered**:
- Enum with methods - rejected due to added complexity
- External configuration - rejected due to overkill for 9 classes

## Implementation Approach

1. Add 4 field mappings to field_mapper.rs
2. Add simple spellcasting calculations to pdf_filler.rs (20 lines max)
3. Add test cases to existing test file

**Total Code**: ~30 lines across 2 existing files
