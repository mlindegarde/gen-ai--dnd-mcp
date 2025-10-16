# Feature Specification: Character Sheet PDF Population (Saves, Skills, Spells)

**Feature Branch**: `004-i-want-to`  
**Created**: 2025-10-16  
**Status**: Draft  
**Input**: User description: "I want to create a new spec that focuses on populating the saving throw and skills sections on page 1 of the character sheet PDF, plus spell preparation checkboxes."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Saving Throws Population (Priority: P1)

A D&D player wants their character's saving throw bonuses automatically calculated and filled into the PDF character sheet based on their ability scores and proficiency selections.

**Why this priority**: Core combat mechanic that every character uses - essential for playable character sheet.

**Independent Test**: Can be fully tested by providing character data with ability scores and saving throw proficiencies, then verifying the PDF shows correct calculated bonuses.

**Acceptance Scenarios**:

1. **Given** a character with Strength 14 (+2 modifier) and proficiency in Strength saves, **When** the PDF is filled, **Then** the Strength saving throw shows +4 (modifier + proficiency bonus)
2. **Given** a character with Wisdom 16 (+3 modifier) but no proficiency in Wisdom saves, **When** the PDF is filled, **Then** the Wisdom saving throw shows +3 (modifier only)
3. **Given** a 5th level character, **When** the PDF is filled, **Then** all saving throws use +3 proficiency bonus in calculations

---

### User Story 2 - Skills Population with Proficiency (Priority: P2)

A D&D player wants their character's skill bonuses automatically calculated and filled into the PDF, including proficiency bonuses where applicable.

**Why this priority**: Skills are used frequently in gameplay and require complex calculations that benefit from automation.

**Independent Test**: Can be tested by providing character data with various skill proficiencies, then verifying PDF shows correct skill bonuses.

**Acceptance Scenarios**:

1. **Given** a character proficient in Athletics (Strength-based) with Strength 16 (+3), **When** the PDF is filled, **Then** Athletics shows +6 (ability modifier + proficiency bonus)
2. **Given** a character proficient in Stealth (Dexterity-based) with Dexterity 14 (+2), **When** the PDF is filled, **Then** Stealth shows +5 (ability modifier + proficiency bonus)
3. **Given** a character not proficient in History (Intelligence-based) with Intelligence 12 (+1), **When** the PDF is filled, **Then** History shows +1 (ability modifier only)

---

### User Story 3 - Proficiency Indicator Marking (Priority: P3)

A D&D player wants visual indicators (filled circles or checkmarks) on their character sheet to show which saving throws and skills they are proficient in.

**Why this priority**: Visual clarity helps players quickly identify their character's strengths during gameplay.

**Independent Test**: Can be tested by providing character data with specific proficiencies, then verifying the PDF shows appropriate visual markers.

**Acceptance Scenarios**:

1. **Given** a character proficient in Dexterity and Wisdom saving throws, **When** the PDF is filled, **Then** only those saving throw proficiency indicators are marked
2. **Given** a character proficient in Perception and Investigation skills, **When** the PDF is filled, **Then** only those skill proficiency indicators are marked

---

### User Story 4 - Spell Preparation Checkbox Marking (Priority: P2)

A D&D spellcaster wants their prepared spells visually indicated on the character sheet by marking the appropriate checkboxes next to each prepared spell.

**Why this priority**: Spell preparation tracking is essential for spellcasting classes and requires precise field mapping to function correctly.

**Independent Test**: Can be tested by providing character data with specific prepared spells for each level, then verifying the PDF shows appropriate checkbox markers.

**Acceptance Scenarios**:

1. **Given** a character with 3 prepared 1st-level spells, **When** the PDF is filled, **Then** exactly 3 checkboxes are marked in the 1st-level spell preparation area
2. **Given** a character with prepared spells across levels 1-5, **When** the PDF is filled, **Then** checkboxes are marked in the correct spell level sections
3. **Given** a character with no prepared spells of a certain level, **When** the PDF is filled, **Then** no checkboxes are marked for that spell level

---

### Edge Cases

- What happens when ability scores are below 10 (negative modifiers)?
- What happens when character data is missing required ability scores?
- What happens when more spells are prepared than available slots for a level?
- What happens when spell preparation data is missing or invalid?

## Clarifications

### Session 2025-10-16

