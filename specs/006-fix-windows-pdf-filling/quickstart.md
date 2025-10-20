# Quickstart: Fix PDF Filling Issues on Windows

## Problem Statement

Three issues prevent proper PDF character sheet generation on Windows:
1. **Checkboxes not visible** - Proficiency and spell preparation checkboxes don't appear checked
2. **Font too large** - Text appears oversized in PDF fields
3. **Debug log missing** - Hardcoded path prevents log file creation on Windows

## Quick Start Guide

### Prerequisites
- Rust 1.75+
- Windows 10/11, macOS, or Linux
- D&D 5e character sheet PDF template

### Running the Fix

```bash
# 1. Create feature branch
git checkout -b 006-fix-windows-pdf-filling

# 2. Make the changes (see Implementation section)

# 3. Test the changes
cargo test

# 4. Generate a test PDF
cargo run test

# 5. Check the output
# - Open filled_character_sheet.pdf
# - Verify checkboxes are visible
# - Verify font sizes look appropriate
# - Check ./mcp_debug.log exists
```

### Testing Checklist

**Debug Logging:**
- [ ] File `./mcp_debug.log` exists in project root
- [ ] Log entries have timestamps
- [ ] Log shows server startup and processing messages

**Font Sizing:**
- [ ] Character name is readable (12pt)
- [ ] Spell names fit in fields (8pt)
- [ ] No text appears cut off

**Checkbox Marking:**
- [ ] Saving throw proficiency checkboxes are visible
- [ ] Skill proficiency checkboxes are visible
- [ ] Spell preparation checkboxes are visible

### Implementation Overview

**File 1: `src/mcp_server.rs` (Lines 11-24)**
```rust
fn log_to_file(message: &str) {
    use std::env;
    use std::path::PathBuf;
    
    let log_path: PathBuf = env::current_dir()
        .map(|d| d.join("mcp_debug.log"))
        .unwrap_or_else(|_| env::temp_dir().join("dnd-mcp-debug.log"));
    
    // ... rest of function
}
```

**File 2: `src/pdf_filler.rs` (fill_pdf_fields method)**
```rust
// Add font size when setting field value
new_dict.set(b"V", Object::String(value.as_bytes().to_vec(), lopdf::StringFormat::Literal));
let font_size = if field_name.contains("Name") { 12 } else { 10 };
let da_string = format!("/Helv {} Tf 0 g", font_size);
new_dict.set(b"DA", Object::String(da_string.as_bytes().to_vec(), lopdf::StringFormat::Literal));
```

**File 3: `src/pdf_filler.rs` (mark_checkboxes method)**
```rust
// Change from String to Name objects
new_dict.set(b"V", Object::Name(b"Yes".to_vec()));
new_dict.set(b"AS", Object::Name(b"Yes".to_vec()));
```

### Common Issues

**Issue: Log file still not created**
- Check current working directory with `pwd` or `cd`
- Verify write permissions
- Check temp directory fallback: `/tmp/dnd-mcp-debug.log` (Linux/Mac) or `%TEMP%\dnd-mcp-debug.log` (Windows)

**Issue: Checkboxes still not visible**
- Open PDF in Adobe Reader (most compatible)
- Check that test character has proficiencies set
- Verify checkbox field names match between code and PDF template

**Issue: Font still too large**
- Check PDF template supports `/DA` property
- Try different font sizes (8-12 range)
- Verify font resource exists in PDF template

### Next Steps

After implementing the fixes:
1. Test on target platform (Windows)
2. Test on other platforms for regression
3. Run full test suite: `cargo test`
4. Generate multiple test characters
5. Review in different PDF viewers

### Documentation

- Full spec: `specs/006-fix-windows-pdf-filling/spec.md`
- Research: `specs/006-fix-windows-pdf-filling/research.md`
- Implementation plan: `specs/006-fix-windows-pdf-filling/plan.md`
- Tasks: `specs/006-fix-windows-pdf-filling/tasks.md`

### Success Criteria

✅ Debug log file created on all platforms
✅ Font sizes appropriate and readable
✅ Checkboxes visible in Adobe Reader, Chrome, Edge
✅ All existing tests pass
✅ No regressions on macOS/Linux
