# Implementation Guide: Fix Windows PDF Filling Issues

## Quick Reference

### Files to Modify
1. `src/mcp_server.rs` - Lines 11-24 (log_to_file function)
2. `src/pdf_filler.rs` - Lines 113-140 (fill_pdf_fields method)
3. `src/pdf_filler.rs` - Lines 169-190 (mark_checkboxes method)

### Total Changes
- **Lines Modified:** ~50 lines
- **New Helper Methods:** 2 (get_font_size_for_field, create_default_appearance)
- **New Dependencies:** None
- **Breaking Changes:** None

---

## Phase 1: Debug Logging Fix (30 minutes)

### Change 1: Update log_to_file function

**File:** `src/mcp_server.rs`  
**Line:** 11

**Before:**
```rust
fn log_to_file(message: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/Users/lindegar/learningplace/specify--test/mcp_debug.log")
    {
        let _ = writeln!(
            file,
            "[{}] {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            message
        );
    }
}
```

**After:**
```rust
fn log_to_file(message: &str) {
    use std::env;
    use std::path::PathBuf;
    
    // Try project root first, fall back to temp directory
    let log_path: PathBuf = env::current_dir()
        .map(|d| d.join("mcp_debug.log"))
        .unwrap_or_else(|_| env::temp_dir().join("dnd-mcp-debug.log"));
    
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        let _ = writeln!(
            file,
            "[{}] {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            message
        );
    }
}
```

**Test:**
```bash
cargo run test
ls -la mcp_debug.log  # Should exist in project root
```

---

## Phase 2: Font Size Fix (1-2 hours)

### Change 2: Add helper methods to PdfFiller

**File:** `src/pdf_filler.rs`  
**Location:** Inside `impl PdfFiller` block (around line 650, after existing methods)

**Add these new methods:**

```rust
/// Determine appropriate font size based on field name
fn get_font_size_for_field(&self, field_name: &str) -> u8 {
    // Character name and player name - larger for readability
    if field_name.contains("CharacterName") || field_name.contains("PlayerName") {
        return 12;
    }
    
    // Spell fields - smaller to fit more text
    if field_name.starts_with("Spells") {
        return 8;
    }
    
    // Narrative fields - slightly smaller
    if field_name.contains("Traits") || 
       field_name.contains("Features") || 
       field_name.contains("Ideals") ||
       field_name.contains("Bonds") ||
       field_name.contains("Flaws") {
        return 9;
    }
    
    // Default for all other fields
    10
}

/// Create PDF default appearance string with font and size
fn create_default_appearance(&self, field_name: &str) -> String {
    let font_size = self.get_font_size_for_field(field_name);
    // Format: /FontName FontSize Tf Color
    // /Helv = Helvetica, Tf = Text Font operator, 0 g = black
    format!("/Helv {} Tf 0 g", font_size)
}
```

### Change 3: Update fill_pdf_fields to set font size

**File:** `src/pdf_filler.rs`  
**Line:** ~129 (where field value is set)

**Before:**
```rust
if let Some(value) = field_values.get(field_name.as_ref()) {
    let mut new_dict = dict.clone();
    new_dict.set(b"V", Object::String(value.as_bytes().to_vec(), lopdf::StringFormat::Literal));
    doc.objects.insert(object_id, Object::Dictionary(new_dict));
}
```

**After:**
```rust
if let Some(value) = field_values.get(field_name.as_ref()) {
    let mut new_dict = dict.clone();
    
    // Set the field value
    new_dict.set(b"V", Object::String(value.as_bytes().to_vec(), lopdf::StringFormat::Literal));
    
    // Set default appearance with appropriate font size
    let da_string = self.create_default_appearance(&field_name);
    new_dict.set(b"DA", Object::String(da_string.as_bytes().to_vec(), lopdf::StringFormat::Literal));
    
    doc.objects.insert(object_id, Object::Dictionary(new_dict));
}
```

**Test:**
```bash
cargo run test
# Open filled_character_sheet.pdf and verify font sizes look appropriate
```

---

## Phase 3: Checkbox Marking Fix (2-3 hours)

### Change 4: Update mark_checkboxes to use Name objects

**File:** `src/pdf_filler.rs`  
**Line:** 169

**Before:**
```rust
fn mark_checkboxes(
    &self,
    doc: &mut Document,
    checkbox_fields: &HashMap<String, bool>,
) -> Result<(), PdfError> {
    for (object_id, object) in doc.objects.clone() {
        if let Object::Dictionary(dict) = object {
            if let Ok(Object::String(field_name_bytes, _)) = dict.get(b"T") {
                let field_name = String::from_utf8_lossy(&field_name_bytes);
                
                if let Some(&should_mark) = checkbox_fields.get(field_name.as_ref()) {
                    if should_mark {
                        let mut new_dict = dict.clone();
                        new_dict.set(b"V", Object::String(b"Yes".to_vec(), lopdf::StringFormat::Literal));
                        doc.objects.insert(object_id, Object::Dictionary(new_dict));
                    }
                }
            }
        }
    }
    Ok(())
}
```

