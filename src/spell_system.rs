use crate::character_model::{Spell, Spells};

pub struct SpellSystem;

impl SpellSystem {
    #[allow(dead_code)]
    pub fn organize_spells_by_level(spells: &Spells) -> Vec<(u8, Vec<&Spell>)> {
        let mut organized = Vec::new();

        if !spells.cantrips.is_empty() {
            organized.push((0, spells.cantrips.iter().collect()));
        }
        if !spells.first_level.is_empty() {
            organized.push((1, spells.first_level.iter().collect()));
        }
        if !spells.second_level.is_empty() {
            organized.push((2, spells.second_level.iter().collect()));
        }
        if !spells.third_level.is_empty() {
            organized.push((3, spells.third_level.iter().collect()));
        }
        if !spells.fourth_level.is_empty() {
            organized.push((4, spells.fourth_level.iter().collect()));
        }
        if !spells.fifth_level.is_empty() {
            organized.push((5, spells.fifth_level.iter().collect()));
        }
        if !spells.sixth_level.is_empty() {
            organized.push((6, spells.sixth_level.iter().collect()));
        }
        if !spells.seventh_level.is_empty() {
            organized.push((7, spells.seventh_level.iter().collect()));
        }
        if !spells.eighth_level.is_empty() {
            organized.push((8, spells.eighth_level.iter().collect()));
        }
        if !spells.ninth_level.is_empty() {
            organized.push((9, spells.ninth_level.iter().collect()));
        }

        organized
    }

    pub fn calculate_spell_attack_bonus(
        spellcasting_ability_modifier: i8,
        proficiency_bonus: u8,
    ) -> i8 {
        spellcasting_ability_modifier + proficiency_bonus as i8
    }

    pub fn calculate_spell_save_dc(spellcasting_ability_modifier: i8, proficiency_bonus: u8) -> u8 {
        8 + spellcasting_ability_modifier as u8 + proficiency_bonus
    }

    pub fn get_spellcasting_ability_modifier(
        ability: &str,
        abilities: &crate::character_model::AbilityScores,
    ) -> i8 {
        match ability.to_lowercase().as_str() {
            "wisdom" => abilities.modifier("wisdom"),
            "intelligence" => abilities.modifier("intelligence"),
            "charisma" => abilities.modifier("charisma"),
            _ => 0,
        }
    }
}
