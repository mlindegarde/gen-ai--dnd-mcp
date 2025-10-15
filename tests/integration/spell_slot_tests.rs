use dnd_character_sheet_filler::character_model::CharacterData;
use dnd_character_sheet_filler::spell_system::{SpellSystem, CasterType};
use std::collections::HashMap;

#[test]
fn test_wizard_level_5_spell_slots() {
    let character_data = CharacterData {
        character: dnd_character_sheet_filler::character_model::Character {
            name: "Test Wizard".to_string(),
            class: "Wizard".to_string(),
            level: 5,
            race: "Human".to_string(),
            background: None,
            player_name: None,
            alignment: None,
            experience_points: None,
        },
        abilities: dnd_character_sheet_filler::character_model::AbilityScores {
            strength: 10,
            dexterity: 14,
            constitution: 13,
            intelligence: 16,
            wisdom: 12,
            charisma: 8,
        },
        combat: None,
        spells: None,
        narrative: None,
    };

    let spell_slots = SpellSystem::calculate_spell_slots(&character_data);
    
    // Expected: 4/3/2/0/0/0/0/0/0 (from full caster table)
    assert_eq!(spell_slots.get("spell_slots_1"), Some(&4));
    assert_eq!(spell_slots.get("spell_slots_2"), Some(&3));
    assert_eq!(spell_slots.get("spell_slots_3"), Some(&2));
    assert_eq!(spell_slots.get("spell_slots_4"), None); // 0 slots not included
    assert_eq!(spell_slots.get("spell_slots_5"), None);
}

#[test] 
fn test_paladin_level_5_spell_slots() {
    let character_data = CharacterData {
        character: dnd_character_sheet_filler::character_model::Character {
            name: "Test Paladin".to_string(),
            class: "Paladin".to_string(),
            level: 5,
            race: "Human".to_string(),
            background: None,
            player_name: None,
            alignment: None,
            experience_points: None,
        },
        abilities: dnd_character_sheet_filler::character_model::AbilityScores {
            strength: 16,
            dexterity: 10,
            constitution: 14,
            intelligence: 8,
            wisdom: 12,
            charisma: 15,
        },
        combat: None,
        spells: None,
        narrative: None,
    };

    let spell_slots = SpellSystem::calculate_spell_slots(&character_data);
    
    // Expected: 4/2/0/0/0/0/0/0/0 (from half caster table)
    assert_eq!(spell_slots.get("spell_slots_1"), Some(&4));
    assert_eq!(spell_slots.get("spell_slots_2"), Some(&2));
    assert_eq!(spell_slots.get("spell_slots_3"), None); // 0 slots not included
}

#[test]
fn test_barbarian_no_spell_slots() {
    let character_data = CharacterData {
        character: dnd_character_sheet_filler::character_model::Character {
            name: "Test Barbarian".to_string(),
            class: "Barbarian".to_string(),
            level: 5,
            race: "Human".to_string(),
            background: None,
            player_name: None,
            alignment: None,
            experience_points: None,
        },
        abilities: dnd_character_sheet_filler::character_model::AbilityScores {
            strength: 16,
            dexterity: 14,
            constitution: 15,
            intelligence: 8,
            wisdom: 12,
            charisma: 10,
        },
        combat: None,
        spells: None,
        narrative: None,
    };

    let spell_slots = SpellSystem::calculate_spell_slots(&character_data);
    
    // Expected: No spell slot fields (non-caster)
    assert!(spell_slots.is_empty());
}

#[test]
fn test_eldritch_knight_fighter_level_7() {
    let character_data = CharacterData {
        character: dnd_character_sheet_filler::character_model::Character {
            name: "Test Eldritch Knight".to_string(),
            class: "Fighter".to_string(),
            level: 7,
            race: "Human".to_string(),
            background: None,
            player_name: None,
            alignment: None,
            experience_points: None,
        },
        abilities: dnd_character_sheet_filler::character_model::AbilityScores {
            strength: 16,
            dexterity: 14,
            constitution: 15,
            intelligence: 14,
            wisdom: 12,
            charisma: 10,
        },
        combat: None,
        spells: None,
        narrative: None,
    };

    let spell_slots = SpellSystem::calculate_spell_slots(&character_data);
    
    // Expected: 4/2/0/0/0/0/0/0/0 (from third caster table)
    assert_eq!(spell_slots.get("spell_slots_1"), Some(&4));
    assert_eq!(spell_slots.get("spell_slots_2"), Some(&2));
    assert_eq!(spell_slots.get("spell_slots_3"), None); // 0 slots not included
}

#[test]
fn test_caster_type_classification() {
    assert_eq!(SpellSystem::get_caster_type("Wizard"), CasterType::Full);
    assert_eq!(SpellSystem::get_caster_type("Sorcerer"), CasterType::Full);
    assert_eq!(SpellSystem::get_caster_type("Cleric"), CasterType::Full);
    assert_eq!(SpellSystem::get_caster_type("Druid"), CasterType::Full);
    assert_eq!(SpellSystem::get_caster_type("Bard"), CasterType::Full);
    
    assert_eq!(SpellSystem::get_caster_type("Paladin"), CasterType::Half);
    assert_eq!(SpellSystem::get_caster_type("Ranger"), CasterType::Half);
    
    assert_eq!(SpellSystem::get_caster_type("Fighter"), CasterType::Third);
    assert_eq!(SpellSystem::get_caster_type("Rogue"), CasterType::Third);
    
    assert_eq!(SpellSystem::get_caster_type("Barbarian"), CasterType::None);
    assert_eq!(SpellSystem::get_caster_type("Monk"), CasterType::None);
}

#[test]
fn test_spell_slot_progression_tables() {
    // Test full caster progression
    let full_slots = SpellSystem::get_spell_slots_for_single_class(5, CasterType::Full);
    assert_eq!(full_slots.get(&1), Some(&4));
    assert_eq!(full_slots.get(&2), Some(&3));
    assert_eq!(full_slots.get(&3), Some(&2));
    assert_eq!(full_slots.get(&4), None);
    
    // Test half caster progression
    let half_slots = SpellSystem::get_spell_slots_for_single_class(5, CasterType::Half);
    assert_eq!(half_slots.get(&1), Some(&4));
    assert_eq!(half_slots.get(&2), Some(&2));
    assert_eq!(half_slots.get(&3), None);
    
    // Test third caster progression
    let third_slots = SpellSystem::get_spell_slots_for_single_class(7, CasterType::Third);
    assert_eq!(third_slots.get(&1), Some(&4));
    assert_eq!(third_slots.get(&2), Some(&2));
    assert_eq!(third_slots.get(&3), None);
    
    // Test non-caster
    let no_slots = SpellSystem::get_spell_slots_for_single_class(5, CasterType::None);
    assert!(no_slots.is_empty());
}
