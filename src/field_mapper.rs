use std::collections::HashMap;

pub struct FieldMapper {
    field_map: HashMap<String, String>,
}

impl FieldMapper {
    pub fn new() -> Self {
        let mut field_map = HashMap::new();

        // Basic character info
        field_map.insert("character_name".to_string(), "CharacterName".to_string());
        field_map.insert("class_level".to_string(), "ClassLevel".to_string());
        field_map.insert("background".to_string(), "Background".to_string());
        field_map.insert("player_name".to_string(), "PlayerName".to_string());
        field_map.insert("race".to_string(), "Race".to_string());
        field_map.insert("alignment".to_string(), "Alignment".to_string());
        field_map.insert(
            "experience_points".to_string(),
            "ExperiencePoints".to_string(),
        );

        // Ability scores
        field_map.insert("strength".to_string(), "STR".to_string());
        field_map.insert("strength_modifier".to_string(), "STRmod".to_string());
        field_map.insert("dexterity".to_string(), "DEX".to_string());
        field_map.insert("dexterity_modifier".to_string(), "DEXmod".to_string());
        field_map.insert("constitution".to_string(), "CON".to_string());
        field_map.insert("constitution_modifier".to_string(), "CONmod".to_string());
        field_map.insert("intelligence".to_string(), "INT".to_string());
        field_map.insert("intelligence_modifier".to_string(), "INTmod".to_string());
        field_map.insert("wisdom".to_string(), "WIS".to_string());
        field_map.insert("wisdom_modifier".to_string(), "WISmod".to_string());
        field_map.insert("charisma".to_string(), "CHA".to_string());
        field_map.insert("charisma_modifier".to_string(), "CHAmod".to_string());

        // Combat stats
        field_map.insert("armor_class".to_string(), "AC".to_string());
        field_map.insert("initiative".to_string(), "Initiative".to_string());
        field_map.insert("speed".to_string(), "Speed".to_string());
        field_map.insert("hit_point_maximum".to_string(), "HPMax".to_string());
        field_map.insert("current_hit_points".to_string(), "HPCurrent".to_string());
        field_map.insert("temporary_hit_points".to_string(), "HPTemp".to_string());

        // Skills
        field_map.insert("acrobatics".to_string(), "Acrobatics".to_string());
        field_map.insert("animal_handling".to_string(), "Animal".to_string());
        field_map.insert("arcana".to_string(), "Arcana".to_string());
        field_map.insert("athletics".to_string(), "Athletics".to_string());
        field_map.insert("deception".to_string(), "Deception".to_string());
        field_map.insert("history".to_string(), "History".to_string());
        field_map.insert("insight".to_string(), "Insight".to_string());
        field_map.insert("intimidation".to_string(), "Intimidation".to_string());
        field_map.insert("investigation".to_string(), "Investigation".to_string());
        field_map.insert("medicine".to_string(), "Medicine".to_string());
        field_map.insert("nature".to_string(), "Nature".to_string());
        field_map.insert("perception".to_string(), "Perception".to_string());
        field_map.insert("performance".to_string(), "Performance".to_string());
        field_map.insert("persuasion".to_string(), "Persuasion".to_string());
        field_map.insert("religion".to_string(), "Religion".to_string());
        field_map.insert("sleight_of_hand".to_string(), "SleightofHand".to_string());
        field_map.insert("stealth".to_string(), "Stealth".to_string());
        field_map.insert("survival".to_string(), "Survival".to_string());

        // Narrative fields
        field_map.insert("personality_traits".to_string(), "Personality".to_string());
        field_map.insert("ideals".to_string(), "Ideals".to_string());
        field_map.insert("bonds".to_string(), "Bonds".to_string());
        field_map.insert("flaws".to_string(), "Flaws".to_string());

        // Currency
        field_map.insert("copper_pieces".to_string(), "CP".to_string());
        field_map.insert("silver_pieces".to_string(), "SP".to_string());
        field_map.insert("electrum_pieces".to_string(), "EP".to_string());
        field_map.insert("gold_pieces".to_string(), "GP".to_string());
        field_map.insert("platinum_pieces".to_string(), "PP".to_string());

        Self { field_map }
    }

    pub fn get_pdf_field_name(&self, json_field: &str) -> Option<&String> {
        self.field_map.get(json_field)
    }

    pub fn get_spell_field_name(&self, level: u8, index: usize) -> String {
        match level {
            0 => format!("Spells {}", 10100 + index), // Cantrips: 10100-101013
            1 => format!("Spells {}", 1014 + index),  // 1st level: 1014-1025
            2 => format!("Spells {}", 1026 + index),  // 2nd level: 1026-1038
            3 => format!("Spells {}", 1039 + index),  // 3rd level: 1039-1051
            4 => format!("Spells {}", 1052 + index),  // 4th level: 1052-1064
            5 => format!("Spells {}", 1065 + index),  // 5th level: 1065-1077
            6 => format!("Spells {}", 1078 + index),  // 6th level: 1078-1090
            7 => format!("Spells {}", 1091 + index),  // 7th level: 1091-1103
            8 => format!("Spells {}", 1104 + index),  // 8th level: 1104-1116
            9 => format!("Spells {}", 1117 + index),  // 9th level: 1117-1129
            _ => format!("Unknown Spell Level {}", level),
        }
    }

    pub fn get_spell_prepared_checkbox(&self, level: u8, index: usize) -> String {
        match level {
            0 => format!("Check Box {}", 314 + index), // 314-321 (8 cantrips)
            1 => format!("Check Box {}", 322 + index), // 322-333 (12 slots)
            2 => format!("Check Box {}", 334 + index), // 334-346 (13 slots)
            3 => format!("Check Box {}", 347 + index), // 347-359 (13 slots)
            4 => format!("Check Box {}", 360 + index), // 360-372 (13 slots)
            5 => format!("Check Box {}", 373 + index), // 373-385 (13 slots)
            6 => format!("Check Box {}", 386 + index), // 386-398 (13 slots)
            7 => format!("Check Box {}", 399 + index), // 399-411 (13 slots)
            8 => format!("Check Box {}", 412 + index), // 412-424 (13 slots)
            9 => format!("Check Box {}", 425 + index), // 425-437 (13 slots)
            _ => format!("Unknown Checkbox Level {}", level),
        }
    }
}
