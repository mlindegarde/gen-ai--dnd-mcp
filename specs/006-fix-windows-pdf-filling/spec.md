# Feature Specification: Fix PDF Filling Issues on Windows

**Feature Branch**: `006-fix-windows-pdf-filling`  
**Created**: 2025-01-20  
**Status**: Draft  
**Input**: User description: "Fix issues with PDF filling on Windows. The issues I am having are that the checkboxes are not being filled in on the PDF, the font size is too large in the PDF, and the debug log file is missing (I think this is because it is hard coded to a path that does not exist on this computer.)"

## Clarifications

### Session 2025-01-20

- Q: Checkbox filling issue → A: Checkboxes are not being marked/checked in the generated PDF on Windows
- Q: Font size issue → A: Text in PDF fields appears too large, likely needs font size specification in appearance stream
- Q: Debug log path → A: Hardcoded path `/Users/lindegar/learningplace/specify--test/mcp_debug.log` in line 15 of `mcp_server.rs` doesn't exist on Windows systems
- Q: Platform-specific issues → A: Focus on cross-platform compatibility for Windows, macOS, and Linux

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Checkbox Marking on Windows (Priority: P0)

When a user generates a D&D character sheet PDF on Windows with proficiencies (saving throws, skills) or prepared spells, the corresponding checkboxes should be visibly marked/checked in the generated PDF.

**Why this priority**: Checkboxes are critical for indicating character proficiencies and spell preparation during gameplay. Without visible checkbox marks, the character sheet is unusable for its primary purpose.

**Independent Test**: Can be fully tested by generating a character sheet with proficiencies on Windows and verifying checkboxes are visually marked in the PDF when opened in any PDF reader.

**Acceptance Scenarios**:

1. **Given** a character with Strength saving throw proficiency on Windows, **When** generating a character sheet PDF, **Then** the Strength saving throw checkbox is visibly checked
2. **Given** a character with Perception skill proficiency on Windows, **When** generating a character sheet PDF, **Then** the Perception skill checkbox is visibly checked
3. **Given** a character with prepared 1st level spells on Windows, **When** generating a character sheet PDF, **Then** the spell preparation checkboxes for those spells are visibly checked
4. **Given** the same character data on macOS/Linux, **When** generating PDFs, **Then** checkboxes remain properly marked (no regression)

---

### User Story 2 - Proper Font Sizing (Priority: P1)

When a user generates a D&D character sheet PDF with text fields, the font size should be appropriate for the field size, allowing text to fit properly without being cut off or appearing oversized.

**Why this priority**: Oversized fonts make character information difficult to read and may cause text truncation, reducing the usability of the character sheet.

**Independent Test**: Can be fully tested by generating a character sheet with various text lengths and verifying font sizes are appropriate for each field type.

**Acceptance Scenarios**:

1. **Given** a character with a name "Thorin Oakenshield", **When** generating a character sheet PDF, **Then** the name fits within the field at a readable font size
2. **Given** a character with multiple spells at each level, **When** generating a character sheet PDF, **Then** spell names appear at appropriate font sizes
3. **Given** a character with long personality traits, **When** generating a character sheet PDF, **Then** the text fits within the field with appropriate font sizing

---

### User Story 3 - Cross-Platform Debug Logging (Priority: P2)

When the MCP server runs on any operating system (Windows, macOS, Linux), debug logs should be written to a location that exists on that system without requiring manual path configuration.

**Why this priority**: Debug logging is essential for troubleshooting issues, but hardcoded paths prevent logging on systems other than the developer's machine.

**Independent Test**: Can be fully tested by running the MCP server on Windows, macOS, and Linux and verifying debug logs are created in an appropriate location for each platform.

**Acceptance Scenarios**:

1. **Given** the MCP server runs on Windows, **When** processing requests, **Then** debug logs are written to a valid Windows path (e.g., current directory or temp directory)
2. **Given** the MCP server runs on macOS, **When** processing requests, **Then** debug logs are written to a valid macOS path
3. **Given** the MCP server runs on Linux, **When** processing requests, **Then** debug logs are written to a valid Linux path
4. **Given** the log directory doesn't exist, **When** the server starts, **Then** the directory is created automatically or logs are written to a fallback location

---

### Edge Cases

- What happens when checkbox appearance streams are missing or malformed in the PDF template?
- How does the system handle PDF viewers that don't support appearance streams?
- What happens when the debug log location is not writable?
- What happens when very long text exceeds field capacity even with reduced font size?

## Requirements *(mandatory)*

### Functional Requirements

#### Checkbox Marking (P0)
- **FR-001**: System MUST set checkbox appearance streams (`/AP`) to properly display checked state on Windows
- **FR-002**: System MUST use checkbox export value or "Yes" value along with appearance stream for checkbox marking
- **FR-003**: System MUST verify checkbox marking works in Adobe Reader, Chrome PDF viewer, and Edge PDF viewer on Windows
- **FR-004**: System MUST maintain checkbox marking compatibility with macOS and Linux PDF viewers

