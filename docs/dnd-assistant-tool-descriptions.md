# D&D Assistant MCP Server Tool Reference

Essential tool descriptions for D&D Assistant MCP Server.

## Core Character Tools

### PDF Character Sheet Generation
`fill_dnd_character_sheet(character_data, output_path, return_pdf_content, allow_rule_violations)` - Used to populate the standard D&D 5e character sheet based on the provided `character_data`.

When using this tool, if not otherwise specified, use the following default values:
- `output_path` - The file should use the format `[character-name]_[class]_[level]` *(all lowercase)*
- `return_pdf_content` - This should always be true
- `allow_rule_violations` - This should always be falst

#### Determining Spells to Prepare
If the character is a spellcasting class, when determining which spells to prepare, use the available context to pick the best spells up to the maximum number of spells that can be prepared for the given character.  Always show the user the list of spells you have selected to prepare.

When using the `fill_dnd_character_sheet` tool, format character data exactly as follows:

```json
{
  "character": {
    "name": "Character Name",
    "class": "Class Name",
    "level": 1,
    "background": "Background Name",
    "player_name": "Player Name (optional)",
    "race": "Race Name",
    "alignment": "Alignment"
  },
  "abilities": {
    "strength": 10,
    "dexterity": 10,
    "constitution": 10,
    "intelligence": 10,
    "wisdom": 10,
    "charisma": 10
  },
  "proficiencies": {
    "saving_throws": ["ability1", "ability2"],
    "skills": ["skill1", "skill2", "skill3"]
  },
  "combat": {
    "armor_class": 10,
    "initiative": 0,
    "speed": 30,
    "hit_point_maximum": 8,
    "current_hit_points": 8,
    "temporary_hit_points": 0,
    "hit_dice": "1d8",
    "hit_dice_total": 1
  },
  "spells": {
    "spellcasting_class": "Class Name",
    "spellcasting_ability": "Intelligence",
    "cantrips": [{"name": "Spell Name", "level": 0, "prepared": true}],
    "first_level": [{"name": "Spell Name", "level": 1, "prepared": true}],
    "second_level": [],
    "third_level": [],
    "fourth_level": [],
    "fifth_level": [],
    "sixth_level": [],
    "seventh_level": [],
    "eighth_level": [],
    "ninth_level": []
  },
  "narrative": {
    "personality_traits": "Trait description",
    "ideals": "Ideal description",
    "bonds": "Bond description",
    "flaws": "Flaw description"
  },
  "equipment": {
    "currency": {"cp": 0, "sp": 0, "ep": 0, "gp": 0, "pp": 0},
    "items": "Comma-separated list of equipment"
  }
}
```

Key requirements:
- XP should be ignored, we do not use that in our campaign
- All spell levels (cantrips through ninth_level) must be present, even if empty arrays
- The number of prepared spells should be correctly limited based on the character stats
- Skills and saving throws use lowercase ability names
- Currency uses abbreviated keys: cp, sp, ep, gp, pp
- Spells need name, level, and prepared fields
- Equipment items go in a single string, not an array