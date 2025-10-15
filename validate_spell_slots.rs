use dnd_character_sheet_filler::character_model::{CharacterData, Character, AbilityScores};
use dnd_character_sheet_filler::spell_system::SpellSystem;

fn main() {
    println!("=== Spell Slot Validation ===");

    // Test Wizard Level 5
    let wizard = CharacterData {
        character: Character {
            name: "Test Wizard".to_string(),
            class: "Wizard".to_string(),
            level: 5,
            race: "Human".to_string(),
            background: None,
            player_name: None,
            alignment: None,
            experience_points: None,
        },
        abilities: AbilityScores {
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

    let wizard_slots = SpellSystem::calculate_spell_slots(&wizard);
    println!("Wizard Level 5 spell slots: {:?}", wizard_slots);
    
    // Expected: spell_slots_1: 4, spell_slots_2: 3, spell_slots_3: 2
    assert_eq!(wizard_slots.get("spell_slots_1"), Some(&4));
    assert_eq!(wizard_slots.get("spell_slots_2"), Some(&3));
    assert_eq!(wizard_slots.get("spell_slots_3"), Some(&2));
    println!("✅ Wizard Level 5 validation passed");

    // Test Paladin Level 5
    let paladin = CharacterData {
        character: Character {
            name: "Test Paladin".to_string(),
            class: "Paladin".to_string(),
            level: 5,
            race: "Human".to_string(),
            background: None,
            player_name: None,
            alignment: None,
            experience_points: None,
        },
        abilities: AbilityScores {
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

    let paladin_slots = SpellSystem::calculate_spell_slots(&paladin);
    println!("Paladin Level 5 spell slots: {:?}", paladin_slots);
    
    // Expected: spell_slots_1: 4, spell_slots_2: 2
    assert_eq!(paladin_slots.get("spell_slots_1"), Some(&4));
    assert_eq!(paladin_slots.get("spell_slots_2"), Some(&2));
    println!("✅ Paladin Level 5 validation passed");

    // Test Barbarian (non-caster)
    let barbarian = CharacterData {
        character: Character {
            name: "Test Barbarian".to_string(),
            class: "Barbarian".to_string(),
            level: 5,
            race: "Human".to_string(),
            background: None,
            player_name: None,
            alignment: None,
            experience_points: None,
        },
        abilities: AbilityScores {
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

    let barbarian_slots = SpellSystem::calculate_spell_slots(&barbarian);
    println!("Barbarian Level 5 spell slots: {:?}", barbarian_slots);
    
    // Expected: empty (no spell slots)
    assert!(barbarian_slots.is_empty());
    println!("✅ Barbarian Level 5 validation passed");

    println!("🎉 All spell slot validations passed!");
}