- Q: Error Handling Strategy → A: Skip problematic calculations, show clear error indicators, complete valid calculations
- Q: Multiclass Proficiency Bonus Handling → A: Do not support multiclassing, will be added later
- Q: Custom Skill-Ability Mappings → A: Use only standard D&D 5e skill-to-ability mappings
- Q: Visual Indicator Implementation → A: Fill existing checkbox/circle fields in the PDF form
- Q: Character Level Range → A: Levels 1-20
- Q: Spell Preparation Checkbox Mapping → A: Systematic field mapping completed for spell levels 1-9, cantrips require no checkboxes

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST calculate saving throw bonuses using ability modifiers plus proficiency bonus when proficient
- **FR-002**: System MUST calculate skill bonuses using appropriate ability modifiers plus proficiency bonus when proficient
- **FR-003**: System MUST determine proficiency bonus based on character level using D&D 5e progression table (single-class characters only, levels 1-20)
- **FR-004**: System MUST populate all six saving throw fields (Strength, Dexterity, Constitution, Intelligence, Wisdom, Charisma)
- **FR-005**: System MUST populate all eighteen standard skill fields with calculated bonuses
- **FR-006**: System MUST mark proficiency indicators by filling existing checkbox/circle fields in the PDF form for saving throws and skills where character is proficient
- **FR-007**: System MUST handle negative ability modifiers correctly in calculations
- **FR-008**: System MUST validate that required ability scores are present before attempting calculations
- **FR-009**: System MUST skip calculations for missing or invalid data while displaying clear error indicators in affected fields
- **FR-010**: System MUST complete all valid calculations even when some data is problematic
- **FR-011**: System MUST NOT support multiclass characters (out of scope for this feature)
- **FR-012**: System MUST use only standard D&D 5e skill-to-ability mappings (no custom ability assignments)
- **FR-013**: System MUST mark spell preparation checkboxes for prepared spells using systematically mapped checkbox fields
- **FR-014**: System MUST support spell preparation marking for spell levels 1-9 using verified checkbox field mappings
- **FR-015**: System MUST NOT mark preparation checkboxes for cantrips (0th level spells) as they do not require preparation
- **FR-016**: System MUST handle cases where more spells are prepared than available checkbox fields by marking available fields only
- **FR-017**: System MUST skip spell preparation marking when spell data is missing or invalid while continuing other processing

### Error Handling

- System displays "ERROR" or similar indicator in PDF fields where calculations cannot be completed
- System continues processing valid data when encountering missing or invalid ability scores
- System provides clear indication of which fields contain errors versus valid calculations
- System skips spell preparation marking when spell data is invalid but continues processing other character data
- System handles overflow cases where more spells are prepared than available checkbox fields

### Key Entities

- **Character**: Contains ability scores, level, proficiencies, and prepared spells
- **Saving Throw**: Links to specific ability score, has proficiency status, displays calculated bonus
- **Skill**: Links to specific ability score, has proficiency status, displays calculated bonus
- **Proficiency Bonus**: Derived from character level using D&D 5e progression table
- **Prepared Spell**: Links to specific spell level, has preparation status, triggers checkbox marking
- **Spell Level**: Represents spell levels 1-9, each with systematically mapped checkbox fields for preparation tracking

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All saving throw bonuses are calculated correctly within 1 second of processing character data
- **SC-002**: All skill bonuses are calculated correctly for characters with proficiency selections
- **SC-003**: 100% of proficiency indicators are marked accurately on the PDF
- **SC-004**: System handles edge cases (negative modifiers, missing data) without errors or incorrect calculations
- **SC-005**: Generated PDF maintains all original formatting and layout of the character sheet template
- **SC-006**: 100% of spell preparation checkboxes are marked accurately for prepared spells across all supported levels (1-9)
- **SC-007**: System correctly handles spell preparation data validation and error cases without affecting other PDF population

## Assumptions

- Character data includes all six ability scores (Strength, Dexterity, Constitution, Intelligence, Wisdom, Charisma)
- Character level is provided for proficiency bonus calculation (levels 1-20 supported)
- Proficiency selections are clearly indicated in character data
- Standard D&D 5e skill-to-ability mappings are used (no custom ability assignments supported)
- PDF template contains identifiable form fields for all saving throws and skills
- Only single-class characters are supported (multiclassing out of scope)
- PDF template contains checkbox/circle fields for proficiency indicators
- Prepared spell data is provided with clear spell level organization (levels 1-9)
- PDF template contains systematically mapped checkbox fields for spell preparation (verified through testing)
- Cantrips (0th level spells) do not require preparation checkboxes as per D&D 5e rules
- Spell preparation checkbox field mappings have been systematically verified and documented
