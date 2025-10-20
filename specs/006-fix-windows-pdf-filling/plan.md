# Implementation Plan: Fix PDF Filling Issues on Windows

## Overview

This plan addresses three critical issues affecting PDF generation on Windows:
1. Checkboxes not being visibly marked
2. Font sizes being too large 
3. Debug log file path being hardcoded to non-existent location

## Phase 1: Debug Logging Fix (Lowest Risk)

**Estimated Time:** 30 minutes

### Tasks

1. **Update log_to_file function**
   - Location: `src/mcp_server.rs`, line 11-24
   - Replace hardcoded path with dynamic path resolution
   - Add directory creation logic
   - Add fallback to temp directory

2. **Test logging on current platform**
   - Run MCP server
   - Verify log file is created in project root
   - Verify log entries are written correctly

### Files Changed
- `src/mcp_server.rs`

### Testing
- Manual: Run server and check for `./mcp_debug.log` file
- Manual: Verify log contents are correct
- Manual: Test with read-only project directory (should use temp)

---

## Phase 2: Font Size Fix (Medium Risk)

**Estimated Time:** 1-2 hours

### Tasks

1. **Add font size configuration**
   - Location: `src/pdf_filler.rs`
   - Create helper method to determine appropriate font size
   - Add `/DA` string generation helper

2. **Update fill_pdf_fields to set default appearance**
   - Location: `src/pdf_filler.rs`, line 113-140
   - Set `/DA` property along with `/V` for text fields
   - Use font size based on field name/type

3. **Create font size mapping**
   - Standard fields: 10pt
   - Name fields: 12pt
   - Spell fields: 8pt
   - Narrative fields: 9pt

### Files Changed
- `src/pdf_filler.rs`

### Testing
- Generate test PDF with various text fields
- Verify font sizes are appropriate
- Check for text cutoff or overflow
- Test in multiple PDF viewers

---

## Phase 3: Checkbox Marking Fix (Highest Risk)

**Estimated Time:** 2-3 hours

### Tasks

1. **Research checkbox field structure**
   - Use existing `extract_fields.rs` or `find_all_checkboxes.rs` utilities
   - Identify checkbox export values in D&D character sheet template
   - Document appearance stream structure

2. **Update mark_checkboxes function**
   - Location: `src/pdf_filler.rs`, line 169-190
   - Set `/AS` (Appearance State) along with `/V`
   - Use Name object for `/AS` value, not String
   - Handle missing `/AP` gracefully

3. **Add checkbox debugging**
   - Log checkbox field processing
   - Output checkbox state information for troubleshooting

### Files Changed
- `src/pdf_filler.rs`

### Testing
- Generate PDF with saving throw proficiencies
- Generate PDF with skill proficiencies  
- Generate PDF with prepared spells
- Test on Windows with:
  - Adobe Reader
  - Chrome PDF viewer
  - Edge PDF viewer
- Test on macOS/Linux for regression

---

## Phase 4: Integration Testing

**Estimated Time:** 1 hour

### Tasks

1. **Run existing test suite**
   ```bash
   cargo test
   ```

2. **Manual test with sample character**
   ```bash
   cargo run test
   ```

3. **Cross-platform validation**
   - Test on Windows (if available)
   - Test on macOS
   - Test on Linux

4. **Visual inspection of PDFs**
   - Check all checkboxes are visible
   - Verify font sizes are appropriate
   - Confirm no regressions in other fields

### Success Criteria
- All existing tests pass
- Checkboxes visible on Windows
- Font sizes appropriate
- Debug log created successfully

---

## Rollback Plan

If issues arise:

1. **Checkpoint commits:** Create commit after each phase
2. **Rollback command:** `git reset --hard <commit-hash>`
3. **Specific rollbacks:**
   - Phase 1 (logging): Low risk, unlikely to need rollback
   - Phase 2 (fonts): Revert `/DA` changes if text doesn't fit
   - Phase 3 (checkboxes): Revert to value-only approach if appearances break

---

## Risk Assessment

