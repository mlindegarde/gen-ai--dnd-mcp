# Calculation API Contract

## Internal Rust API for Saving Throws and Skills Calculations

### Core Functions

#### `calculate_ability_modifier(ability_score: u8) -> i8`
Calculates D&D 5e ability modifier from ability score.

**Input**: Ability score (1-30)  
**Output**: Modifier (-5 to +10)  
**Formula**: `(ability_score - 10) / 2` (integer division)

#### `get_proficiency_bonus(level: u8) -> Result<u8, ValidationError>`
Returns proficiency bonus for character level.

**Input**: Character level (1-20)  
**Output**: Proficiency bonus (+2 to +6)  
**Error**: `InvalidLevel` if level outside 1-20 range

#### `calculate_saving_throw_bonus(ability_score: u8, level: u8, is_proficient: bool) -> Result<i8, CalculationError>`
Calculates total saving throw bonus.

**Input**: 
- `ability_score`: Base ability score
- `level`: Character level for proficiency bonus
- `is_proficient`: Whether character is proficient in this save

**Output**: Total bonus (ability modifier + proficiency if applicable)  
**Error**: `MissingAbilityScore`, `InvalidLevel`

#### `calculate_skill_bonus(skill: Skill, ability_scores: &AbilityScores, level: u8, is_proficient: bool, has_expertise: bool) -> Result<i8, CalculationError>`
Calculates total skill bonus.

**Input**:
- `skill`: Skill enum value
- `ability_scores`: All character ability scores
- `level`: Character level for proficiency bonus
- `is_proficient`: Whether character is proficient in this skill
- `has_expertise`: Whether character has expertise in this skill

**Output**: Total bonus (ability modifier + proficiency multiplier)  
**Error**: `MissingAbilityScore`, `InvalidLevel`, `InvalidSkill`

### Data Structures

#### `SavingThrowResult`
```rust
pub struct SavingThrowResult {
    pub ability: Ability,
    pub total_bonus: i8,
    pub ability_modifier: i8,
    pub proficiency_bonus: u8,
    pub is_proficient: bool,
}
```

#### `SkillResult`
```rust
pub struct SkillResult {
    pub skill: Skill,
    pub total_bonus: i8,
    pub ability_modifier: i8,
    pub proficiency_bonus: u8,
    pub is_proficient: bool,
    pub has_expertise: bool,
}
```

#### `CalculationError`
```rust
pub enum CalculationError {
    MissingAbilityScore(Ability),
    InvalidLevel(u8),
    InvalidSkill(String),
    ValidationFailed(String),
}
```

### PDF Field Mapping

#### `map_saving_throw_fields() -> HashMap<Ability, String>`
Returns mapping of abilities to PDF field names for saving throws.

#### `map_skill_fields() -> HashMap<Skill, String>`
Returns mapping of skills to PDF field names.

#### `map_proficiency_indicator_fields() -> HashMap<String, String>`
Returns mapping of saves/skills to their proficiency checkbox field names.

### MCP Tool Response

#### `fill_dnd_character_sheet` Response
```rust
pub struct FillCharacterSheetResponse {
    pub success: bool,
    pub output_file: String,
    pub pdf_content: Option<String>, // Base64 encoded PDF if return_pdf_content=true
    pub calculated_values: CalculatedValues,
    pub errors: Vec<CalculationError>,
}
```

### Error Handling Contract

- Functions return `Result<T, E>` for all fallible operations
- Partial success: Continue processing valid data, collect errors for invalid data
- Error indicators: Use "ERROR" text in PDF fields where calculations fail
- Validation: Check all required data before attempting calculations
