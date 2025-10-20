# Tasks: Fix PDF Filling Issues on Windows

## Phase 1: Debug Logging Fix ✓

### Task 1.1: Update log_to_file function
- [ ] Replace hardcoded path with `env::current_dir()` approach
- [ ] Add fallback to `env::temp_dir()`  
- [ ] Add error handling for path operations
- [ ] Test log file creation

**File:** `src/mcp_server.rs`
**Lines:** 11-24

### Task 1.2: Test logging
- [ ] Run MCP server and verify log file exists
- [ ] Check log entries are being written
- [ ] Test on Windows (if available)
- [ ] Test fallback with read-only directory

---

## Phase 2: Font Size Fix ✓

### Task 2.1: Create font size helper methods
- [ ] Implement `get_font_size_for_field(&self, field_name: &str) -> u8`
- [ ] Implement `create_default_appearance(&self, field_name: &str) -> String`
- [ ] Define font size mapping rules

**File:** `src/pdf_filler.rs`
**New methods in `PdfFiller` impl**

### Task 2.2: Update fill_pdf_fields
- [ ] Set `/DA` property when setting `/V` for text fields
- [ ] Apply appropriate font size based on field name
- [ ] Test with long text values

**File:** `src/pdf_filler.rs`
**Lines:** 113-140 (fill_pdf_fields method)

### Task 2.3: Test font sizing
- [ ] Generate PDF with character name
- [ ] Generate PDF with spell lists
- [ ] Generate PDF with personality traits
- [ ] Verify no text cutoff
- [ ] Check in multiple PDF viewers

---

## Phase 3: Checkbox Marking Fix ✓

### Task 3.1: Research checkbox structure
- [ ] Run `cargo run --bin extract_fields` to examine checkbox fields
- [ ] Document export values ("Yes", "Off", etc.)
- [ ] Identify appearance stream structure
- [ ] Check if template has `/AP` dictionaries

**Utility:** `src/bin/extract_fields.rs` or `find_all_checkboxes.rs`

### Task 3.2: Update mark_checkboxes function
- [ ] Change `/V` from String to Name object
- [ ] Add `/AS` (Appearance State) setting
- [ ] Use Object::Name for both `/V` and `/AS`
- [ ] Add debug logging for checkbox operations

**File:** `src/pdf_filler.rs`
**Lines:** 169-190 (mark_checkboxes method)

### Task 3.3: Test checkbox visibility
- [ ] Generate PDF with Strength save proficiency
- [ ] Generate PDF with Perception skill proficiency
- [ ] Generate PDF with prepared 1st level spell
- [ ] Test in Adobe Reader (Windows)
- [ ] Test in Chrome PDF viewer
- [ ] Test in Edge PDF viewer
- [ ] Verify no regression on macOS/Linux

---

## Phase 4: Integration & Testing ✓

### Task 4.1: Run automated tests
- [ ] Run `cargo test` - all tests must pass
- [ ] Run `cargo clippy` - no new warnings
- [ ] Fix any test failures

### Task 4.2: Manual testing
- [ ] Run `cargo run test` with sample character
- [ ] Open generated PDF in multiple viewers
- [ ] Visual inspection checklist:
  - [ ] Checkboxes are visible and marked
  - [ ] Font sizes are appropriate
  - [ ] No text cutoff or overflow
  - [ ] Debug log file exists

### Task 4.3: Cross-platform validation
- [ ] Test on Windows (primary target)
- [ ] Test on macOS (regression check)
- [ ] Test on Linux (regression check)
- [ ] Document any platform-specific observations

---

## Phase 5: Documentation & Cleanup ✓

### Task 5.1: Update documentation
- [ ] Update README.md with Windows compatibility notes
- [ ] Document debug log location
- [ ] Add troubleshooting section for PDF viewing issues

### Task 5.2: Code cleanup
- [ ] Remove any debug print statements
- [ ] Ensure consistent code formatting
- [ ] Add code comments where needed
- [ ] Update error messages for clarity

### Task 5.3: Git housekeeping
- [ ] Create feature branch: `006-fix-windows-pdf-filling`
- [ ] Commit changes with descriptive messages
- [ ] Push to remote
- [ ] Create pull request

---

## Checklist Summary

**Total Tasks:** 20
**Estimated Time:** 5-7 hours

**Completion Tracking:**
- [ ] Phase 1 Complete (Debug Logging)
- [ ] Phase 2 Complete (Font Sizing)
- [ ] Phase 3 Complete (Checkbox Marking)
- [ ] Phase 4 Complete (Integration Testing)
- [ ] Phase 5 Complete (Documentation)

---

## Blocked/Waiting

- None currently

---

## Notes

- Checkpoint commits after each phase for easy rollback
- Focus on Phase 1 first to improve debugging for subsequent phases
- Phase 3 (checkboxes) is highest priority but also highest risk
- Test thoroughly on Windows before considering complete
