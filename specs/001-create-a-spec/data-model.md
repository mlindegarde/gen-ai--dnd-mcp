# Data Model: MCP Server with PDF Form Filling Tool

**Date**: 2025-10-14  
**Source**: Extracted from feature specification and D&D 5e rules

## Core Entities

### Character
Primary entity representing a D&D 5e character with all attributes needed for character sheet population.

**Fields**:
- `name: String` - Character name
- `class: String` - Character class (Fighter, Wizard, etc.)
- `level: u8` - Character level (1-20)
- `race: String` - Character race
- `background: String` - Character background
- `alignment: String` - Character alignment
- `experience_points: u32` - Current XP

**Validation Rules**:
- Level must be 1-20
- Experience points must match level requirements
- Class, race, background must be valid D&D 5e options

### AbilityScores
Core ability scores that drive most character mechanics.

**Fields**:
- `strength: u8` - Strength score (1-20)
- `dexterity: u8` - Dexterity score (1-20)
- `constitution: u8` - Constitution score (1-20)
- `intelligence: u8` - Intelligence score (1-20)
- `wisdom: u8` - Wisdom score (1-20)
- `charisma: u8` - Charisma score (1-20)

**Derived Fields** (calculated if not provided):
- `strength_modifier: i8` - (strength - 10) / 2
- `dexterity_modifier: i8` - (dexterity - 10) / 2
- `constitution_modifier: i8` - (constitution - 10) / 2
- `intelligence_modifier: i8` - (intelligence - 10) / 2
- `wisdom_modifier: i8` - (wisdom - 10) / 2
- `charisma_modifier: i8` - (charisma - 10) / 2

**Validation Rules**:
- All ability scores must be 1-20
- Modifiers must match formula: (score - 10) / 2 (rounded down)

### SavingThrows
Character's saving throw bonuses with proficiency tracking.

**Fields**:
- `strength_save: i8` - Strength saving throw bonus
- `dexterity_save: i8` - Dexterity saving throw bonus
- `constitution_save: i8` - Constitution saving throw bonus
- `intelligence_save: i8` - Intelligence saving throw bonus
- `wisdom_save: i8` - Wisdom saving throw bonus
- `charisma_save: i8` - Charisma saving throw bonus
- `strength_proficient: bool` - Proficiency in Strength saves
- `dexterity_proficient: bool` - Proficiency in Dexterity saves
- `constitution_proficient: bool` - Proficiency in Constitution saves
- `intelligence_proficient: bool` - Proficiency in Intelligence saves
- `wisdom_proficient: bool` - Proficiency in Wisdom saves
- `charisma_proficient: bool` - Proficiency in Charisma saves

**Derived Calculation** (if not provided):
- Base: ability_modifier + proficiency_bonus (if proficient)

### Skills
Character's skill bonuses and proficiencies with proficiency indicators.

**Fields**:
- `acrobatics: i8` - Acrobatics skill bonus (Dex-based)
- `animal_handling: i8` - Animal Handling skill bonus (Wis-based)
- `arcana: i8` - Arcana skill bonus (Int-based)
- `athletics: i8` - Athletics skill bonus (Str-based)
- `deception: i8` - Deception skill bonus (Cha-based)
- `history: i8` - History skill bonus (Int-based)
- `insight: i8` - Insight skill bonus (Wis-based)
- `intimidation: i8` - Intimidation skill bonus (Cha-based)
- `investigation: i8` - Investigation skill bonus (Int-based)
- `medicine: i8` - Medicine skill bonus (Wis-based)
- `nature: i8` - Nature skill bonus (Int-based)
- `perception: i8` - Perception skill bonus (Wis-based)
- `performance: i8` - Performance skill bonus (Cha-based)
- `persuasion: i8` - Persuasion skill bonus (Cha-based)
- `religion: i8` - Religion skill bonus (Int-based)
- `sleight_of_hand: i8` - Sleight of Hand skill bonus (Dex-based)
- `stealth: i8` - Stealth skill bonus (Dex-based)
- `survival: i8` - Survival skill bonus (Wis-based)