### Low Risk (Phase 1 - Logging)
- **Impact:** No effect on PDF generation
- **Complexity:** Simple path manipulation
- **Testing:** Easy to verify manually

### Medium Risk (Phase 2 - Fonts)
- **Impact:** Could cause text overflow/cutoff
- **Complexity:** Moderate - font size selection logic
- **Testing:** Requires visual inspection of multiple fields
- **Mitigation:** Use conservative font sizes, start with 10pt default

### High Risk (Phase 3 - Checkboxes)
- **Impact:** Could break existing checkbox marking on other platforms
- **Complexity:** PDF specification details, object types
- **Testing:** Requires testing on multiple platforms and viewers
- **Mitigation:** Test thoroughly, use minimal changes approach

---

## Implementation Order Rationale

1. **Logging First:** Improves debugging capability for subsequent phases
2. **Fonts Second:** Lower risk than checkboxes, provides visual improvement
3. **Checkboxes Last:** Highest risk, benefits from improved logging

---

## Code Examples

### Phase 1: Logging Fix

```rust
fn log_to_file(message: &str) {
    use std::env;
    use std::path::PathBuf;
    
    let log_path: PathBuf = env::current_dir()
        .map(|d| d.join("mcp_debug.log"))
        .unwrap_or_else(|_| env::temp_dir().join("dnd-mcp-debug.log"));
    
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        let _ = writeln!(file, "[{}] {}", 
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            message
        );
    }
}
```

### Phase 2: Font Size Fix

```rust
fn get_font_size_for_field(&self, field_name: &str) -> u8 {
    if field_name.contains("CharacterName") || field_name.contains("Name") {
        12
    } else if field_name.starts_with("Spells") {
        8
    } else if field_name.contains("Traits") || field_name.contains("Features") {
        9
    } else {
        10 // default
    }
}

fn create_default_appearance(&self, field_name: &str) -> String {
    let font_size = self.get_font_size_for_field(field_name);
    format!("/Helv {} Tf 0 g", font_size)
}

// In fill_pdf_fields:
let mut new_dict = dict.clone();
new_dict.set(b"V", Object::String(value.as_bytes().to_vec(), lopdf::StringFormat::Literal));
let da_string = self.create_default_appearance(&field_name);
new_dict.set(b"DA", Object::String(da_string.as_bytes().to_vec(), lopdf::StringFormat::Literal));
doc.objects.insert(object_id, Object::Dictionary(new_dict));
```

### Phase 3: Checkbox Fix

```rust
fn mark_checkboxes(&self, doc: &mut Document, checkbox_fields: &HashMap<String, bool>) -> Result<(), PdfError> {
    for (object_id, object) in doc.objects.clone() {
        if let Object::Dictionary(dict) = object {
            if let Ok(Object::String(field_name_bytes, _)) = dict.get(b"T") {
                let field_name = String::from_utf8_lossy(&field_name_bytes);
                
                if let Some(&should_mark) = checkbox_fields.get(field_name.as_ref()) {
                    if should_mark {
                        let mut new_dict = dict.clone();
                        // Set the value
                        new_dict.set(b"V", Object::Name(b"Yes".to_vec()));
                        // Set the appearance state to match
                        new_dict.set(b"AS", Object::Name(b"Yes".to_vec()));
                        doc.objects.insert(object_id, Object::Dictionary(new_dict));
                    }
                }
            }
        }
    }
    Ok(())
}
```

---

## Dependencies

**No new external dependencies required.**

Existing dependencies sufficient:
- `lopdf` - PDF manipulation
- `std::env` - Cross-platform paths  
- `std::fs` - File operations

---

## Validation Checklist

- [ ] Debug log file created in project root
- [ ] Debug log file works on Windows/macOS/Linux
- [ ] Font sizes appropriate for all field types
- [ ] No text cutoff in critical fields
- [ ] Checkboxes visible in Adobe Reader (Windows)
- [ ] Checkboxes visible in Chrome PDF viewer
- [ ] Checkboxes visible in Edge PDF viewer
- [ ] No checkbox regression on macOS/Linux
- [ ] All existing tests pass
- [ ] Manual test character sheet looks correct