**After:**
```rust
fn mark_checkboxes(
    &self,
    doc: &mut Document,
    checkbox_fields: &HashMap<String, bool>,
) -> Result<(), PdfError> {
    for (object_id, object) in doc.objects.clone() {
        if let Object::Dictionary(dict) = object {
            if let Ok(Object::String(field_name_bytes, _)) = dict.get(b"T") {
                let field_name = String::from_utf8_lossy(&field_name_bytes);
                
                if let Some(&should_mark) = checkbox_fields.get(field_name.as_ref()) {
                    if should_mark {
                        let mut new_dict = dict.clone();
                        
                        // Use Name objects instead of String for checkbox values
                        // This is required by PDF spec for button fields
                        new_dict.set(b"V", Object::Name(b"Yes".to_vec()));
                        
                        // Set appearance state to match the value
                        // This tells PDF viewers which appearance to display
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

**Key Changes:**
1. Changed `Object::String(...)` to `Object::Name(...)` for `/V` field
2. Added `/AS` (Appearance State) field with same value
3. Added explanatory comments

**Test:**
```bash
cargo run test
# Open filled_character_sheet.pdf on Windows
# Verify checkboxes are visible in:
#   - Adobe Reader
#   - Chrome PDF viewer
#   - Edge PDF viewer
```

---

## Complete Testing Checklist

### Phase 1 - Logging
```bash
# Start fresh
rm -f mcp_debug.log

# Run the server
cargo run test

# Verify log exists
ls -la mcp_debug.log

# Check contents
tail mcp_debug.log
```

**Expected:** Log file exists in project root with timestamp entries

---

### Phase 2 - Font Sizing
```bash
# Generate PDF
cargo run test

# Open PDF and check these fields:
# - CharacterName: Should be 12pt (larger)
# - Ability scores: Should be 10pt (standard)
# - Spell names: Should be 8pt (smaller)
# - Personality Traits: Should be 9pt (medium)
```

**Expected:** Font sizes are appropriate, text is readable, no cutoff

---

### Phase 3 - Checkboxes
```bash
# Generate PDF with proficiencies
cargo run test

# Open PDF and verify these checkboxes are CHECKED:
# - Saving throw proficiencies (if character has them)
# - Skill proficiencies (if character has them)
# - Prepared spells (if character has them)
```

**Expected:** All proficiency checkboxes are visibly marked/checked

---

## Testing on Multiple Platforms

### Windows
```powershell
cargo test
cargo run test
# Open PDF in Adobe Reader, Chrome, Edge
```

### macOS
```bash
cargo test
cargo run test
# Open PDF in Preview, Chrome
```

### Linux
```bash
cargo test
cargo run test
# Open PDF in Evince, Chrome
```

---

## Troubleshooting

### Issue: Log file still not created
```bash
# Check current directory
pwd

# Check permissions
ls -la .

# Try temp directory
ls -la /tmp/dnd-mcp-debug.log  # macOS/Linux
dir %TEMP%\dnd-mcp-debug.log   # Windows
```

### Issue: Checkboxes still not visible
```bash
# Verify checkbox fields exist
cargo run --bin find_all_checkboxes

# Check test character has proficiencies
cat tests/fixtures/sample-character.json | grep -A 10 proficiencies

# Try different PDF viewer (Adobe Reader most reliable)
```

### Issue: Font still wrong
```bash
# Check if /DA is being set
# Add debug print in create_default_appearance:
println!("Setting font for {}: {}", field_name, da_string);

# Verify PDF template supports fonts
cargo run --bin extract_fields
```

---

## Validation Commands

```bash
# 1. Build check
cargo build --release

# 2. Lint check
cargo clippy -- -D warnings

# 3. Test suite
cargo test

# 4. Format check
cargo fmt -- --check

# 5. Generate test PDF
cargo run test

# 6. Manual inspection
# Open filled_character_sheet.pdf
# Check: ✓ Checkboxes  ✓ Fonts  ✓ Log file
```

---

## Git Workflow

```bash
# Create feature branch
git checkout -b 006-fix-windows-pdf-filling

# Make changes following this guide

# Commit Phase 1
git add src/mcp_server.rs
git commit -m "Fix: Use cross-platform path for debug log"

# Commit Phase 2
git add src/pdf_filler.rs
git commit -m "Fix: Add font size specification to text fields"

# Commit Phase 3
git add src/pdf_filler.rs
git commit -m "Fix: Use Name objects and appearance state for checkboxes"

# Test everything
cargo test
cargo run test

# Push
git push origin 006-fix-windows-pdf-filling
```

---

## Success Indicators

✅ **Debug Logging Fixed:**
- `mcp_debug.log` exists in project root
- Log entries have timestamps
- Works on Windows, macOS, Linux

✅ **Font Sizing Fixed:**
- Character name is 12pt (larger, readable)
- Standard fields are 10pt (appropriate)
- Spell fields are 8pt (fit more text)
- No text cutoff in critical fields

✅ **Checkbox Marking Fixed:**
- All proficiency checkboxes are visible
- Works in Adobe Reader (Windows)
- Works in Chrome PDF viewer
- Works in Edge PDF viewer
- No regression on macOS/Linux

✅ **Quality Checks:**
- All tests pass: `cargo test`
- No clippy warnings: `cargo clippy`
- Code is formatted: `cargo fmt`
- Manual test works: `cargo run test`

---

## Estimated Time Breakdown

| Phase | Task | Time |
|-------|------|------|
| Phase 1 | Update logging | 15 min |
| Phase 1 | Test logging | 15 min |
| Phase 2 | Add helper methods | 30 min |
| Phase 2 | Update fill_pdf_fields | 20 min |
| Phase 2 | Test font sizes | 30 min |
| Phase 3 | Research checkboxes | 30 min |
| Phase 3 | Update mark_checkboxes | 30 min |
| Phase 3 | Test checkboxes | 60 min |
| Phase 4 | Integration testing | 60 min |
| Phase 5 | Documentation | 30 min |
| **TOTAL** | | **5-7 hours** |

---

## Questions or Issues?

Refer to these documents:
- **spec.md** - Full feature specification
- **research.md** - Technical background
- **plan.md** - Detailed implementation plan
- **tasks.md** - Task checklist
- **quickstart.md** - Quick start guide
