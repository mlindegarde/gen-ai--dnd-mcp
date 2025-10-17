# Feature Specification: Character Sheet Field Population Bug Fixes

**Feature Branch**: `005-fix-bugs-i`  
**Created**: 2025-10-17  
**Status**: Draft  
**Input**: User description: "fix bugs - I want to addresss the issue with the Personality Traits not getting populated, passive wisdome (perception) not being populatedand the CP, SP, EP, GP, and PP not being populated. Also include the Features & Traits field."

## Clarifications

### Session 2025-10-17

- Q: Hit Dice field structure → A: Two separate fields: `HDTotal` and `HD` both populated per D&D 5e rules
- Q: Passive Perception field name → A: Field name is `Passive`
- Q: Personality Traits field name → A: `PersonalityTraits`
- Q: Currency field names → A: `CP`, `SP`, `EP`, `GP`, `PP`
- Q: Features & Traits field addition → A: Include `Features & Traits` field for character features and racial traits

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Character Narrative Display (Priority: P1)

When a user creates a D&D character sheet with personality traits, the personality traits text should appear in the appropriate field on the generated PDF character sheet.

**Why this priority**: Personality traits are core to D&D character identity and roleplay. Missing this information makes the character sheet incomplete for gameplay.

**Independent Test**: Can be fully tested by providing character data with personality traits and verifying the PDF contains the traits text in the correct field.

**Acceptance Scenarios**:

1. **Given** a character with personality traits "I am brave and loyal", **When** generating a character sheet PDF, **Then** the `PersonalityTraits` field displays "I am brave and loyal"
2. **Given** a character with no personality traits specified, **When** generating a character sheet PDF, **Then** the `PersonalityTraits` field remains empty

---

### User Story 2 - Passive Perception Calculation (Priority: P2)

When a user creates a D&D character sheet, the passive Wisdom (Perception) score should be automatically calculated and displayed based on the character's Wisdom modifier and proficiency bonus.

**Why this priority**: Passive Perception is frequently referenced during gameplay for detecting hidden threats and secrets. Missing this calculation forces manual computation.

**Independent Test**: Can be fully tested by providing character data with Wisdom score and perception proficiency, then verifying the PDF shows the correct passive perception value (10 + Wisdom modifier + proficiency bonus if proficient).

**Acceptance Scenarios**:

1. **Given** a character with Wisdom 14 (+2 modifier) and proficiency in Perception, **When** generating a character sheet PDF, **Then** the `Passive` field displays the calculated value (14: 10 + 2 Wis mod + 2 prof bonus)
2. **Given** a character with Wisdom 10 (+0 modifier) and no perception proficiency, **When** generating a character sheet PDF, **Then** the `Passive` field displays "10"

---

### User Story 3 - Hit Dice Field Population (Priority: P3)

When a user creates a D&D character sheet, the Hit Dice fields should display the character's hit dice information based on their class and level. The `HDTotal` field shows the total number of hit dice available, and the `HD` field shows the hit dice type.

**Why this priority**: Hit Dice are essential for short rest healing and tracking character resources during gameplay. Missing this information forces manual calculation and reference lookup.

**Independent Test**: Can be fully tested by providing character data with class and level, then verifying the PDF shows the correct hit dice values in both fields.

**Acceptance Scenarios**:

1. **Given** a 3rd level Fighter, **When** generating a character sheet PDF, **Then** the `HDTotal` field displays "3" and the `HD` field displays "d10"
2. **Given** a 5th level Wizard, **When** generating a character sheet PDF, **Then** the `HDTotal` field displays "5" and the `HD` field displays "d6"

---

### User Story 4 - Currency Field Population (Priority: P4)

When a user creates a D&D character sheet with currency amounts (copper, silver, electrum, gold, platinum pieces), each currency type should appear in its respective field on the generated PDF.

**Why this priority**: Currency tracking is essential for equipment purchases and campaign progression. Missing currency information prevents proper resource management.

**Independent Test**: Can be fully tested by providing character data with various currency amounts and verifying each currency type appears in its corresponding PDF field.

**Acceptance Scenarios**:

1. **Given** a character with 50 CP, 25 SP, 10 EP, 100 GP, and 5 PP, **When** generating a character sheet PDF, **Then** the `CP`, `SP`, `EP`, `GP`, and `PP` fields display the correct amounts
2. **Given** a character with only gold pieces specified, **When** generating a character sheet PDF, **Then** the `GP` field shows the amount and other currency fields remain empty