**Proficiency Fields**:
- `acrobatics_proficient: bool` - Proficiency indicator
- `animal_handling_proficient: bool` - Proficiency indicator
- `arcana_proficient: bool` - Proficiency indicator
- `athletics_proficient: bool` - Proficiency indicator
- `deception_proficient: bool` - Proficiency indicator
- `history_proficient: bool` - Proficiency indicator
- `insight_proficient: bool` - Proficiency indicator
- `intimidation_proficient: bool` - Proficiency indicator
- `investigation_proficient: bool` - Proficiency indicator
- `medicine_proficient: bool` - Proficiency indicator
- `nature_proficient: bool` - Proficiency indicator
- `perception_proficient: bool` - Proficiency indicator
- `performance_proficient: bool` - Proficiency indicator
- `persuasion_proficient: bool` - Proficiency indicator
- `religion_proficient: bool` - Proficiency indicator
- `sleight_of_hand_proficient: bool` - Proficiency indicator
- `stealth_proficient: bool` - Proficiency indicator
- `survival_proficient: bool` - Proficiency indicator

**Derived Calculation** (if not provided):
- Base: ability_modifier + proficiency_bonus (if proficient) + expertise_bonus (if expertise)

### Combat
Combat-related statistics and attributes.

**Fields**:
- `armor_class: u8` - Armor Class
- `initiative: i8` - Initiative bonus
- `speed: u8` - Movement speed in feet
- `hit_point_maximum: u16` - Maximum hit points
- `current_hit_points: u16` - Current hit points
- `temporary_hit_points: u16` - Temporary hit points
- `hit_dice: String` - Hit dice (e.g., "1d10")
- `death_saves_successes: u8` - Death save successes (0-3)
- `death_saves_failures: u8` - Death save failures (0-3)

**Validation Rules**:
- Current HP cannot exceed maximum HP + temporary HP
- Death saves must be 0-3
- Initiative typically equals Dex modifier

### Spells
Character's spell collection organized by level with prepared state tracking.

**Fields**:
- `cantrips: Vec<Spell>` - Level 0 spells (cantrips)
- `first_level: Vec<Spell>` - 1st level spells
- `second_level: Vec<Spell>` - 2nd level spells
- `third_level: Vec<Spell>` - 3rd level spells
- `fourth_level: Vec<Spell>` - 4th level spells
- `fifth_level: Vec<Spell>` - 5th level spells
- `sixth_level: Vec<Spell>` - 6th level spells
- `seventh_level: Vec<Spell>` - 7th level spells
- `eighth_level: Vec<Spell>` - 8th level spells
- `ninth_level: Vec<Spell>` - 9th level spells

### Spell
Individual spell with name, level, and prepared state.

**Fields**:
- `name: String` - Spell name
- `level: u8` - Spell level (0-9)
- `prepared: bool` - Whether spell is prepared (defaults to false)

**Validation Rules**:
- Level must be 0-9
- Cantrips (level 0) are always considered prepared
- Prepared state affects PDF circle marking

### Spellcasting
Spellcasting mechanics and derived values.

**Fields**:
- `spellcasting_ability: String` - Primary spellcasting ability ("Wisdom", "Intelligence", or "Charisma")
- `spell_attack_bonus: i8` - Spell attack roll bonus
- `spell_save_dc: u8` - Spell save difficulty class

**Derived Calculation** (if not provided):
- Spell attack bonus = spellcasting ability modifier + proficiency bonus
- Spell save DC = 8 + spellcasting ability modifier + proficiency bonus

### Proficiencies
Character proficiencies in various categories.

**Fields**:
- `armor: Vec<String>` - Armor proficiencies (e.g., "Light armor", "Shields")
- `weapons: Vec<String>` - Weapon proficiencies (e.g., "Simple weapons", "Longswords")
- `tools: Vec<String>` - Tool proficiencies (e.g., "Thieves' tools", "Smith's tools")
- `languages: Vec<String>` - Known languages (e.g., "Common", "Elvish")

### Equipment
Character's equipment, weapons, and gear.

