use crate::character_model::{AbilityScores, CharacterData};
use crate::errors::ValidationError;

pub struct DndValidator {
    pub allow_rule_violations: bool,
}

impl DndValidator {
    pub fn new(allow_rule_violations: bool) -> Self {
        Self {
            allow_rule_violations,
        }
    }

    pub fn validate(
        &self,
        character: &CharacterData,
    ) -> Result<Vec<ValidationError>, Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Validate ability scores
        self.validate_ability_scores(&character.abilities, &mut errors);

        // Validate character level
        self.validate_level(character.character.level, &mut errors);

        // Validate experience points if provided
        if let Some(xp) = character.character.experience_points {
            self.validate_experience_points(xp, character.character.level, &mut errors);
        }

        // Validate spells if provided
        if let Some(spells) = &character.spells {
            self.validate_spells(spells, &mut errors);
        }

        if errors.is_empty() || self.allow_rule_violations {
            Ok(errors)
        } else {
            Err(errors)
        }
    }

    fn validate_ability_scores(
        &self,
        abilities: &AbilityScores,
        errors: &mut Vec<ValidationError>,
    ) {
        let scores = [
            ("Strength", abilities.strength),
            ("Dexterity", abilities.dexterity),
            ("Constitution", abilities.constitution),
            ("Intelligence", abilities.intelligence),
            ("Wisdom", abilities.wisdom),
            ("Charisma", abilities.charisma),
        ];

        for (name, score) in scores {
            if score < 1 || score > 20 {
                errors.push(ValidationError::AbilityScore {
                    ability: name.to_string(),
                    value: score,
                    message: "must be between 1 and 20".to_string(),
                });
            }
        }
    }

    fn validate_level(&self, level: u8, errors: &mut Vec<ValidationError>) {
        if level < 1 || level > 20 {
            errors.push(ValidationError::Level {
                level,
                message: "must be between 1 and 20".to_string(),
            });
        }
    }

    fn validate_experience_points(&self, xp: u32, level: u8, errors: &mut Vec<ValidationError>) {
        let required_xp = match level {
            1 => 0,
            2 => 300,
            3 => 900,
            4 => 2700,
            5 => 6500,
            6 => 14000,
            7 => 23000,
            8 => 34000,
            9 => 48000,
            10 => 64000,
            11 => 85000,
            12 => 100000,
            13 => 120000,
            14 => 140000,
            15 => 165000,
            16 => 195000,
            17 => 225000,
            18 => 265000,
            19 => 305000,
            20 => 355000,
            _ => return,
        };

        if xp < required_xp {
            errors.push(ValidationError::ExperiencePoints {
                xp,
                level,
                required: required_xp,
            });
        }
    }

    fn validate_spells(
        &self,
        spells: &crate::character_model::Spells,
        errors: &mut Vec<ValidationError>,
    ) {
        // Validate cantrips are always prepared
        for cantrip in &spells.cantrips {
            if cantrip.level != 0 {
                errors.push(ValidationError::Spell {
                    spell: cantrip.name.clone(),
                    level: cantrip.level,
                    message: "Cantrips must be level 0".to_string(),
                });
            }
        }

        // Validate spell levels match their arrays
        let spell_arrays = [
            (&spells.first_level, 1),
            (&spells.second_level, 2),
            (&spells.third_level, 3),
            (&spells.fourth_level, 4),
            (&spells.fifth_level, 5),
            (&spells.sixth_level, 6),
            (&spells.seventh_level, 7),
            (&spells.eighth_level, 8),
            (&spells.ninth_level, 9),
        ];

        for (spell_list, expected_level) in spell_arrays {
            for spell in spell_list {
                if spell.level != expected_level {
                    errors.push(ValidationError::Spell {
                        spell: spell.name.clone(),
                        level: spell.level,
                        message: format!(
                            "has level {} but was placed in level {} array",
                            spell.level, expected_level
                        ),
                    });
                }
            }
        }
    }

    pub fn proficiency_bonus(level: u8) -> u8 {
        match level {
            1..=4 => 2,
            5..=8 => 3,
            9..=12 => 4,
            13..=16 => 5,
            17..=20 => 6,
            _ => 2,
        }
    }
}
