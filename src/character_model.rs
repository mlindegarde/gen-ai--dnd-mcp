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
pub struct CharacterData {
    pub character: Character,
    pub abilities: AbilityScores,
    pub proficiencies: Option<Proficiencies>,
    pub combat: Option<Combat>,
    pub spells: Option<Spells>,
    pub equipment: Option<Equipment>,
    pub narrative: Option<CharacterNarrative>,
}
