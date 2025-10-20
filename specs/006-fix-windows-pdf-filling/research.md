# Research: Fix PDF Filling Issues on Windows

## Problem Analysis

### 1. Checkbox Marking Issue

**Current Implementation:**
```rust
// src/pdf_filler.rs, line 169-190
fn mark_checkboxes(&self, doc: &mut Document, checkbox_fields: &HashMap<String, bool>) {
    for (object_id, object) in doc.objects.clone() {
        if let Object::Dictionary(dict) = object {
            if let Ok(Object::String(field_name_bytes, _)) = dict.get(b"T") {
                let field_name = String::from_utf8_lossy(&field_name_bytes);
                if let Some(&should_mark) = checkbox_fields.get(field_name.as_ref()) {
                    if should_mark {
                        let mut new_dict = dict.clone();
                        new_dict.set(b"V", Object::String(b"Yes".to_vec(), ...));
                        doc.objects.insert(object_id, Object::Dictionary(new_dict));
                    }
                }
            }
        }
    }
}
```

**Problem:** Only sets the value (`/V`) to "Yes", but doesn't update:
- `/AS` (Appearance State) - tells the viewer which appearance to use
- `/AP` (Appearance Dictionary) - may need regeneration signal

**Why Windows is affected:** Some PDF viewers (especially on Windows) rely more heavily on appearance streams rather than just the value field.

### 2. Font Size Issue

**Current Implementation:**
```rust
// src/pdf_filler.rs, line 129
new_dict.set(b"V", Object::String(value.as_bytes().to_vec(), lopdf::StringFormat::Literal));
```

**Problem:** Only sets value (`/V`) without setting default appearance (`/DA`) that includes font size.

**PDF Field Appearance Format:**
```
/DA (font_name font_size Tf color_r color_g color_b rg)
Example: /DA (/Helv 10 Tf 0 0 0 rg)
```

Without `/DA`, PDF readers use their own default font size, which may be too large.

### 3. Debug Log Path Issue

**Current Implementation:**
```rust
// src/mcp_server.rs, line 15
.open("/Users/lindegar/learningplace/specify--test/mcp_debug.log")
```

**Problem:** Absolute path hardcoded to macOS-specific location that doesn't exist on Windows.

**Windows vs Unix Paths:**
- macOS/Linux: `/Users/username/...`
- Windows: `C:\Users\username\...`

## Technical Research

### PDF Checkbox Specification (PDF 1.7)

From Adobe PDF Reference:

**Button Field Types:**
- Push buttons (no states)
- Check boxes (two states: on/off)
- Radio buttons (mutually exclusive)

**Checkbox State Representation:**
```
/V /Yes          % Value - logical state
/AS /Yes         % Appearance State - which appearance to display
/AP << 
  /N <<          % Normal appearances
    /Yes 23 0 R  % Reference to checked appearance
    /Off 24 0 R  % Reference to unchecked appearance
  >>
>>
```

**Export Value:** The "on" state is typically "Yes" but can be any name. Common values:
- "Yes"
- "On"  
- Custom values (e.g., "Strength" for Strength save)

### Font Sizing in PDF Forms

**Default Appearance String Format:**
```
/DA (font_name font_size Tf text_r text_g text_b rg)
```

**Common Values:**
- `/Helv` - Helvetica (built-in PDF font)
- `10 Tf` - 10 point font size
- `0 g` or `0 0 0 rg` - Black color

**Font Size Guidelines:**
- Standard fields: 10-12pt
- Dense fields (spells, inventory): 7-9pt
- Small fields (modifiers, scores): 8-10pt

### Cross-Platform Path Resolution

**Rust Standard Library Options:**

1. **Current Directory (Relative Path):**
```rust
use std::env;
let log_path = env::current_dir()?.join("mcp_debug.log");
```

2. **Temp Directory (Fallback):**
```rust
use std::env;
let log_path = env::temp_dir().join("dnd-mcp-debug.log");
```

3. **Project Root (Best for Development):**
```rust
// Relative to current working directory
let log_path = "./mcp_debug.log";
```

**Directory Creation:**
```rust
use std::fs;
use std::path::Path;

if let Some(parent) = Path::new(&log_path).parent() {
    fs::create_dir_all(parent)?;
}
```

## Research Findings

### Checkbox Appearance Stream Requirements

**Minimum Required Changes:**
1. Set `/V` to export value (currently done: "Yes")
2. Set `/AS` to same export value (NEW: "Yes")
3. Optionally remove `/AP` to force regeneration (already done in another function)

**Enhanced Approach:**
1. Set `/V` to "Yes"
2. Set `/AS` to "/Yes" (as a Name object, not String)
3. Keep existing `/AP` if present
4. If no `/AP`, rely on PDF viewer to generate from `/V` and `/AS`

### Font Size Best Practices

Based on D&D character sheet analysis:
- Character name: 12pt
- Ability scores: 10pt
- Skill/save names: 9pt
- Spell names: 8pt
- Long text fields (personality, etc.): 9pt

**Implementation Strategy:**
- Apply default 10pt to all fields initially
- Use field name patterns to apply specific sizes:
  - `*name*`: 12pt
  - `Spells*`: 8pt
  - `*Traits*`: 9pt

### Path Resolution Decision

**Recommended Approach:**
```rust
fn get_log_path() -> PathBuf {
    // Try current directory first
    if let Ok(current_dir) = env::current_dir() {
        let log_path = current_dir.join("mcp_debug.log");
        if let Ok(_) = OpenOptions::new().append(true).create(true).open(&log_path) {
            return log_path;
        }
    }
    
    // Fallback to temp directory
    env::temp_dir().join("dnd-mcp-debug.log")
}
```

**Benefits:**
- Works on all platforms
- No hardcoded paths
- Graceful fallback
- Easy to find during development (in project root)

## Experiments Needed

1. **Checkbox Testing:**
   - Generate PDF with only `/V` set (current behavior)
   - Generate PDF with `/V` and `/AS` set
   - Test in: Adobe Reader (Windows), Chrome PDF viewer, Edge PDF viewer
   - Compare visibility across viewers

2. **Font Size Testing:**
   - Generate PDF without `/DA` (current behavior)
   - Generate PDF with `/DA` set to various sizes (8pt, 10pt, 12pt)
   - Verify text fits and is readable

3. **Path Testing:**
   - Test log creation on Windows, macOS, Linux
   - Verify permissions and directory creation
   - Test fallback behavior when project root is read-only

## References

- PDF Reference 1.7: Section 8.6.2 (Form Fields)
- PDF Reference 1.7: Section 12.7.4 (Button Fields)
- Rust std::env documentation: https://doc.rust-lang.org/std/env/
- lopdf crate documentation: https://docs.rs/lopdf/
