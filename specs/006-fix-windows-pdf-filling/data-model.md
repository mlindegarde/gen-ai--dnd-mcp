# Data Model: Fix PDF Filling Issues on Windows

## Overview

This feature doesn't introduce new data structures but modifies how existing PDF field data is written. The changes affect PDF object properties and system path handling.

## PDF Object Model Changes

### Text Field Objects (Modified)

**Before:**
```rust
Dictionary {
    "T": String("CharacterName", Literal),  // Field name
    "V": String("Thorin", Literal),         // Value only
}
```

**After:**
```rust
Dictionary {
    "T": String("CharacterName", Literal),  // Field name
    "V": String("Thorin", Literal),         // Value
    "DA": String("/Helv 12 Tf 0 g", Literal), // NEW: Default appearance with font size
}
```

### Checkbox Field Objects (Modified)

**Before:**
```rust
Dictionary {
    "T": String("Check Box 11", Literal),           // Field name (Strength save)
    "V": String("Yes", Literal),                    // Value as String
}
```

**After:**
```rust
Dictionary {
    "T": String("Check Box 11", Literal),           // Field name (Strength save)
    "V": Name(b"Yes"),                              // CHANGED: Value as Name object
    "AS": Name(b"Yes"),                             // NEW: Appearance state
}
```

## Configuration Data

### Font Size Mapping (New)

```rust
struct FontSizeMap {
    name_fields: u8,        // 12pt - character name, player name
    standard_fields: u8,    // 10pt - ability scores, skills, saves
    dense_fields: u8,       // 8pt - spell lists, inventory
    narrative_fields: u8,   // 9pt - personality traits, backstory
}

impl Default for FontSizeMap {
    fn default() -> Self {
        Self {
            name_fields: 12,
            standard_fields: 10,
            dense_fields: 8,
            narrative_fields: 9,
        }
    }
}
```

### Log Path Configuration (New)

```rust
struct LogConfig {
    primary_path: PathBuf,   // Project root: ./mcp_debug.log
    fallback_path: PathBuf,  // Temp dir: /tmp/dnd-mcp-debug.log
}

impl LogConfig {
    fn resolve() -> PathBuf {
        if let Ok(current_dir) = env::current_dir() {
            let log_path = current_dir.join("mcp_debug.log");
            if Self::is_writable(&log_path) {
                return log_path;
            }
        }
        env::temp_dir().join("dnd-mcp-debug.log")
    }
    
    fn is_writable(path: &PathBuf) -> bool {
        // Try to open file for writing
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .is_ok()
    }
}
```

## PDF Field Types Affected

### Text Fields
Fields that receive `/DA` property:

- `CharacterName`
- `ClassLevel`
- `Race`
- `Background`
- `PlayerName`
- `Alignment`
- `STR`, `DEX`, `CON`, `INT`, `WIS`, `CHA` (ability scores)
- `STRmod`, `DEXmod`, `CONmod`, `INTmod`, `WISmod`, `CHAmod` (modifiers)
- All skill fields (`Acrobatics`, `AnimalHandling`, etc.)
- Spell name fields (`Spells 1014` through various levels)
- Narrative fields (`PersonalityTraits`, `Ideals`, `Bonds`, `Flaws`)
- Equipment fields
- Features & Traits

### Checkbox Fields
Fields that receive `/V` and `/AS` as Name objects:

- Saving throw proficiency checkboxes:
  - `Check Box 11` (Strength)
  - `Check Box 18` (Dexterity)
  - `Check Box 19` (Constitution)
  - `Check Box 20` (Intelligence)
  - `Check Box 21` (Wisdom)
  - `Check Box 22` (Charisma)

- Skill proficiency checkboxes:
  - `Check Box 23` through `Check Box 40` (18 skills)

- Spell preparation checkboxes:
  - Level 1: `Check Box 313` through `Check Box 324`
  - Level 2: `Check Box 325` through `Check Box 337`
  - Level 3: `Check Box 338` through `Check Box 350`
  - Level 4: `Check Box 351` through `Check Box 363`
  - Level 5: `Check Box 364` through `Check Box 372`
  - Level 6: `Check Box 373` through `Check Box 381`
  - Level 7: `Check Box 382` through `Check Box 390`
  - Level 8: `Check Box 391` through `Check Box 399`
  - Level 9: `Check Box 400` through `Check Box 408`