**Fields**:
- `weapons: Vec<Weapon>` - List of weapons
- `armor: Option<Armor>` - Equipped armor
- `shield: Option<Shield>` - Equipped shield
- `equipment: Vec<String>` - Other equipment items
- `currency: Currency` - Character's money

### Weapon
Individual weapon with attack and damage information.

**Fields**:
- `name: String` - Weapon name
- `attack_bonus: i8` - Attack roll bonus
- `damage: String` - Damage dice and type (e.g., "1d8+3 slashing")

### Currency
Character's monetary wealth.

**Fields**:
- `copper: u32` - Copper pieces
- `silver: u32` - Silver pieces
- `electrum: u32` - Electrum pieces
- `gold: u32` - Gold pieces
- `platinum: u32` - Platinum pieces

### CharacterNarrative
Character's personality, backstory, and descriptive elements.

**Fields**:
- `personality_traits: String` - Character personality traits
- `ideals: String` - Character ideals and motivations
- `bonds: String` - Character bonds and connections
- `flaws: String` - Character flaws and weaknesses
- `character_appearance: String` - Physical description
- `additional_features_traits: String` - Additional features and traits
- `treasure: String` - Notable treasure and possessions
- `character_backstory: String` - Character background story

## Relationships

- Character **has one** AbilityScores
- Character **has one** SavingThrows
- Character **has one** Skills
- Character **has one** Combat
- Character **has one** Spells
- Character **has one** Spellcasting
- Character **has one** Proficiencies
- Character **has one** Equipment
- Character **has one** CharacterNarrative
- Equipment **has many** Weapons
- Equipment **has one** Currency
- Spells **has many** Spell (organized by level)

## State Transitions

### Character Creation Flow
1. **Draft** → Basic info entered (name, class, race)
2. **Abilities Set** → Ability scores assigned
3. **Skills Calculated** → Derived values computed
4. **Spells Added** → Spell lists populated with prepared states
5. **Equipment Added** → Gear and weapons assigned
6. **Narrative Complete** → Personality and backstory filled
7. **Complete** → Ready for PDF generation

### Validation States
- **Valid** → All D&D 5e rules satisfied
- **Invalid** → Rule violations present, requires override to proceed
- **Incomplete** → Missing required fields for PDF generation

## JSON Schema Structure

```json
{
  "character": {
    "name": "string",
    "class": "string",
    "level": "number",
    "race": "string",
    "background": "string",
    "alignment": "string",
    "experience_points": "number"
  },
  "abilities": {
    "strength": "number",
    "dexterity": "number",
    "constitution": "number",
    "intelligence": "number",
    "wisdom": "number",
    "charisma": "number"
  },
  "skills": {
    "athletics": "number",
    "athletics_proficient": "boolean",
    "acrobatics": "number",
    "acrobatics_proficient": "boolean"
    // ... all 18 skills with proficiency flags
  },
  "spells": {
    "cantrips": [
      {
        "name": "string",
        "level": 0,
        "prepared": "boolean"
      }
    ],
    "first_level": [
      {
        "name": "string", 
        "level": 1,
        "prepared": "boolean"
      }
    ]
    // ... through ninth_level
  },
  "spellcasting": {
    "spellcasting_ability": "string",
    "spell_attack_bonus": "number",
    "spell_save_dc": "number"
  },
  "proficiencies": {
    "armor": ["string"],
    "weapons": ["string"],
    "tools": ["string"],
    "languages": ["string"]
  },
  "combat": {
    "armor_class": "number",
    "initiative": "number",
    "speed": "number",
    "hit_point_maximum": "number",
    "current_hit_points": "number"
  },
  "equipment": {
    "weapons": [
      {
        "name": "string",
        "attack_bonus": "number",
        "damage": "string"
      }
    ],
    "currency": {
      "gold": "number",
      "silver": "number",
      "copper": "number"
    }
  },
  "narrative": {
    "personality_traits": "string",
    "ideals": "string",
    "bonds": "string",
    "flaws": "string",
    "character_appearance": "string",
    "additional_features_traits": "string",
    "treasure": "string",
    "character_backstory": "string"
  }
}
```
