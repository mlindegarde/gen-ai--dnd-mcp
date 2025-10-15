use crate::character_model::{Spell, Spells};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CasterType {
    Full,
    Half,
    Third,
    None,
}

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

    /// D&D 5e spell slot progression tables
    /// Returns spell slots by level for full casters (Wizard, Sorcerer, Cleric, Druid, Bard)
    fn get_full_caster_spell_slots(level: u8) -> HashMap<u8, u8> {
        let mut slots = HashMap::new();
        match level {
            1 => { slots.insert(1, 2); }
            2 => { slots.insert(1, 3); }
            3 => { slots.insert(1, 4); slots.insert(2, 2); }
            4 => { slots.insert(1, 4); slots.insert(2, 3); }
            5 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 2); }
            6 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); }
            7 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 1); }
            8 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 2); }
            9 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 1); }
            10 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 2); }
            11 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 2); slots.insert(6, 1); }
            12 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 2); slots.insert(6, 1); }
            13 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 2); slots.insert(6, 1); slots.insert(7, 1); }
            14 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 2); slots.insert(6, 1); slots.insert(7, 1); }
            15 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 2); slots.insert(6, 1); slots.insert(7, 1); slots.insert(8, 1); }
            16 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 2); slots.insert(6, 1); slots.insert(7, 1); slots.insert(8, 1); }
            17 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 2); slots.insert(6, 1); slots.insert(7, 1); slots.insert(8, 1); slots.insert(9, 1); }
            18 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 3); slots.insert(6, 1); slots.insert(7, 1); slots.insert(8, 1); slots.insert(9, 1); }
            19 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 3); slots.insert(6, 2); slots.insert(7, 1); slots.insert(8, 1); slots.insert(9, 1); }
            20 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 3); slots.insert(6, 2); slots.insert(7, 2); slots.insert(8, 1); slots.insert(9, 1); }
            _ => {}
        }
        slots
    }

    /// Half caster progression (Paladin, Ranger)
    fn get_half_caster_spell_slots(level: u8) -> HashMap<u8, u8> {
        let mut slots = HashMap::new();
        match level {
            2 => { slots.insert(1, 2); }
            3 => { slots.insert(1, 3); }
            4 => { slots.insert(1, 3); }
            5 => { slots.insert(1, 4); slots.insert(2, 2); }
            6 => { slots.insert(1, 4); slots.insert(2, 2); }
            7 => { slots.insert(1, 4); slots.insert(2, 3); }
            8 => { slots.insert(1, 4); slots.insert(2, 3); }
            9 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 2); }
            10 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 2); }
            11 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); }
            12 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); }
            13 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 1); }
            14 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 1); }
            15 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 2); }
            16 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 2); }
            17 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 1); }
            18 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 1); }
            19 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 2); }
            20 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 3); slots.insert(5, 2); }
            _ => {}
        }
        slots
    }

    /// Third caster progression (Eldritch Knight Fighter, Arcane Trickster Rogue)
    fn get_third_caster_spell_slots(level: u8) -> HashMap<u8, u8> {
        let mut slots = HashMap::new();
        match level {
            3 => { slots.insert(1, 2); }
            4 => { slots.insert(1, 3); }
            5 => { slots.insert(1, 3); }
            6 => { slots.insert(1, 3); }
            7 => { slots.insert(1, 4); slots.insert(2, 2); }
            8 => { slots.insert(1, 4); slots.insert(2, 2); }
            9 => { slots.insert(1, 4); slots.insert(2, 2); }
            10 => { slots.insert(1, 4); slots.insert(2, 3); }
            11 => { slots.insert(1, 4); slots.insert(2, 3); }
            12 => { slots.insert(1, 4); slots.insert(2, 3); }
            13 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 2); }
            14 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 2); }
            15 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 2); }
            16 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); }
            17 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); }
            18 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); }
            19 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 1); }
            20 => { slots.insert(1, 4); slots.insert(2, 3); slots.insert(3, 3); slots.insert(4, 1); }
            _ => {}
        }
        slots
    }

    /// Determine caster type from character class
    pub fn get_caster_type(class: &str) -> CasterType {
        match class.to_lowercase().as_str() {
            "wizard" | "sorcerer" | "cleric" | "druid" | "bard" => CasterType::Full,
            "paladin" | "ranger" => CasterType::Half,
            "fighter" | "rogue" => CasterType::Third, // Assumes Eldritch Knight/Arcane Trickster
            _ => CasterType::None,
        }
    }

    /// Calculate spell slots for a single-class character
    pub fn calculate_spell_slots(character: &crate::character_model::CharacterData) -> HashMap<String, u8> {
        let mut spell_slots = HashMap::new();
        
        // Determine caster type from class
        let caster_type = Self::get_caster_type(&character.character.class);
        
        // Log warning for unknown classes that default to non-caster
        if caster_type == CasterType::None && !Self::is_known_non_caster(&character.character.class) {
            eprintln!("Warning: Unknown character class '{}' treated as non-spellcaster", character.character.class);
        }
        
        // Get spell slots from appropriate progression table
        let slots = Self::get_spell_slots_for_single_class(character.character.level, caster_type);
        
        // Convert to logical field names for PDF mapping
        for level in 1..=9 {
            if let Some(count) = slots.get(&level) {
                spell_slots.insert(format!("spell_slots_{}", level), *count);
            }
        }
        
        spell_slots
    }

    /// Check if a class is a known non-spellcaster
    fn is_known_non_caster(class: &str) -> bool {
        matches!(class.to_lowercase().as_str(), 
            "barbarian" | "monk" | "fighter" | "rogue" | "artificer" | "blood hunter"
        )
    }

    /// Get spell slots for single-class character based on level and caster type
    pub fn get_spell_slots_for_single_class(level: u8, caster_type: CasterType) -> HashMap<u8, u8> {
        // Validate level range
        if level < 1 || level > 20 {
            eprintln!("Warning: Character level {} is outside valid range (1-20), clamping", level);
        }
        
        let clamped_level = level.clamp(1, 20);
        
        let slots = match caster_type {
            CasterType::Full => Self::get_full_caster_spell_slots(clamped_level),
            CasterType::Half => Self::get_half_caster_spell_slots(clamped_level),
            CasterType::Third => Self::get_third_caster_spell_slots(clamped_level),
            CasterType::None => HashMap::new(), // Non-casters have no spell slots
        };
        
        // Log spell slot calculation for debugging
        if !slots.is_empty() {
            let slot_summary: Vec<String> = (1..=9)
                .filter_map(|level| slots.get(&level).map(|count| format!("{}:{}", level, count)))
                .collect();
            eprintln!("Spell slots calculated: {} (level {}, {:?})", slot_summary.join("/"), clamped_level, caster_type);
        }
        
        slots
    }
}