## PDF Object Type Changes

### lopdf Object Types Used

**String vs Name Objects:**

```rust
// OLD: Checkbox value as String
Object::String(b"Yes".to_vec(), lopdf::StringFormat::Literal)

// NEW: Checkbox value as Name
Object::Name(b"Yes".to_vec())
```

**Why the change?**
- PDF specification requires button field states to be Name objects, not Strings
- Some PDF viewers (especially on Windows) enforce this requirement strictly
- Name objects reference appearance stream keys correctly

## Default Appearance String Format

```
/DA (<font_name> <font_size> Tf <red> <green> <blue> rg)

Components:
- font_name: PDF font resource name (e.g., /Helv, /Times)
- font_size: Point size (e.g., 10, 12)
- Tf: "Text font" operator
- r g b: RGB color values (0-1 range)
- rg: "RGB fill color" operator

Examples:
/Helv 12 Tf 0 g          - Helvetica 12pt black (grayscale)
/Helv 10 Tf 0 0 0 rg     - Helvetica 10pt black (RGB)
/Times 9 Tf 0 g          - Times 9pt black
```

## System Path Model

### Path Resolution Logic

```
┌─────────────────────────────┐
│ Attempt: current_dir()      │
│ Path: ./mcp_debug.log       │
└──────────┬──────────────────┘
           │
           ├─ Success & Writable? ──> Use this path
           │
           └─ Failed or Not Writable
                     │
                     ▼
┌─────────────────────────────┐
│ Fallback: temp_dir()        │
│ Path: $TEMP/dnd-mcp-debug.log│
└─────────────────────────────┘
```

### Platform-Specific Paths

| Platform | Primary Path | Fallback Path |
|----------|--------------|---------------|
| Windows | `C:\path\to\project\mcp_debug.log` | `C:\Users\USER\AppData\Local\Temp\dnd-mcp-debug.log` |
| macOS | `/path/to/project/mcp_debug.log` | `/tmp/dnd-mcp-debug.log` |
| Linux | `/path/to/project/mcp_debug.log` | `/tmp/dnd-mcp-debug.log` |

## Impact on Existing Data Structures

**No changes to:**
- `CharacterData` struct
- `CharacterInfo` struct
- `Abilities` struct
- `Proficiencies` struct
- `Spells` struct
- Any JSON schema or API contracts

**Only changes to:**
- PDF object dictionaries (runtime only)
- Internal path handling (runtime only)

## Constants and Configuration

```rust
// Font sizes (suggested additions to pdf_filler.rs)
const FONT_SIZE_NAME: u8 = 12;
const FONT_SIZE_STANDARD: u8 = 10;
const FONT_SIZE_DENSE: u8 = 8;
const FONT_SIZE_NARRATIVE: u8 = 9;

// Default font
const DEFAULT_FONT: &str = "/Helv";

// Checkbox export value
const CHECKBOX_ON_VALUE: &[u8] = b"Yes";
const CHECKBOX_OFF_VALUE: &[u8] = b"Off";

// Log file name
const LOG_FILE_NAME: &str = "mcp_debug.log";
```

## Validation Rules

### Font Size Validation
- Must be between 6 and 14 points
- Should not cause text overflow (visual check required)

### Path Validation
- Must be writable
- Parent directory must exist or be creatable
- Should not throw errors on permission issues

### Checkbox Value Validation
- Must use Name objects, not Strings
- Export value must match template expectations ("Yes"/"Off")
- Appearance state must match value

## Error Handling

**No new error types needed.**

Existing error handling covers:
- `PdfError::WriteError` - PDF save failures
- `io::Error` - File system operations
- Graceful degradation for log failures (already silent)

## Database/Storage Impact

**None.** This feature only affects:
- In-memory PDF objects before writing
- Temporary log file on filesystem
- No database, no persistent storage changes
