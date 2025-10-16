# D&D 5e Checkbox Mapping Progress

## Process
1. User checks ONE skill proficiency checkbox in `docs/debug-sheet.pdf` 
2. Run: `cargo run --bin read_checkbox_states -- "docs/debug-sheet.pdf" | grep -v "Off"`
3. Identify which "Check Box X" shows `Value: 'Yes'` and `Appearance: 'Yes'`
4. Record the mapping below
5. Move to next skill alphabetically

## Confirmed Mappings

### Saving Throws (COMPLETE)
- Strength = Check Box 11 ✅
- Dexterity = Check Box 18 ✅  
- Constitution = Check Box 19 ✅
- Intelligence = Check Box 20 ✅
- Wisdom = Check Box 21 ✅
- Charisma = Check Box 22 ✅

### Skills (COMPLETE! 🎉)
- **Acrobatics = Check Box 23** ✅
- **Animal Handling = Check Box 24** ✅
- **Arcana = Check Box 25** ✅
- **Athletics = Check Box 26** ✅
- **Deception = Check Box 27** ✅
- **History = Check Box 28** ✅
- **Insight = Check Box 29** ✅
- **Intimidation = Check Box 30** ✅
- **Investigation = Check Box 31** ✅
- **Medicine = Check Box 32** ✅
- **Nature = Check Box 33** ✅
- **Perception = Check Box 34** ✅
- **Performance = Check Box 35** ✅
- **Persuasion = Check Box 36** ✅
- **Religion = Check Box 37** ✅
- **Sleight of Hand = Check Box 38** ✅
- **Stealth = Check Box 39** ✅
- **Survival = Check Box 40** ✅
- Athletics = ?
- Deception = ?
- History = ?
- Insight = ?
- Intimidation = ?
- Investigation = ?
- Medicine = ?
- Nature = ?
- Perception = ?
- Performance = ?
- Persuasion = ?
- Religion = ?
- Sleight of Hand = ?
- Stealth = ?
- Survival = ?

## Current Status
- **ALL MAPPINGS COMPLETE!** 🎉🎉
- **Skills:** 18/18 mapped (100% complete)
- **Saving Throws:** 6/6 mapped (100% complete) 
- **Spells:** All levels 1-9 mapped (100% complete)
- **Cantrips:** No checkboxes needed ✅

## Spell Checkbox Mappings (IN PROGRESS)

### Cantrips (0th Level)
- **No checkboxes needed** (cantrips don't require preparation) ✅

### 1st Level Spells
- **Check Box 251** ✅
- **Check Box 309** ✅
- **Check Box 3010** ✅
- **Check Box 3011** ✅
- **Check Box 3012** ✅
- **Check Box 3013** ✅
- **Check Box 3014** ✅
- **Check Box 3015** ✅
- **Check Box 3016** ✅
- **Check Box 3017** ✅
- **Check Box 3018** ✅
- **Check Box 3019** ✅

### 2nd Level Spells
- **Check Box 313** ✅
- **Check Box 310** ✅
- **Check Box 3020** ✅
- **Check Box 3021** ✅
- **Check Box 3022** ✅
- **Check Box 3023** ✅
- **Check Box 3024** ✅
- **Check Box 3025** ✅
- **Check Box 3026** ✅
- **Check Box 3027** ✅
- **Check Box 3028** ✅
- **Check Box 3029** ✅
- **Check Box 3030** ✅

### 3rd Level Spells
- **Check Box 315** ✅
- **Check Box 314** ✅
- **Check Box 3031** ✅
- **Check Box 3032** ✅
- **Check Box 3033** ✅
- **Check Box 3034** ✅
- **Check Box 3035** ✅
- **Check Box 3036** ✅
- **Check Box 3037** ✅
- **Check Box 3038** ✅
- **Check Box 3039** ✅
- **Check Box 3040** ✅
- **Check Box 3041** ✅

### 4th Level Spells
- **Check Box 317** ✅
- **Check Box 316** ✅
- **Check Box 3042** ✅
- **Check Box 3043** ✅
- **Check Box 3044** ✅
- **Check Box 3045** ✅
- **Check Box 3046** ✅
- **Check Box 3047** ✅
- **Check Box 3048** ✅
- **Check Box 3049** ✅
- **Check Box 3050** ✅
- **Check Box 3051** ✅
- **Check Box 3052** ✅

### 5th Level Spells
- **Check Box 319** ✅
- **Check Box 318** ✅
- **Check Box 3053** ✅
- **Check Box 3054** ✅
- **Check Box 3055** ✅
- **Check Box 3056** ✅
- **Check Box 3057** ✅
- **Check Box 3058** ✅
- **Check Box 3059** ✅

### 6th Level Spells
- **Check Box 321** ✅
- **Check Box 320** ✅
- **Check Box 3060** ✅
- **Check Box 3061** ✅
- **Check Box 3062** ✅
- **Check Box 3063** ✅
- **Check Box 3064** ✅
- **Check Box 3065** ✅
- **Check Box 3066** ✅

### 7th Level Spells
- **Check Box 323** ✅
- **Check Box 322** ✅
- **Check Box 3067** ✅
- **Check Box 3068** ✅
- **Check Box 3069** ✅
- **Check Box 3070** ✅
- **Check Box 3071** ✅
- **Check Box 3072** ✅
- **Check Box 3073** ✅

### 8th Level Spells
- **Check Box 325** ✅
- **Check Box 324** ✅
- **Check Box 3074** ✅
- **Check Box 3075** ✅
- **Check Box 3076** ✅
- **Check Box 3077** ✅
- **Check Box 3078** ✅

### 9th Level Spells
- **Check Box 327** ✅
- **Check Box 326** ✅
- **Check Box 3079** ✅
- **Check Box 3080** ✅
- **Check Box 3081** ✅
- **Check Box 3082** ✅
- **Check Box 3083** ✅

## Command to run after each checkbox check:
```bash
cd /Users/lindegar/learningplace/specify--test && cargo run --bin read_checkbox_states -- "docs/debug-sheet.pdf" | grep -v "Off"
```
