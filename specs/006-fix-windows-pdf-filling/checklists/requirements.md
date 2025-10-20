# Requirements Checklist: Fix PDF Filling Issues on Windows

## Functional Requirements

### Checkbox Marking (P0)

- [ ] **FR-001**: System MUST set checkbox appearance streams (`/AP`) to properly display checked state on Windows
- [ ] **FR-002**: System MUST use checkbox export value or "Yes" value along with appearance stream for checkbox marking
- [ ] **FR-003**: System MUST verify checkbox marking works in Adobe Reader, Chrome PDF viewer, and Edge PDF viewer on Windows
- [ ] **FR-004**: System MUST maintain checkbox marking compatibility with macOS and Linux PDF viewers

### Font Sizing (P1)

- [ ] **FR-005**: System MUST set default appearance (`/DA`) with appropriate font size for text fields
- [ ] **FR-006**: System MUST use font size between 8-12 points for standard text fields
- [ ] **FR-007**: System MUST use smaller font sizes (6-8 points) for fields with dense information (e.g., spell lists)
- [ ] **FR-008**: System MUST preserve existing font resources in the PDF template
- [ ] **FR-009**: System SHOULD dynamically calculate font size based on text length and field dimensions when possible

### Debug Logging (P2)

- [ ] **FR-010**: System MUST use platform-independent path resolution for debug log files
- [ ] **FR-011**: System MUST write debug logs to project root directory by default (e.g., `./mcp_debug.log`)
- [ ] **FR-012**: System MUST create log directory if it doesn't exist
- [ ] **FR-013**: System SHOULD fallback to system temp directory if project root is not writable
- [ ] **FR-014**: System MUST handle log file write failures gracefully without crashing

## Non-Functional Requirements

- [ ] **NFR-001**: Checkbox marking must work consistently across Windows 10, Windows 11, macOS 12+, and Linux
- [ ] **NFR-002**: Font sizing changes must not increase PDF generation time by more than 10%
- [ ] **NFR-003**: Debug logging must not impact MCP server performance or throughput

## Success Criteria

- [ ] **SC-001**: 100% of checkbox fields are visually marked on Windows when tested with Adobe Reader, Chrome, and Edge
- [ ] **SC-002**: 100% of test character sheets show appropriate font sizing with no text cutoff in critical fields
- [ ] **SC-003**: Debug log file is successfully created on Windows, macOS, and Linux test systems
- [ ] **SC-004**: All existing tests pass without regression
- [ ] **SC-005**: Character sheets generated before and after fix are visually identical except for checkbox visibility and font sizing improvements
- [ ] **SC-006**: PDF generation time remains within 10% of current performance

## Testing Requirements

### Unit Testing
- [ ] Test log file path resolution on different platforms
- [ ] Test font size calculation for different field types
- [ ] Test checkbox marking logic

### Integration Testing
- [ ] Run full test suite: `cargo test`
- [ ] Run manual test: `cargo run test`
- [ ] Verify output PDF opens in multiple viewers

### Platform Testing
- [ ] Test on Windows 10 or 11
- [ ] Test on macOS (regression)
- [ ] Test on Linux (regression)

### Visual Testing
- [ ] Checkboxes visible in Adobe Reader
- [ ] Checkboxes visible in Chrome PDF viewer
- [ ] Checkboxes visible in Edge PDF viewer
- [ ] Font sizes appropriate across all fields
- [ ] No text cutoff in any field
- [ ] Debug log file exists and contains entries

## Edge Cases Covered

- [ ] Checkbox appearance streams missing in template
- [ ] PDF viewers that don't support appearance streams
- [ ] Debug log directory not writable
- [ ] Very long text exceeding field capacity
- [ ] Read-only project directory
- [ ] Missing font resources in PDF template

## Documentation Requirements

- [ ] Update README.md with Windows compatibility notes
- [ ] Document debug log file location
- [ ] Add troubleshooting section for PDF viewer issues
- [ ] Document font sizing approach
- [ ] Update comments in code where appropriate

## Code Quality Requirements

- [ ] No new `cargo clippy` warnings
- [ ] Code follows Rust formatting standards
- [ ] Error handling is comprehensive
- [ ] Debug statements removed (use proper logging)
- [ ] Comments added where logic is complex

## Acceptance Criteria

**The feature is complete when:**

1. ✅ All functional requirements are implemented
2. ✅ All success criteria are met
3. ✅ All tests pass on all platforms
4. ✅ Visual inspection confirms fixes work
5. ✅ Documentation is updated
6. ✅ Code is reviewed and cleaned up
7. ✅ Changes are committed to feature branch

---

## Verification Steps

**After implementation, verify each item:**

```bash
# 1. Build without errors
cargo build

# 2. Run tests
cargo test

# 3. Check code quality
cargo clippy

# 4. Generate test PDF
cargo run test

# 5. Verify outputs
- [ ] ./mcp_debug.log exists
- [ ] filled_character_sheet.pdf has visible checkboxes
- [ ] filled_character_sheet.pdf has appropriate font sizes

# 6. Cross-platform check
- [ ] Test on Windows
- [ ] Test on macOS
- [ ] Test on Linux
```

---

## Sign-off

- [ ] Developer: Implementation complete
- [ ] Tester: Manual testing complete
- [ ] Reviewer: Code review complete
- [ ] Ready to merge: All requirements met
