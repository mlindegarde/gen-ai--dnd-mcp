# D&D 5e Character Sample Data

**Purpose**: Test data for validating PDF filling and rule validation
**Date**: 2025-10-14

## Complete Character Example

```json
{
  "character": {
    "name": "Thorin Ironforge",
    "class": "Fighter",
    "level": 5,
    "background": "Soldier",
    "player_name": "Alex",
    "race": "Mountain Dwarf",
    "alignment": "Lawful Good",
    "experience_points": 6500
  },
  "abilities": {
    "strength": 16,
    "dexterity": 12,
    "constitution": 15,
    "intelligence": 10,
    "wisdom": 13,
    "charisma": 8
  },
  "proficiencies": {
    "saving_throws": ["strength", "constitution"],
    "skills": ["athletics", "intimidation", "perception", "survival"]
  },
  "combat": {
    "armor_class": 18,
    "initiative": 1,
    "speed": 25,
    "hit_point_maximum": 47,
    "current_hit_points": 47,
    "temporary_hit_points": 0,
    "hit_dice": "5d10",
    "hit_dice_total": 5
  },
  "spells": {
    "spellcasting_class": "Eldritch Knight",
    "spellcasting_ability": "Intelligence",
    "cantrips": [
      {"name": "Mage Hand", "level": 0, "prepared": true},
      {"name": "Prestidigitation", "level": 0, "prepared": true}
    ],
    "first_level": [
      {"name": "Shield", "level": 1, "prepared": true},
      {"name": "Magic Missile", "level": 1, "prepared": true},
      {"name": "Cure Wounds", "level": 1, "prepared": false}
    ]
  },
  "narrative": {
    "personality_traits": "I face problems head-on. A simple, direct solution is the best path to success.",
    "ideals": "Responsibility. I do what I must and obey just authority.",
    "bonds": "I fight for those who cannot fight for themselves.",
    "flaws": "I have little respect for anyone who is not a proven warrior."
  },
  "equipment": {
    "currency": {"cp": 0, "sp": 0, "ep": 0, "gp": 150, "pp": 0},
    "items": "Chain mail, shield, warhammer, handaxe (2), light crossbow, crossbow bolts (20), explorer's pack"
  }
}
```

## Minimal Character (MVP Test)

```json
{
  "character": {
    "name": "Test Character",
    "class": "Fighter", 
    "level": 1,
    "race": "Human"
  },
  "abilities": {
    "strength": 15,
    "dexterity": 14,
    "constitution": 13,
    "intelligence": 12,
    "wisdom": 10,
    "charisma": 8
  }
}
```

## Edge Case: High Level Spellcaster

```json
{
  "character": {
    "name": "Gandalf the Grey",
    "class": "Wizard",
    "level": 17,
    "race": "Human",
    "experience_points": 225000
  },
  "abilities": {
    "strength": 8,
    "dexterity": 12,
    "constitution": 14,
    "intelligence": 20,
    "wisdom": 15,
    "charisma": 13
  },
  "spells": {
    "spellcasting_class": "Wizard",
    "spellcasting_ability": "Intelligence",
    "cantrips": [
      {"name": "Mage Hand", "level": 0, "prepared": true},
      {"name": "Prestidigitation", "level": 0, "prepared": true},
      {"name": "Light", "level": 0, "prepared": true},
      {"name": "Minor Illusion", "level": 0, "prepared": true}
    ],
    "ninth_level": [
      {"name": "Wish", "level": 9, "prepared": true},
      {"name": "Time Stop", "level": 9, "prepared": false}
    ]
  }
}
```

## Rule Violation Test (allow_rule_violations: true)

```json
{
  "character": {
    "name": "Overpowered Hero",
    "class": "Fighter",
    "level": 1,
    "race": "Human"
  },
  "abilities": {
    "strength": 25,
    "dexterity": 22,
    "constitution": 20,
    "intelligence": 18,
    "wisdom": 16,
    "charisma": 14
  }
}
```

## Empty Spells Test

```json
{
  "character": {
    "name": "Non-Caster",
    "class": "Barbarian",
    "level": 3,
    "race": "Half-Orc"
  },
  "abilities": {
    "strength": 17,
    "dexterity": 13,
    "constitution": 16,
    "intelligence": 8,
    "wisdom": 12,
    "charisma": 9
  },
  "spells": {
    "cantrips": [],
    "first_level": []
  }
}
```
