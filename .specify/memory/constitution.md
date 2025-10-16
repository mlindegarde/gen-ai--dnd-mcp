# D&D Character Sheet Filler Constitution

## Core Principles

### I. Local-First Processing
All PDF processing MUST run locally without cloud dependencies. Character data MUST remain on user's machine. No external API calls for core functionality.

### II. Minimal Dependencies
Use only essential dependencies: lopdf for PDF manipulation, serde_json for JSON handling. Avoid heavy frameworks or unnecessary complexity.

### III. D&D 5e Rule Accuracy
All calculations MUST follow official D&D 5e rules exactly. Spell save DC = 8 + proficiency + modifier. Proficiency bonus by level table. No house rules or approximations.

### VII. Scope Limitations
Expertise (double proficiency bonus) is explicitly OUT OF SCOPE for this tool. Only basic proficiency bonuses are supported to maintain simplicity.

### IV. Rust Idiomatic Code
Follow standard Rust conventions: snake_case, proper error handling with Result types, no unwrap() in production code, comprehensive unit tests.

### V. Simplicity First
When adding new features, MUST take the simplest approach that works. Prefer inline code over new modules. Avoid abstractions until proven necessary. No over-engineering for this simple project.

### VI. Single Responsibility
Each module has one clear purpose. PDF filling, field mapping, rule validation are separate concerns. No god objects or mixed responsibilities.

## Quality Standards

### Testing Requirements
- Unit tests for all calculation functions
- Integration tests for PDF filling workflow  
- Test coverage for edge cases (invalid classes, extreme ability scores)

### Performance Standards
- PDF processing under 2 seconds for files under 5MB
- Memory usage under 100MB during processing
- No blocking operations on main thread

### Error Handling
- Graceful degradation for invalid character data
- Clear error messages for user-facing issues
- Validation before PDF modification attempts

## Development Workflow

### Implementation Order
1. Tests first for new calculations
2. Implement minimal viable functionality
3. Integrate with existing systems
4. Add comprehensive error handling

### Code Review Requirements
- All changes must maintain backward compatibility
- New features require corresponding tests
- Performance impact must be measured

## Governance

This constitution supersedes all other practices. Changes require explicit documentation and migration plan. All implementations must verify compliance with these principles.

**Version**: 1.2.0 | **Ratified**: 2025-10-15 | **Last Amended**: 2025-10-16