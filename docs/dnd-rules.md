# D&D 5e Rule Specifications

**Purpose**: Complete D&D 5e rule definitions for validation and calculation
**Date**: 2025-10-14

## Core Mechanics

### Ability Scores
- **Range**: 1-20 (standard for player characters)
- **Modifier Formula**: `(ability_score - 10) / 2` (rounded down)
- **Examples**:
  - Score 8 → Modifier -1
  - Score 10 → Modifier 0  
  - Score 15 → Modifier +2
  - Score 20 → Modifier +5

### Proficiency Bonus by Level
- **Levels 1-4**: +2
- **Levels 5-8**: +3
- **Levels 9-12**: +4
- **Levels 13-16**: +5
- **Levels 17-20**: +6

### Saving Throws
- **Formula**: `ability_modifier + proficiency_bonus (if proficient)`
- **Proficiency**: Determined by character class
- **Examples**:
  - Dex save, Dex 14 (+2), Level 3, Proficient → +2 + 2 = +4
  - Str save, Str 10 (0), Level 5, Not Proficient → 0

### Skills
- **Formula**: `ability_modifier + proficiency_bonus (if proficient) + expertise_bonus (if expertise)`
- **Expertise Bonus**: Equals proficiency bonus (double proficiency)
- **Base Abilities**:
  - Acrobatics (Dex), Animal Handling (Wis), Arcana (Int)
  - Athletics (Str), Deception (Cha), History (Int)
  - Insight (Wis), Intimidation (Cha), Investigation (Int)
  - Medicine (Wis), Nature (Int), Perception (Wis)
  - Performance (Cha), Persuasion (Cha), Religion (Int)
  - Sleight of Hand (Dex), Stealth (Dex), Survival (Wis)

## Spellcasting Rules

### Spell Levels
- **Cantrips**: Level 0, unlimited use, always prepared
- **Leveled Spells**: Levels 1-9, limited by spell slots
- **Prepared Spells**: Must be prepared to cast (except cantrips)

### Spellcasting Mechanics
- **Spell Attack Bonus**: `spellcasting_ability_modifier + proficiency_bonus`
- **Spell Save DC**: `8 + spellcasting_ability_modifier + proficiency_bonus`
- **Spellcasting Abilities by Class**:
  - Cleric, Druid, Ranger → Wisdom
  - Wizard → Intelligence  
  - Bard, Paladin, Sorcerer, Warlock → Charisma

### Spell Organization
- **Cantrips**: Separate section, always prepared
- **1st-9th Level**: Organized by spell level in PDF
- **Prepared State**: Marked with filled circles in PDF

## Validation Rules

### Character Level Constraints
- **Level Range**: 1-20
- **Experience Points by Level**:
  - Level 1: 0 XP
  - Level 2: 300 XP
  - Level 3: 900 XP
  - Level 4: 2,700 XP
  - Level 5: 6,500 XP
  - [Continue through Level 20: 355,000 XP]

### Combat Statistics
- **Armor Class**: Minimum 1 (no maximum)
- **Hit Points**: Minimum 1 for maximum, 0+ for current
- **Initiative**: Typically equals Dex modifier
- **Speed**: Typically 30 feet for most races
- **Death Saves**: 0-3 successes, 0-3 failures

### Equipment Constraints
- **Currency**: Non-negative integers only
- **Weapons**: Must have name, attack bonus, and damage
- **Armor**: Affects AC calculation

## Error Messages

### Ability Score Errors
- `"Strength score {value} must be between 1 and 20"`
- `"Ability modifier {calculated} doesn't match expected {expected} for score {score}"`

### Level Errors  
- `"Character level {level} must be between 1 and 20"`
- `"Experience points {xp} insufficient for level {level} (requires {required})"`

### Spell Errors
- `"Spell level {level} must be between 0 and 9"`
- `"Cantrips (level 0 spells) are always prepared"`
- `"Spellcasting ability '{ability}' must be Wisdom, Intelligence, or Charisma"`

### Proficiency Errors
- `"Proficiency bonus {bonus} invalid for level {level} (expected {expected})"`
- `"Skill bonus calculation error: {skill} should be {expected} but got {actual}"`

## Override Behavior

When `allow_rule_violations: true`:
- Log validation errors as warnings
- Proceed with provided values
- Mark violations in output for user awareness
- Still enforce basic type constraints (strings, number ranges)
