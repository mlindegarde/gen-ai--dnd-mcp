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

        // Proficiency bonus
        field_map.insert("proficiency_bonus".to_string(), "ProfBonus".to_string());

        // Spell attack and save DC
        field_map.insert("spell_attack_bonus".to_string(), "SpellAtkBonus".to_string());
        field_map.insert("spell_save_dc".to_string(), "SpellSaveDC".to_string());

        // Spell slot counts (corrected field names from PDF analysis)
        field_map.insert("spell_slots_1".to_string(), "SlotsTotal 19".to_string());
        field_map.insert("spell_slots_2".to_string(), "SlotsTotal 20".to_string());
        field_map.insert("spell_slots_3".to_string(), "SlotsTotal 21".to_string());
        field_map.insert("spell_slots_4".to_string(), "SlotsTotal 22".to_string());
        field_map.insert("spell_slots_5".to_string(), "SlotsTotal 23".to_string());
        field_map.insert("spell_slots_6".to_string(), "SlotsTotal 24".to_string());
        field_map.insert("spell_slots_7".to_string(), "SlotsTotal 25".to_string());
        field_map.insert("spell_slots_8".to_string(), "SlotsTotal 26".to_string());
        field_map.insert("spell_slots_9".to_string(), "SlotsTotal 27".to_string());

        Self { field_map }
    }

    pub fn get_pdf_field_name(&self, json_field: &str) -> Option<&String> {
        self.field_map.get(json_field)
    }

    pub fn get_spell_field_name(&self, level: u8, index: usize) -> String {
        match level {
            0 => {
                // Cantrips: 1014, 1016-1022
                let cantrip_fields = [1014, 1016, 1017, 1018, 1019, 1020, 1021, 1022];
                if index < cantrip_fields.len() {
                    format!("Spells {}", cantrip_fields[index])
                } else {
                    format!("Spells {}", 1014 + index)
                }
            },
            1 => {
                // Level 1: 1015, 1023-1033
                let level1_fields = [1015, 1023, 1024, 1025, 1026, 1027, 1028, 1029, 1030, 1031, 1032, 1033];
                if index < level1_fields.len() {
                    format!("Spells {}", level1_fields[index])
                } else {
                    format!("Spells {}", 1015 + index)
                }
            },
            2 => {
                // Level 2: 1034-1046
                let level2_fields = [1046, 1034, 1035, 1036, 1037, 1038, 1039, 1040, 1041, 1042, 1043, 1044, 1045];
                if index < level2_fields.len() {
                    format!("Spells {}", level2_fields[index])
                } else {
                    format!("Spells {}", 1034 + index)
                }
            },
            3 => {
                // Level 3: 1047-1059
                let level3_fields = [1048, 1047, 1049, 1050, 1051, 1052, 1053, 1054, 1055, 1056, 1057, 1059];
                if index < level3_fields.len() {
                    format!("Spells {}", level3_fields[index])
                } else {
                    format!("Spells {}", 1047 + index)
                }
            },
            4 => {
                // Level 4: 1060-1072
                let level4_fields = [1061, 1060, 1062, 1063, 1064, 1065, 1066, 1067, 1068, 1069, 1070, 1071, 1072];
                if index < level4_fields.len() {
                    format!("Spells {}", level4_fields[index])
                } else {
                    format!("Spells {}", 1060 + index)
                }
            },
            5 => {
                // Level 5: 1073-1081
                let level5_fields = [1074, 1073, 1075, 1076, 1077, 1078, 1079, 1080, 1081];
                if index < level5_fields.len() {
                    format!("Spells {}", level5_fields[index])
                } else {
                    format!("Spells {}", 1073 + index)
                }
            },
            6 => {
                // Level 6: 1082-1090
                let level6_fields = [1083, 1082, 1084, 1085, 1086, 1087, 1088, 1089, 1090];
                if index < level6_fields.len() {
                    format!("Spells {}", level6_fields[index])
                } else {
                    format!("Spells {}", 1082 + index)
                }
            },
            7 => {
                // Level 7: 1091-1099
                let level7_fields = [1092, 1091, 1093, 1094, 1095, 1096, 1097, 1098, 1099];
                if index < level7_fields.len() {
                    format!("Spells {}", level7_fields[index])
                } else {
                    format!("Spells {}", 1091 + index)
                }
            },
            8 => {
                // Level 8: 10100-10106
                let level8_fields = [10101, 10100, 10102, 10103, 10104, 10105, 10106];
                if index < level8_fields.len() {
                    format!("Spells {}", level8_fields[index])
                } else {
                    format!("Spells {}", 10100 + index)
                }
            },
            9 => {
                // Level 9: 10107-101013
                let level9_fields = [10108, 10107, 10109, 101010, 101011, 101012, 101013];
                if index < level9_fields.len() {
                    format!("Spells {}", level9_fields[index])
                } else {
                    format!("Spells {}", 10107 + index)
                }
            },
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
