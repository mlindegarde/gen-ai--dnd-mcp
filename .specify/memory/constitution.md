<!--
Sync Impact Report:
- Version change: 1.3.0 → 1.4.0
- Modified principles: Testing Requirements → Testing Philosophy (hobby project, testing optional)
- Modified principles: Development Workflow (removed test-first requirement)
- Added sections: None
- Removed sections: None
- Templates requiring updates: ✅ All templates reviewed - testing sections are optional
- Follow-up TODOs: None
-->

# D&D Character Sheet Filler Constitution

## Core Principles

### I. Local-First Processing
All PDF processing MUST run locally without cloud dependencies. Character data MUST remain on user's machine. No external API calls for core functionality.

### II. Minimal Dependencies
Use only essential dependencies: lopdf for PDF manipulation, serde_json for JSON handling. Avoid heavy frameworks or unnecessary complexity.

### III. D&D 5e Rule Accuracy
All calculations MUST follow official D&D 5e rules exactly. Spell save DC = 8 + proficiency + modifier. Proficiency bonus by level table. No house rules or approximations.

### IV. Rust Idiomatic Code
Follow standard Rust conventions: snake_case, proper error handling with Result types, no unwrap() in production code, comprehensive unit tests.

### V. Simplicity First
When adding new features, MUST take the simplest approach that works. Prefer inline code over new modules. Avoid abstractions until proven necessary. No over-engineering for this simple project.

### VI. Single Responsibility
Each module has one clear purpose. PDF filling, field mapping, rule validation are separate concerns. No god objects or mixed responsibilities.

### VII. Scope Limitations
The following advanced D&D features are explicitly OUT OF SCOPE to maintain tool simplicity:
- Multi-classing support (characters with multiple classes)
- Expert skill tracking (double proficiency bonus mechanics)
- Advanced character options beyond basic single-class progression

Only basic single-class character mechanics are supported. This limitation ensures focused development and prevents feature creep that would complicate the core PDF filling functionality.

## Quality Standards

### Testing Philosophy (Hobby Project)
This is a hobby project where formal testing is **OPTIONAL**. Developers should:
- Focus on manual testing and verification
- Run the program to validate changes work as expected
- Skip unit/integration tests unless they personally find them helpful
- Prioritize working code over test coverage

Automated tests may exist but are not mandatory for feature completion.

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
1. Implement minimal viable functionality
2. Manual testing and verification
3. Integrate with existing systems
4. Add error handling as needed

### Code Review Standards
- All changes should maintain backward compatibility when practical
- Manual verification that features work as intended
- Document any known limitations

## Governance

This constitution supersedes all other practices. Changes require explicit documentation and migration plan. All implementations must verify compliance with these principles.

**Version**: 1.4.0 | **Ratified**: 2025-10-15 | **Last Amended**: 2025-10-20
