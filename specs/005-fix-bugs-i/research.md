# Research: Character Sheet Field Population Bug Fixes

**Date**: 2025-10-17  
**Feature**: 005-fix-bugs-i

## Technical Decisions

### Decision: Field Mapping Strategy
**What was chosen**: Extend existing field_mapper.rs with missing PDF field names  
**Rationale**: Existing field mapping system already handles ability scores, spells, and combat stats. Adding personality traits, passive perception, hit dice, and currency fields follows established patterns.  
**Alternatives considered**: 
- Create new mapping module → Rejected: Violates Simplicity First principle
- Hardcode field names in pdf_filler → Rejected: Violates Single Responsibility principle

### Decision: Passive Perception Calculation
**What was chosen**: Add calculation method in pdf_filler.rs using formula: 10 + Wisdom modifier + (proficiency bonus if proficient)  
**Rationale**: Follows D&D 5e official rules exactly per Constitution principle III. Calculation logic belongs in PDF filler where other derived values are computed.  
**Alternatives considered**:
- Add to character_model.rs → Rejected: PDF-specific calculation, not core character data
- Create separate calculator module → Rejected: Over-engineering for single calculation

### Decision: Hit Dice Implementation  
**What was chosen**: Class-to-hit-dice mapping table in pdf_filler.rs, format as "[level]d[die_size]"  
**Rationale**: Simple lookup table approach. Single-class only per Constitution scope limitations.  
**Alternatives considered**:
- Complex class system with inheritance → Rejected: Violates Simplicity First and Scope Limitations
- External configuration file → Rejected: Increases complexity, hardcoded mapping sufficient

### Decision: Currency Field Population
**What was chosen**: Direct field mapping from equipment.currency JSON to PDF fields  
**Rationale**: Straightforward data transfer, no calculations needed. Follows existing equipment handling patterns.  
**Alternatives considered**:
- Currency conversion logic → Rejected: Not required by specification, adds complexity
- Validation of currency amounts → Rejected: Handled by existing validation system

### Decision: Personality Traits Handling
**What was chosen**: Use existing narrative_handler.rs with field mapping to PDF personality traits field  
**Rationale**: Narrative handler already processes character personality data. Just need to map to correct PDF field.  
**Alternatives considered**:
- Inline personality processing → Rejected: Violates Single Responsibility principle
- New narrative module → Rejected: Functionality already exists

### Decision: Features & Traits Handling
**What was chosen**: Direct field mapping from character features and racial traits data to `Features & Traits` PDF field  
**Rationale**: Similar to personality traits, this is straightforward text data that needs to be displayed. Follows existing narrative handling patterns.  
**Alternatives considered**:
- Separate fields for features vs traits → Rejected: PDF has single combined field
- Complex feature categorization → Rejected: Violates Simplicity First principle

## Implementation Approach

### Concrete Implementation Steps:

#### 1. **field_mapper.rs** - Add Missing Field Mappings
**Location**: In `FieldMapper::new()` method, add these entries to the `field_map` HashMap:
```rust
// Add after existing narrative fields (around line 85)
field_map.insert("passive_perception".to_string(), "Passive".to_string());
field_map.insert("hit_dice_total".to_string(), "HDTotal".to_string());
field_map.insert("hit_dice_type".to_string(), "HD".to_string());
field_map.insert("features_traits".to_string(), "Features & Traits".to_string());

// Update existing personality_traits mapping (around line 82)
// Change from "Personality" to "PersonalityTraits"
field_map.insert("personality_traits".to_string(), "PersonalityTraits".to_string());
```

#### 2. **pdf_filler.rs** - Add Field Population Logic
**Location**: In `get_field_values()` method, add these sections:

**A. Passive Perception Calculation** (add after ability scores section, around line 320):
```rust
// Passive Perception calculation
let prof_bonus = self.calculate_proficiency_bonus(character.level);
let is_perception_proficient = character_data.proficiencies
    .as_ref()
    .map(|p| p.skills.contains(&"perception".to_string()))
    .unwrap_or(false);
let passive_perception = 10 + self.calculate_modifier(abilities.wisdom) + 
    if is_perception_proficient { prof_bonus } else { 0 };
if let Some(field_name) = self.field_mapper.get_pdf_field_name("passive_perception") {
    fields.insert(field_name.clone(), passive_perception.to_string());
}
```

**B. Hit Dice Fields** (add after proficiency bonus section, around line 380):
```rust
// Hit Dice fields
let hit_die_type = match character.class.as_str() {
    "Barbarian" => "d12",
    "Fighter" | "Paladin" | "Ranger" => "d10",
    "Bard" | "Cleric" | "Druid" | "Monk" | "Rogue" | "Warlock" => "d8", 
    "Sorcerer" | "Wizard" => "d6",
    _ => "d8", // Default
};
if let Some(field_name) = self.field_mapper.get_pdf_field_name("hit_dice_total") {
    fields.insert(field_name.clone(), character.level.to_string());
}
if let Some(field_name) = self.field_mapper.get_pdf_field_name("hit_dice_type") {
    fields.insert(field_name.clone(), hit_die_type.to_string());
}
```

**C. Features & Traits Field** (add after narrative section, around line 520):
```rust
// Features & Traits field
if let Some(features_traits) = &character_data.features_traits {
    let mut combined_text = Vec::new();
    if let Some(features) = &features_traits.features {
        combined_text.extend(features.iter().cloned());
    }
    if let Some(traits) = &features_traits.traits {
        combined_text.extend(traits.iter().cloned());
    }
    if !combined_text.is_empty() {
        let features_text = combined_text.join(", ");
        let truncated = if features_text.len() > 500 {
            format!("{}...", &features_text[..497])
        } else {
            features_text
        };
        if let Some(field_name) = self.field_mapper.get_pdf_field_name("features_traits") {
            fields.insert(field_name.clone(), truncated);
        }
    }
}
```

#### 3. **character_model.rs** - Add Features & Traits Data Structure
**Location**: Add new struct and update CharacterData:
```rust
// Add after CharacterNarrative struct (around line 175)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesTraits {
    pub features: Option<Vec<String>>,
    pub traits: Option<Vec<String>>,
}

// Update CharacterData struct (around line 185)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterData {
    pub character: Character,
    pub abilities: AbilityScores,
    pub proficiencies: Option<Proficiencies>,
    pub combat: Option<Combat>,
    pub spells: Option<Spells>,
    pub equipment: Option<Equipment>,
    pub narrative: Option<CharacterNarrative>,
    pub features_traits: Option<FeaturesTraits>, // Add this line
}
```

### Implementation Order:
1. **character_model.rs** - Add FeaturesTraits struct and update CharacterData
2. **field_mapper.rs** - Add new field mappings 
3. **pdf_filler.rs** - Add field population logic in get_field_values()
4. **Build and test** - `cargo build --release`

### Risk Mitigation:
- **Field name accuracy**: Field names confirmed from clarification session
- **Data availability**: All new fields handle missing data gracefully with Option types
- **Text truncation**: Features & traits limited to 500 chars to prevent PDF overflow

## Dependencies Analysis

**No new dependencies required** - All functionality can be implemented with existing lopdf and serde_json dependencies, maintaining Constitutional principle II (Minimal Dependencies).

## Performance Impact

**Minimal performance impact expected**:
- Simple field mappings: O(1) lookups
- Passive perception calculation: Single arithmetic operation  
- Hit dice formatting: String formatting operation
- Currency field population: Direct data transfer

All operations well within Constitutional performance standards (PDF processing under 2 seconds).
