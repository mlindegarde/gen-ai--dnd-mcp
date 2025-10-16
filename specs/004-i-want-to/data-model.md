# Data Model: Character Sheet PDF Population

**Feature**: Character Sheet PDF Population (Saves, Skills, Spells)  
**Generated**: 2025-10-16  
**Source**: [spec.md](./spec.md)

## Core Entities

### Character
Primary entity containing all character information needed for PDF population.

**Fields:**
- `ability_scores: AbilityScores` - Six core D&D ability scores
- `level: u8` - Character level (1-20) for proficiency bonus calculation
- `saving_throw_proficiencies: Vec<String>` - List of proficient saving throws
- `skill_proficiencies: Vec<String>` - List of proficient skills
- `skill_expertise: Vec<String>` - List of skills with expertise (double proficiency)
- `prepared_spells: HashMap<u8, Vec<String>>` - Prepared spells by level (1-9)

**Validation Rules:**
- Level must be 1-20
- Ability scores must be present for all six abilities
- Saving throw proficiencies must be valid ability names
- Skill proficiencies must be valid D&D 5e skill names
- Expertise skills must also be in proficiencies list
- Prepared spells must be organized by valid spell levels (1-9)

### AbilityScores
Contains the six core D&D ability scores.

**Fields:**
- `strength: u8`
- `dexterity: u8`
- `constitution: u8`
- `intelligence: u8`
- `wisdom: u8`
- `charisma: u8`

**Validation Rules:**
- Each score must be 1-30 (D&D 5e range)
- Scores below 10 result in negative modifiers

### SavingThrow
Calculated saving throw bonus for display in PDF.

**Fields:**
- `ability: String` - Associated ability name
- `modifier: i8` - Ability modifier (-5 to +10)
- `proficiency_bonus: u8` - Proficiency bonus if proficient (0 or level-based)
- `total_bonus: i8` - Final calculated bonus
- `is_proficient: bool` - Whether character is proficient

**Calculations:**
- `modifier = (ability_score - 10) / 2`
- `total_bonus = modifier + (proficiency_bonus if is_proficient else 0)`

### Skill
Calculated skill bonus for display in PDF.

**Fields:**
- `name: String` - Skill name
- `ability: String` - Associated ability name
- `modifier: i8` - Ability modifier
- `proficiency_bonus: u8` - Base proficiency bonus
- `total_bonus: i8` - Final calculated bonus
- `is_proficient: bool` - Whether character is proficient
- `has_expertise: bool` - Whether character has expertise

**Calculations:**
- `modifier = (ability_score - 10) / 2`
- `effective_proficiency = proficiency_bonus * (2 if has_expertise else 1 if is_proficient else 0)`
- `total_bonus = modifier + effective_proficiency`

### PreparedSpell
Represents a prepared spell for checkbox marking.

**Fields:**
- `level: u8` - Spell level (1-9)
- `name: String` - Spell name
- `checkbox_field: String` - PDF checkbox field name for this spell slot

**Validation Rules:**
- Level must be 1-9 (cantrips don't require preparation)
- Checkbox field must be from verified mapping

## Relationships

```
Character (1) ──→ (6) SavingThrow
Character (1) ──→ (18) Skill  
Character (1) ──→ (0..*) PreparedSpell
```

## State Transitions

### Character Processing Flow
1. **Input Validation**: Validate all required fields present
2. **Calculation Phase**: Calculate all bonuses and modifiers
3. **PDF Mapping Phase**: Map calculated values to PDF field names
4. **Checkbox Marking Phase**: Mark proficiency and preparation checkboxes
5. **Error Handling Phase**: Handle any validation or calculation errors

### Error States
- **ValidationError**: Missing or invalid input data
- **CalculationError**: Mathematical calculation failures
- **MappingError**: PDF field mapping failures
- **CheckboxError**: Checkbox marking failures

## Field Mappings

### Saving Throw Checkboxes (Verified)
- Strength: "Check Box 11"
- Dexterity: "Check Box 18"
- Constitution: "Check Box 19"
- Intelligence: "Check Box 20"
- Wisdom: "Check Box 21"
- Charisma: "Check Box 22"

### Skill Checkboxes (Verified)
- Acrobatics: "Check Box 23"
- Animal Handling: "Check Box 24"
- Arcana: "Check Box 25"
- Athletics: "Check Box 26"
- Deception: "Check Box 27"
- History: "Check Box 28"
- Insight: "Check Box 29"
- Intimidation: "Check Box 30"
- Investigation: "Check Box 31"
- Medicine: "Check Box 32"
- Nature: "Check Box 33"
- Perception: "Check Box 34"
- Performance: "Check Box 35"
- Persuasion: "Check Box 36"
- Religion: "Check Box 37"
- Sleight of Hand: "Check Box 38"
- Stealth: "Check Box 39"
- Survival: "Check Box 40"

### Spell Preparation Checkboxes (Verified)
- Level 1: Check Box 251, 309, 3010-3019 (12 total)
- Level 2: Check Box 313, 310, 3020-3030 (13 total)
- Level 3: Check Box 315, 314, 3031-3041 (13 total)
- Level 4: Check Box 317, 316, 3042-3052 (13 total)
- Level 5: Check Box 319, 318, 3053-3059 (9 total)
- Level 6: Check Box 321, 320, 3060-3066 (9 total)
- Level 7: Check Box 323, 322, 3067-3073 (9 total)
- Level 8: Check Box 325, 324, 3074-3078 (7 total)
- Level 9: Check Box 327, 326, 3079-3083 (7 total)

## Implementation Notes

- All checkbox mappings have been systematically verified through testing
- Cantrips (level 0) do not require preparation checkboxes per D&D 5e rules
- Error handling must be granular to allow partial success
- Field mappings are hardcoded based on verified PDF structure
