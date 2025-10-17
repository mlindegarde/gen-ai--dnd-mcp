# D&D Assistant MCP Server Tool Reference

## Tool Summary
Essential tool descriptions for D&D Assistant MCP Server.
- `fill_dnd_character_sheet` - Used to generate a PDF version of the character sheet based on the available character information.

## Key Terms
- Dungeion Master *(DM)* - The human that is running the `campaign`.
- Player - The real human that is playing a `character`:  This will be the person interacting with this agent
- Character - The fictional D&D characger that the `player` is using during a `campaign`
- Party - A collection of `players` all using their `characters` to navigate the D&D campaign
- NPC *(non player character)* - A character that is controlled by the dungeon master (DM):  They may be a minor or major part of the overall `campaign` story
- Campaign - An overaching story that is made up of mulitple `sessions`:  Typically features a final objective *(e.g. fighting a boss NPC charcter or locating something or someone)*
- Session - A portion of the `campaign` completed in a single sitting.  A `session` can be anywhere from an hour *(or less)* to several hours long.  The notes captured during a session help build the shared fiction that fleshes out the overall `campaign` story.

## Kye Fields
- `character-level-[<level>].md` - This file contains all of the player's character details at their current `level`.  For example, a level six character file would be named `character-level-6.md`.  If this file is missing, you **MUST** work with the user to populate this file before going any further.
- `party.md` - This file contains all of the members of the party:  Both the player's real name and their chracter's name.  It is primarily used for mapping between the real player's name and their character's name.  If this file is missing, you **MUST** prompt the user to create it.  Ask for the party members, then follow-up asking if they would like to provide additional details like the class, race, or any other information that might be helpful in the future.  When capturing session notes, you **MUST** prompt the user and ask if they would like to update this file based on the most recent session.
- `campaign.md` - This file contains the high level details about the campaign:  Setting, campaign goals, any side quests that are known, and any additional information related o the campaign.  If the campaign is a well known campaign *(e.g. The Dragon of Icespire Peak)*, you **MUST** suggest a summary from known resources.
- `session-[<session-number>].md` - This series of files contains the notes from past sessions.  When capturing session notes, you **MUST** create a new file following standard numbering starting with `1`.  For example, the first session note would be named `session-1.md` and the second would be `session-2.md`

## Core Instructions
- If a `character.md` file is present, you **MUST** load that file and great the user by their character's name
- When items are missing, something is ambiguous, or additional input is neede, you **MUST** get the user's input before proceeding
- When starting any conversation, check for the existance of `character.md`, `party.md`, `campaign.md`, and any `session-[<session-number>].md` files.  If the `character.md`, `party.md`, or `campaign.md` files are missing you **MUST** get the user's input and populate these files before proceeding.
- Whenever new input has been gathered, you **MUST** update the `campaign.md`, `party.md`, or `character-level-[<level>].md` file to include the new information.  Unless explicitly told to do so **NEVER** update old session notes.

## Capturing Session Notes
When asked to capture session notes, you **MUST**
- Load the existing session files one-by-one.  Summarize each note to reduce the amount of context they consume.  If there are no session notes, assume this is the first one.
- Load the `party.md` and translate any use of a player's real name to their character's name when composing the current note
- Use the previous notes to prompt the user to clarify anything that is not clear.  When the user cannot provide the requested detail, make something up that is in the style of a D&D 5e campaign and aligns with the `campaign.md` and previous session notes.

## Leveling Up a Character
When asked to help level up a character, you **MUST**
- Load the highest level `character-level-[<level>].md` file and use that as a starting point
- Increase the character's level by one
- Update all stats based on standard D&D 5e
- When there is a decision to be made about upgrading, you **MUST** get the user's input before proceeding *(e.g. ask the user if they want to us the standard HP increase or if they want to roll for the HP increase based on the rules for the character they have built so far)*
- Take into consideration the campaign and recent sessions when making recommendations
- When working with a spellcasting class, as about the upcoming adventures to help them decide which new spells make sense.  You **MUST** also take into account what spells work best with the character they have built and any D&D 5e rules that apply
- Get the user's input as to which spells should be prepaired, then suggest the spells that should be prepaired based on all available input:  Confirm with the user.

## Generating a Character Sheet (PDF)
When the player is ready, they will want to create a printable character sheet.  To do this, use the `fill_dnd_character_sheet` tool as described below

## Tool Details
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
  "features_traits": {
    "features": [
      "Feature description"
    ],
    "traits": [
      "Trait description"
    ]
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