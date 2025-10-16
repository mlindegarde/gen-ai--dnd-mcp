# PDF Field Mapping: Saving Throws and Skills

Based on debug-sheet.pdf values matching first 2-4 characters of field names.

## Saving Throw Fields

### Saving Throw Bonuses (Text Fields)
```rust
// Saving throw bonus values (debug shows "ST S", "ST D", etc.)
field_map.insert("strength_save".to_string(), "ST Strength".to_string());
field_map.insert("dexterity_save".to_string(), "ST Dexterity".to_string());
field_map.insert("constitution_save".to_string(), "ST Constitution".to_string());
field_map.insert("intelligence_save".to_string(), "ST Intelligence".to_string());
field_map.insert("wisdom_save".to_string(), "ST Wisdom".to_string());
field_map.insert("charisma_save".to_string(), "ST Charisma".to_string());
```

### Saving Throw Proficiency Indicators (Checkbox Fields)
```rust
// Proficiency checkboxes for saving throws (debug shows "Che", "Chec", etc.)
field_map.insert("strength_save_prof".to_string(), "Check Box 11".to_string());
field_map.insert("dexterity_save_prof".to_string(), "Check Box 18".to_string());
field_map.insert("constitution_save_prof".to_string(), "Check Box 19".to_string());
field_map.insert("intelligence_save_prof".to_string(), "Check Box 20".to_string());
field_map.insert("wisdom_save_prof".to_string(), "Check Box 21".to_string());
field_map.insert("charisma_save_prof".to_string(), "Check Box 22".to_string());
```

## Skill Fields

### Skill Bonuses (Text Fields) - Already Exist
The skill bonus text fields are already mapped in the existing code:
- "acrobatics" → "Acrobatics"
- "athletics" → "Athletics"
- etc.

### Skill Proficiency Indicators (Checkbox Fields)
```rust
// Proficiency checkboxes for skills (following existing pattern)
field_map.insert("acrobatics_prof".to_string(), "Check Box 23".to_string());
field_map.insert("animal_handling_prof".to_string(), "Check Box 24".to_string());
field_map.insert("arcana_prof".to_string(), "Check Box 25".to_string());
field_map.insert("athletics_prof".to_string(), "Check Box 26".to_string());
field_map.insert("deception_prof".to_string(), "Check Box 27".to_string());
field_map.insert("history_prof".to_string(), "Check Box 28".to_string());
field_map.insert("insight_prof".to_string(), "Check Box 29".to_string());
field_map.insert("intimidation_prof".to_string(), "Check Box 30".to_string());
field_map.insert("investigation_prof".to_string(), "Check Box 31".to_string());
field_map.insert("medicine_prof".to_string(), "Check Box 32".to_string());
field_map.insert("nature_prof".to_string(), "Check Box 33".to_string());
field_map.insert("perception_prof".to_string(), "Check Box 34".to_string());
field_map.insert("performance_prof".to_string(), "Check Box 35".to_string());
field_map.insert("persuasion_prof".to_string(), "Check Box 36".to_string());
field_map.insert("religion_prof".to_string(), "Check Box 37".to_string());
field_map.insert("sleight_of_hand_prof".to_string(), "Check Box 38".to_string());
field_map.insert("stealth_prof".to_string(), "Check Box 39".to_string());
field_map.insert("survival_prof".to_string(), "Check Box 40".to_string());
```

### Skill Expertise Indicators (Checkbox Fields)
```rust
// Expertise checkboxes for skills (double proficiency)
field_map.insert("acrobatics_exp".to_string(), "Check Box 41".to_string());
field_map.insert("animal_handling_exp".to_string(), "Check Box 42".to_string());
field_map.insert("arcana_exp".to_string(), "Check Box 43".to_string());
field_map.insert("athletics_exp".to_string(), "Check Box 44".to_string());
field_map.insert("deception_exp".to_string(), "Check Box 45".to_string());
field_map.insert("history_exp".to_string(), "Check Box 46".to_string());
field_map.insert("insight_exp".to_string(), "Check Box 47".to_string());
field_map.insert("intimidation_exp".to_string(), "Check Box 48".to_string());
field_map.insert("investigation_exp".to_string(), "Check Box 49".to_string());
field_map.insert("medicine_exp".to_string(), "Check Box 50".to_string());
field_map.insert("nature_exp".to_string(), "Check Box 51".to_string());
field_map.insert("perception_exp".to_string(), "Check Box 52".to_string());
field_map.insert("performance_exp".to_string(), "Check Box 53".to_string());
field_map.insert("persuasion_exp".to_string(), "Check Box 54".to_string());
field_map.insert("religion_exp".to_string(), "Check Box 55".to_string());
field_map.insert("sleight_of_hand_exp".to_string(), "Check Box 56".to_string());
field_map.insert("stealth_exp".to_string(), "Check Box 57".to_string());
field_map.insert("survival_exp".to_string(), "Check Box 58".to_string());
```

## Notes

1. **Checkbox Numbering**: The checkbox numbers are estimated based on the existing pattern. The actual numbers should be verified from the debug PDF.

2. **Saving Throw Field Names**: Following the "ST" prefix pattern commonly used in D&D PDFs.

3. **Existing Skills**: The skill bonus text fields already exist in field_mapper.rs, so we only need to add proficiency and expertise checkboxes.

4. **Verification Needed**: These field names need to be verified against the actual debug-sheet.pdf values you mentioned.

## Action Required

Please verify these field names against the debug PDF and provide corrections for any that don't match the actual field names shown in the PDF values.