---

### User Story 5 - Features & Traits Field Population (Priority: P5)

When a user creates a D&D character sheet with character features and racial traits, this information should appear in the `Features & Traits` field on the generated PDF character sheet.

**Why this priority**: Character features and racial traits define unique abilities and characteristics that are essential for gameplay mechanics and character identity. Missing this information makes the character sheet incomplete for rules reference.

**Independent Test**: Can be fully tested by providing character data with features and traits, then verifying the PDF contains the information in the correct field.

**Acceptance Scenarios**:

1. **Given** a character with features "Darkvision, Fey Ancestry" and traits "Extra Language", **When** generating a character sheet PDF, **Then** the `Features & Traits` field displays "Darkvision, Fey Ancestry, Extra Language"
2. **Given** a character with no features or traits specified, **When** generating a character sheet PDF, **Then** the `Features & Traits` field remains empty

---

### Edge Cases

- What happens when personality traits text exceeds the PDF field character limit?
- How does the system handle invalid Wisdom scores for passive perception calculation?
- What happens when currency amounts are negative or exceed reasonable limits?
- What happens when features and traits text exceeds the PDF field character limit?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST populate the `PersonalityTraits` PDF field when character narrative data includes personality traits
- **FR-002**: System MUST calculate and display passive Wisdom (Perception) in the `Passive` field using the formula: 10 + Wisdom modifier + (proficiency bonus if proficient in Perception)
- **FR-003**: System MUST populate the hit dice fields based on single-class character class and level using D&D 5e hit dice mapping: `HDTotal` field shows character level, `HD` field shows die type (Fighter=d10, Wizard=d6, etc.)
- **FR-004**: System MUST populate individual currency fields (`CP`, `SP`, `EP`, `GP`, `PP`) when character equipment data includes currency amounts
- **FR-005**: System MUST handle missing narrative data gracefully by leaving personality traits field empty
- **FR-006**: System MUST handle missing currency data gracefully by leaving currency fields empty
- **FR-007**: System MUST truncate personality traits text if it exceeds PDF field capacity
- **FR-008**: System MUST validate Wisdom scores are within valid D&D range (1-30) before calculating passive perception
- **FR-009**: System MUST populate `HDTotal` with character level and `HD` with appropriate die size based on class
- **FR-010**: System MUST populate the `Features & Traits` PDF field when character data includes features and racial traits
- **FR-011**: System MUST handle missing features and traits data gracefully by leaving the field empty
- **FR-012**: System MUST truncate features and traits text if it exceeds PDF field capacity
- **FR-013**: System MUST support single-class characters only (multiclass support excluded from this feature)

### Key Entities *(include if feature involves data)*

- **Character Narrative**: Contains personality traits, ideals, bonds, and flaws text
- **Ability Scores**: Contains Wisdom score used for passive perception calculation
- **Proficiencies**: Contains skill proficiencies including Perception
- **Character Class**: Contains class name used for hit dice determination
- **Features & Traits**: Contains character features and racial traits information
- **Currency**: Contains amounts for each coin type (CP, SP, EP, GP, PP)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of character sheets with personality traits data display the traits text in the `PersonalityTraits` PDF field
- **SC-002**: 100% of character sheets display correctly calculated passive Wisdom (Perception) values
- **SC-003**: 100% of character sheets display correctly formatted hit dice based on class and level
- **SC-004**: 100% of character sheets with currency data display all specified currency amounts in their respective PDF fields (`CP`, `SP`, `EP`, `GP`, `PP`)
- **SC-005**: 100% of character sheets with features and traits data display the information in the `Features & Traits` PDF field
- **SC-006**: Character sheet generation completes successfully for all test cases without errors
- **SC-007**: PDF field population accuracy reaches 100% for all five bug areas (personality traits, passive perception, hit dice, currency, features & traits)

## Assumptions

- Character data follows the existing JSON schema structure
- PDF template contains the necessary fields for personality traits, passive perception, hit dice, currency, and features & traits
- Field mapping system exists and needs updates to include these fields
- Passive perception calculation follows standard D&D 5e rules (10 + Wisdom modifier + proficiency bonus if applicable)
- Hit dice mapping follows standard D&D 5e class progression for single-class characters only
- Features and traits information comes from character race and class features
- Multiclass characters and expert skills are explicitly excluded from this feature scope
