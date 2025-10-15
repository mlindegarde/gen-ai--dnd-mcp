# D&D 5e Character Sheet PDF Field Analysis

**Purpose**: Document all fillable fields in the D&D 5e character sheet PDF for implementation reference
**Source**: docs/5E_CharacterSheet_Fillable.pdf
**Date**: 2025-10-14

## Page 1: Character Information

### Basic Character Info
- `CharacterName` - Character name text field
- `ClassLevel` - Class and level text field  
- `Background` - Background text field
- `PlayerName` - Player name text field
- `Race` - Race text field
- `Alignment` - Alignment text field
- `ExperiencePoints` - Experience points number field

### Ability Scores
- `STR` - Strength score (1-20)
- `STRmod` - Strength modifier (calculated)
- `DEX` - Dexterity score (1-20) 
- `DEXmod` - Dexterity modifier (calculated)
- `CON` - Constitution score (1-20)
- `CONmod` - Constitution modifier (calculated)
- `INT` - Intelligence score (1-20)
- `INTmod` - Intelligence modifier (calculated)
- `WIS` - Wisdom score (1-20)
- `WISmod` - Wisdom modifier (calculated)
- `CHA` - Charisma score (1-20)
- `CHAmod` - Charisma modifier (calculated)

### Saving Throws
- `ST Strength` - Strength saving throw bonus
- `ST Dexterity` - Dexterity saving throw bonus
- `ST Constitution` - Constitution saving throw bonus
- `ST Intelligence` - Intelligence saving throw bonus
- `ST Wisdom` - Wisdom saving throw bonus
- `ST Charisma` - Charisma saving throw bonus
- `Check Box 11` - Strength save proficiency indicator
- `Check Box 18` - Dexterity save proficiency indicator
- `Check Box 19` - Constitution save proficiency indicator
- `Check Box 20` - Intelligence save proficiency indicator
- `Check Box 21` - Wisdom save proficiency indicator
- `Check Box 22` - Charisma save proficiency indicator

### Skills
- `Acrobatics` - Acrobatics skill bonus
- `Check Box 23` - Acrobatics proficiency indicator
- `Animal` - Animal Handling skill bonus
- `Check Box 24` - Animal Handling proficiency indicator
- `Arcana` - Arcana skill bonus
- `Check Box 25` - Arcana proficiency indicator
- `Athletics` - Athletics skill bonus
- `Check Box 26` - Athletics proficiency indicator
- `Deception` - Deception skill bonus
- `Check Box 27` - Deception proficiency indicator
- `History` - History skill bonus
- `Check Box 28` - History proficiency indicator
- `Insight` - Insight skill bonus
- `Check Box 29` - Insight proficiency indicator
- `Intimidation` - Intimidation skill bonus
- `Check Box 30` - Intimidation proficiency indicator
- `Investigation` - Investigation skill bonus
- `Check Box 31` - Investigation proficiency indicator
- `Medicine` - Medicine skill bonus
- `Check Box 32` - Medicine proficiency indicator
- `Nature` - Nature skill bonus
- `Check Box 33` - Nature proficiency indicator
- `Perception` - Perception skill bonus
- `Check Box 34` - Perception proficiency indicator
- `Performance` - Performance skill bonus
- `Check Box 35` - Performance proficiency indicator
- `Persuasion` - Persuasion skill bonus
- `Check Box 36` - Persuasion proficiency indicator
- `Religion` - Religion skill bonus
- `Check Box 37` - Religion proficiency indicator
- `SleightofHand` - Sleight of Hand skill bonus
- `Check Box 38` - Sleight of Hand proficiency indicator
- `Stealth` - Stealth skill bonus
- `Check Box 39` - Stealth proficiency indicator
- `Survival` - Survival skill bonus
- `Check Box 40` - Survival proficiency indicator

### Combat Stats
- `AC` - Armor Class
- `Initiative` - Initiative bonus
- `Speed` - Movement speed
- `HPMax` - Hit Point Maximum
- `HPCurrent` - Current Hit Points
- `HPTemp` - Temporary Hit Points
- `HD` - Hit Dice
- `HDTotal` - Hit Dice Total
- `DeathSaveSuccess` - Death save successes (checkboxes)
- `DeathSaveFailures` - Death save failures (checkboxes)

### Proficiencies & Languages
- `Proficiencies` - Other proficiencies and languages text area

## Page 2: Character Details

### Personality & Background
- `Personality` - Personality traits text area
- `Ideals` - Ideals text area
- `Bonds` - Bonds text area
- `Flaws` - Flaws text area

### Features & Traits
- `Features and Traits` - Features and traits text area

### Equipment
- `Equipment` - Equipment list text area
- `CP` - Copper pieces
- `SP` - Silver pieces
- `EP` - Electrum pieces
- `GP` - Gold pieces
- `PP` - Platinum pieces

### Attacks & Spellcasting
- `Attacks` - Attacks and spellcasting text area
- `AttacksSpellcasting` - Additional attacks text area

## Page 3: Spells

### Spellcasting Info
- `Spellcasting Class` - Spellcasting class
- `Spellcasting Ability` - Spellcasting ability
- `Spell Save DC` - Spell save DC
- `Spell Attack Bonus` - Spell attack bonus

### Cantrips (Level 0)
- `Cantrips 1` through `Cantrips 8` - Cantrip spell names
- `Check Box 314` through `Check Box 321` - Cantrip prepared indicators

### 1st Level Spells
- `Spells 1014` through `Spells 1025` - 1st level spell names (12 slots)
- `Check Box 315` through `Check Box 326` - 1st level spell prepared indicators

### 2nd Level Spells
- `Spells 1026` through `Spells 1038` - 2nd level spell names (13 slots)
- `Check Box 327` through `Check Box 339` - 2nd level spell prepared indicators

### 3rd Level Spells
- `Spells 1039` through `Spells 1051` - 3rd level spell names (13 slots)
- `Check Box 340` through `Check Box 352` - 3rd level spell prepared indicators

### 4th Level Spells
- `Spells 1052` through `Spells 1064` - 4th level spell names (13 slots)
- `Check Box 353` through `Check Box 365` - 4th level spell prepared indicators

### 5th Level Spells
- `Spells 1065` through `Spells 1074` - 5th level spell names (9 slots)
- `Check Box 366` through `Check Box 374` - 5th level spell prepared indicators

### 6th Level Spells
- `Spells 1075` through `Spells 1084` - 6th level spell names (9 slots)
- `Check Box 375` through `Check Box 383` - 6th level spell prepared indicators

### 7th Level Spells
- `Spells 1085` through `Spells 1094` - 7th level spell names (9 slots)
- `Check Box 384` through `Check Box 392` - 7th level spell prepared indicators

### 8th Level Spells
- `Spells 1095` through `Spells 1101` - 8th level spell names (7 slots)
- `Check Box 393` through `Check Box 399` - 8th level spell prepared indicators

### 9th Level Spells
- `Spells 1102` through `Spells 1108` - 9th level spell names (7 slots)
- `Check Box 400` through `Check Box 406` - 9th level spell prepared indicators

## Field Mapping Notes

- Proficiency indicators use checkbox fields with specific numbers
- Spell prepared states use checkbox fields numbered sequentially
- Text areas support multi-line content
- Number fields have validation ranges where applicable
- Some fields are calculated and should be auto-populated