#### Font Sizing (P1)
- **FR-005**: System MUST set default appearance (`/DA`) with appropriate font size for text fields
- **FR-006**: System MUST use font size between 8-12 points for standard text fields
- **FR-007**: System MUST use smaller font sizes (6-8 points) for fields with dense information (e.g., spell lists)
- **FR-008**: System MUST preserve existing font resources in the PDF template
- **FR-009**: System SHOULD dynamically calculate font size based on text length and field dimensions when possible

#### Debug Logging (P2)
- **FR-010**: System MUST use platform-independent path resolution for debug log files
- **FR-011**: System MUST write debug logs to project root directory by default (e.g., `./mcp_debug.log`)
- **FR-012**: System MUST create log directory if it doesn't exist
- **FR-013**: System SHOULD fallback to system temp directory if project root is not writable
- **FR-014**: System MUST handle log file write failures gracefully without crashing

### Non-Functional Requirements

- **NFR-001**: Checkbox marking must work consistently across Windows 10, Windows 11, macOS 12+, and Linux
- **NFR-002**: Font sizing changes must not increase PDF generation time by more than 10%
- **NFR-003**: Debug logging must not impact MCP server performance or throughput

### Key Entities *(include if feature involves data)*

- **PDF Field Object**: Dictionary object containing field properties (`/T`, `/V`, `/AP`, `/DA`)
- **Checkbox Field**: Button field with appearance stream defining checked/unchecked states
- **Appearance Stream (`/AP`)**: Dictionary containing Normal (`/N`) appearances for checked states
- **Default Appearance (`/DA`)**: String defining font, font size, and color for text fields
- **Log File Path**: Cross-platform file system path for debug logging

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of checkbox fields are visually marked on Windows when tested with Adobe Reader, Chrome, and Edge
- **SC-002**: 100% of test character sheets show appropriate font sizing with no text cutoff in critical fields
- **SC-003**: Debug log file is successfully created on Windows, macOS, and Linux test systems
- **SC-004**: All existing tests pass without regression
- **SC-005**: Character sheets generated before and after fix are visually identical except for checkbox visibility and font sizing improvements
- **SC-006**: PDF generation time remains within 10% of current performance

## Technical Approach *(optional)*

### Checkbox Marking Solution

**Problem**: The current implementation only sets the value (`/V`) to "Yes" but doesn't update the appearance stream (`/AP`), causing checkboxes to appear unchecked in some PDF viewers on Windows.

**Solution**: 
1. Update `mark_checkboxes()` to create or update appearance streams for checkbox fields
2. Use `/AS` (Appearance State) to reference the correct appearance
3. Generate ZapfDingbats checkmark appearance if missing
4. Test with multiple PDF readers on Windows

**Code Location**: `src/pdf_filler.rs`, line 169 (`mark_checkboxes` function)

### Font Sizing Solution

**Problem**: Text fields don't specify font size in their default appearance (`/DA`), causing PDF readers to use default (often too large) sizes.

**Solution**:
1. Update `fill_pdf_fields()` to set `/DA` property with font and size
2. Use format: `/Helv 10 Tf 0 g` (Helvetica, 10pt, black)
3. Adjust font size based on field type (standard vs. dense information)
4. Preserve existing font resources from template

**Code Location**: `src/pdf_filler.rs`, line 129 (where `/V` is set)

### Debug Logging Solution

**Problem**: Hardcoded path `/Users/lindegar/learningplace/specify--test/mcp_debug.log` only works on developer's machine.

**Solution**:
1. Replace hardcoded path with relative path to project root
2. Use `std::env::current_dir()` or `std::env::temp_dir()` as fallback
3. Create directories if needed using `std::fs::create_dir_all()`
4. Handle write failures gracefully

**Code Location**: `src/mcp_server.rs`, line 15 (`log_to_file` function)

## Assumptions

- PDF template contains properly structured checkbox and text fields
- PDF template uses standard fonts (Helvetica, Times, etc.)
- Checkbox fields use standard export value "Yes" or similar
- System has write permissions to project directory or temp directory
- PDF viewers follow standard PDF 1.7 specification for form fields
- Performance impact of appearance stream generation is negligible

## Out of Scope

- Custom font embedding or advanced typography
- Dynamic font size calculation based on actual text rendering
- GUI configuration for log file location
- Support for non-standard checkbox export values beyond "Yes"/"Off"
- Advanced appearance customization (colors, styles)
- Multi-platform automated testing infrastructure (manual testing acceptable)

## Dependencies

- `lopdf` crate for PDF manipulation
- Standard library `std::fs` and `std::env` for cross-platform paths
- No new external dependencies required

## Risks

- **Risk**: Appearance stream generation may not work correctly for all checkbox field types
  - **Mitigation**: Test with actual D&D character sheet PDF template; fallback to value-only setting if appearance fails
  
- **Risk**: Font size changes may cause text overflow in some fields
  - **Mitigation**: Use conservative font sizes; add text truncation if needed
  
- **Risk**: Cross-platform path handling may have edge cases
  - **Mitigation**: Use standard library functions; test on multiple platforms; include fallback to temp directory
