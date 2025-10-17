use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub class: String,
    pub level: u8,
    pub race: String,
    pub background: Option<String>,
    pub player_name: Option<String>,
    pub alignment: Option<String>,
    pub experience_points: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityScores {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

impl AbilityScores {
    pub fn modifier(&self, ability: &str) -> i8 {
        let score = match ability {
            "strength" => self.strength,
            "dexterity" => self.dexterity,
            "constitution" => self.constitution,
            "intelligence" => self.intelligence,
            "wisdom" => self.wisdom,
            "charisma" => self.charisma,
            _ => return 0,
        };
        ((score as i8) - 10) / 2
    }

    pub fn get_ability_score(&self, ability: &str) -> u8 {
        match ability {
            "strength" => self.strength,
            "dexterity" => self.dexterity,
            "constitution" => self.constitution,
            "intelligence" => self.intelligence,
            "wisdom" => self.wisdom,
            "charisma" => self.charisma,
            _ => 10, // Default to 10 if invalid ability
        }
    }
}

// D&D 5e calculation functions
pub fn get_proficiency_bonus(level: u8) -> u8 {
    match level {
        1..=4 => 2,
        5..=8 => 3,
        9..=12 => 4,
        13..=16 => 5,
        17..=20 => 6,
        _ => 2, // Default to +2 for invalid levels
    }
}

pub fn calculate_saving_throw_bonus(ability_score: u8, level: u8, is_proficient: bool) -> i8 {
    let modifier = ((ability_score as i8) - 10) / 2;
    let proficiency_bonus = if is_proficient { get_proficiency_bonus(level) as i8 } else { 0 };
    modifier + proficiency_bonus
}

pub fn calculate_skill_bonus(ability_score: u8, level: u8, is_proficient: bool) -> i8 {
    let modifier = ((ability_score as i8) - 10) / 2;
    let proficiency_bonus = if is_proficient { get_proficiency_bonus(level) as i8 } else { 0 };
    modifier + proficiency_bonus
}

// Standard D&D 5e skill-to-ability mappings
pub fn get_skill_ability(skill: &str) -> &'static str {
    match skill {
        // Strength-based
        "athletics" => "strength",
        
        // Dexterity-based
        "acrobatics" => "dexterity",
        "sleight_of_hand" => "dexterity",
        "stealth" => "dexterity",
        
        // Intelligence-based
        "arcana" => "intelligence",
        "history" => "intelligence",
        "investigation" => "intelligence",
        "nature" => "intelligence",
        "religion" => "intelligence",
        
        // Wisdom-based
        "animal_handling" => "wisdom",
        "insight" => "wisdom",
        "medicine" => "wisdom",
        "perception" => "wisdom",
        "survival" => "wisdom",
        
        // Charisma-based
        "deception" => "charisma",
        "intimidation" => "charisma",
        "performance" => "charisma",
        "persuasion" => "charisma",
        
        _ => "intelligence", // Default fallback
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skills {
    pub proficient_skills: Vec<String>,
    pub expertise_skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Combat {
    pub armor_class: Option<u8>,
    pub initiative: Option<i8>,
    pub speed: Option<u8>,
    pub hit_point_maximum: Option<u16>,
    pub current_hit_points: Option<u16>,
    pub temporary_hit_points: Option<u16>,
    pub hit_dice: Option<String>,
    pub hit_dice_total: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub level: u8,
    pub prepared: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spells {
    pub spellcasting_class: Option<String>,
    pub spellcasting_ability: Option<String>,
    pub cantrips: Vec<Spell>,
    pub first_level: Vec<Spell>,
    pub second_level: Vec<Spell>,
    pub third_level: Vec<Spell>,
    pub fourth_level: Vec<Spell>,
    pub fifth_level: Vec<Spell>,
    pub sixth_level: Vec<Spell>,
    pub seventh_level: Vec<Spell>,
    pub eighth_level: Vec<Spell>,
    pub ninth_level: Vec<Spell>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proficiencies {
    pub saving_throws: Vec<String>,
    pub skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub currency: Option<Currency>,
    pub items: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub cp: u32,
    pub sp: u32,
    pub ep: u32,
    pub gp: u32,
    pub pp: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterNarrative {
    pub personality_traits: Option<String>,
    pub ideals: Option<String>,
    pub bonds: Option<String>,
    pub flaws: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesTraits {
    pub features: Option<Vec<String>>,
    pub traits: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterData {
    pub character: Character,
    pub abilities: AbilityScores,
    pub proficiencies: Option<Proficiencies>,
    pub combat: Option<Combat>,
    pub spells: Option<Spells>,
    pub equipment: Option<Equipment>,
    pub narrative: Option<CharacterNarrative>,
    pub features_traits: Option<FeaturesTraits>,
}
