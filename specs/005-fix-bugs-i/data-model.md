# Data Model: Character Sheet Field Population Bug Fixes

**Date**: 2025-10-17  
**Feature**: 005-fix-bugs-i

## Entities

### Character Narrative (Existing)
**Purpose**: Contains character personality and background information  
**Fields**:
- `personality_traits: Option<String>` - Character personality description
- `ideals: Option<String>` - Character ideals and motivations  
- `bonds: Option<String>` - Character bonds and connections
- `flaws: Option<String>` - Character flaws and weaknesses

**Validation Rules**:
- Personality traits text truncated if exceeds PDF field capacity (500 chars)
- All fields optional (graceful handling of missing data)

**State Transitions**: None (static data)

### Ability Scores (Existing - Extended)
**Purpose**: Character ability scores used for calculations  
**Fields**:
- `wisdom: u8` - Wisdom ability score (1-30)
- Other abilities unchanged

**New Calculations**:
- `passive_perception() -> u8` - 10 + wisdom_modifier + (proficiency_bonus if proficient in Perception)

**Validation Rules**:
- Wisdom score must be 1-30 (D&D 5e valid range)
- Passive perception calculation only if valid wisdom score

### Character Class (Existing - Extended)  
**Purpose**: Character class information for hit dice calculation
**Fields**:
- `class: String` - Character class name
- `level: u8` - Character level (1-20)

**New Calculations**:
- `hit_dice() -> String` - Format: "[level]d[die_size]" based on class

**Hit Dice Mapping**:
```rust
match class.as_str() {
    "Barbarian" => "d12",
    "Fighter" | "Paladin" | "Ranger" => "d10", 
    "Bard" | "Cleric" | "Druid" | "Monk" | "Rogue" | "Warlock" => "d8",
    "Sorcerer" | "Wizard" => "d6",
    _ => "d8" // Default fallback
}
```

**Validation Rules**:
- Level must be 1-20 (D&D 5e valid range)
- Single-class only (multi-class explicitly excluded)

### Features & Traits (New)
**Purpose**: Contains character features and racial traits information  
**Fields**:
- `features: Option<Vec<String>>` - Character class features
- `traits: Option<Vec<String>>` - Racial traits and abilities

**Validation Rules**:
- All fields optional (graceful handling of missing data)
- Combined text truncated if exceeds PDF field capacity (500 chars)
- Features and traits combined into single display string

**State Transitions**: None (static data)

### Currency (Existing)
**Purpose**: Character currency amounts  
**Fields**:
- `cp: Option<u32>` - Copper pieces
- `sp: Option<u32>` - Silver pieces  
- `ep: Option<u32>` - Electrum pieces
- `gp: Option<u32>` - Gold pieces
- `pp: Option<u32>` - Platinum pieces

**Validation Rules**:
- All currency amounts optional
- No negative values (u32 type enforcement)
- No maximum limits (reasonable amounts assumed)

**State Transitions**: None (static data)

### Proficiencies (Existing - Referenced)
**Purpose**: Character skill and saving throw proficiencies
**Fields**:
- `skills: Vec<String>` - List of proficient skills including "perception"

**Usage**: Referenced for passive perception calculation (proficiency bonus applied if "perception" in skills list)

## Field Mappings

### PDF Field Names (New Mappings)
```rust
// Personality traits
"personality_traits" -> "PersonalityTraits" // PDF field name

// Passive perception  
"passive_perception" -> "Passive" // PDF field name

// Hit dice
"hit_dice_total" -> "HDTotal" // PDF field name
"hit_dice_type" -> "HD" // PDF field name

// Currency fields
"cp" -> "CP" // Copper pieces PDF field
"sp" -> "SP" // Silver pieces PDF field  
"ep" -> "EP" // Electrum pieces PDF field
"gp" -> "GP" // Gold pieces PDF field
"pp" -> "PP" // Platinum pieces PDF field

// Features & Traits
"features_traits" -> "Features & Traits" // PDF field name
```

## Data Flow

### Bug Fix Processing Flow:
1. **Input**: Character JSON data with narrative, abilities, class, currency, features & traits
2. **Field Mapping**: Map logical field names to PDF field names
3. **Calculations**: 
   - Calculate passive perception from wisdom + proficiency
   - Format hit dice from class + level (separate HDTotal and HD fields)
4. **Population**: Fill PDF fields with calculated/mapped values
5. **Output**: PDF with populated personality traits, passive perception, hit dice, currency, features & traits

### Error Handling:
- Missing narrative data → Empty personality traits field
- Invalid wisdom score → Skip passive perception calculation  
- Unknown class → Use d8 hit dice default
- Missing currency → Empty currency fields
- PDF field not found → Log warning, continue processing

## Relationships

- **Character** → **Narrative** (1:1, optional)
- **Character** → **Abilities** (1:1, required) 
- **Character** → **Proficiencies** (1:1, optional)
- **Character** → **Features & Traits** (1:1, optional)
- **Character** → **Equipment** → **Currency** (1:1:1, optional)
- **PDF Fields** ← **Field Mapper** ← **Character Data** (mapping layer)
