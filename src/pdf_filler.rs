use crate::character_model::CharacterData;
use crate::dnd_validator::DndValidator;
use crate::errors::{PdfError, ValidationError};
use crate::field_mapper::FieldMapper;
use crate::narrative_handler::NarrativeHandler;
use crate::spell_system::SpellSystem;
use lopdf::{Document, Object};
use std::collections::HashMap;

pub struct PdfFiller {
    field_mapper: FieldMapper,
    validator: DndValidator,
}

impl PdfFiller {
    pub fn new(allow_rule_violations: bool) -> Self {
        Self {
            field_mapper: FieldMapper::new(),
            validator: DndValidator::new(allow_rule_violations),
        }
    }

    pub fn fill_character_sheet_from_bytes(
        &self,
        character_data: &CharacterData,
        template_bytes: &[u8],
        output_path: &str,
    ) -> Result<FillResult, PdfError> {
        // Validate character data
        let validation_result = self.validator.validate(character_data);
        let validation_errors = match validation_result {
            Ok(warnings) => warnings,
            Err(errors) => {
                return Err(PdfError::WriteError(format!(
                    "Validation failed: {:?}",
                    errors
                )))
            }
        };

        // Load the PDF template from bytes
        let mut doc = Document::load_mem(template_bytes)
            .map_err(|e| PdfError::WriteError(format!("Failed to load PDF from bytes: {}", e)))?;

        // Fill the PDF form fields
        self.fill_pdf_fields(&mut doc, character_data)?;

        // Save the filled PDF with explicit sync
        doc.save(output_path)
            .map_err(|e| PdfError::WriteError(format!("Failed to save PDF: {}", e)))?;

        // Force filesystem sync to ensure file is written
        use std::fs::File;
        use std::io::Write;
        if let Ok(mut file) = File::options().write(true).open(output_path) {
            let _ = file.flush();
            let _ = file.sync_all();
        }

        Ok(FillResult {
            success: true,
            output_file: output_path.to_string(),
            validation_errors,
            calculated_fields: self.calculate_derived_values(character_data),
        })
    }

    pub fn fill_character_sheet(
        &self,
        character_data: &CharacterData,
        template_path: &str,
        output_path: &str,
    ) -> Result<FillResult, PdfError> {
        // Validate character data
        let validation_result = self.validator.validate(character_data);
        let validation_errors = match validation_result {
            Ok(warnings) => warnings,
            Err(errors) => {
                return Err(PdfError::WriteError(format!(
                    "Validation failed: {:?}",
                    errors
                )))
            }
        };

        // Load the PDF template
        let mut doc = Document::load(template_path)
            .map_err(|e| PdfError::WriteError(format!("Failed to load PDF: {}", e)))?;

        // Fill the PDF form fields
        self.fill_pdf_fields(&mut doc, character_data)?;

        // Save the filled PDF with explicit sync
        doc.save(output_path)
            .map_err(|e| PdfError::WriteError(format!("Failed to save PDF: {}", e)))?;

        // Force filesystem sync to ensure file is written
        use std::fs::File;
        use std::io::Write;
        if let Ok(mut file) = File::options().write(true).open(output_path) {
            let _ = file.flush();
            let _ = file.sync_all();
        }

        Ok(FillResult {
            success: true,
            output_file: output_path.to_string(),
            validation_errors,
            calculated_fields: self.calculate_derived_values(character_data),
        })
    }

    fn fill_pdf_fields(
        &self,
        doc: &mut Document,
        character_data: &CharacterData,
    ) -> Result<(), PdfError> {
        let field_values = self.get_field_values(character_data);
        let checkbox_fields = self.get_checkbox_fields(character_data);
        
        // Set NeedAppearances to true so PDF readers regenerate field appearances
        // This is critical for checkbox visibility on Windows
        if let Ok(catalog) = doc.catalog() {
            if let Ok(acroform_ref) = catalog.get(b"AcroForm") {
                if let Object::Reference(ref_id) = acroform_ref {
                    if let Ok(Object::Dictionary(mut acroform_dict)) = doc.get_object(*ref_id).cloned() {
                        acroform_dict.set(b"NeedAppearances", Object::Boolean(true));
                        doc.objects.insert(*ref_id, Object::Dictionary(acroform_dict));
                    }
                }
            }
        }
        
        // Fill text fields
        for (object_id, object) in doc.objects.clone() {
            if let Object::Dictionary(dict) = object {
                if let Ok(Object::String(field_name_bytes, _)) = dict.get(b"T") {
                    let field_name = String::from_utf8_lossy(&field_name_bytes);
                    
                    if let Some(value) = field_values.get(field_name.as_ref()) {
                        let mut new_dict = dict.clone();
                        
                        // Set the field value
                        new_dict.set(b"V", Object::String(value.as_bytes().to_vec(), lopdf::StringFormat::Literal));
                        
                        // Set default appearance with appropriate font size
                        let da_string = self.create_default_appearance(&field_name);
                        new_dict.set(b"DA", Object::String(da_string.as_bytes().to_vec(), lopdf::StringFormat::Literal));
                        
                        doc.objects.insert(object_id, Object::Dictionary(new_dict));
                    }
                }
            }
        }
        
        // Mark checkboxes
        self.mark_checkboxes(doc, &checkbox_fields)?;
        
        Ok(())
    }
    
    #[allow(dead_code)]
    fn update_form_fields(
        &self,
        doc: &mut Document,
        fields: &[Object],
        field_values: &HashMap<String, String>,
    ) -> Result<(), PdfError> {
        for field_obj in fields {
            if let Object::Reference(field_ref) = field_obj {
                if let Ok(Object::Dictionary(field_dict)) = doc.get_object(*field_ref) {
                    if let Ok(Object::String(field_name_bytes, _)) = field_dict.get(b"T") {
                        let field_name = String::from_utf8_lossy(&field_name_bytes);
                        
                        if let Some(value) = field_values.get(field_name.as_ref()) {
                            let mut new_dict = field_dict.clone();
                            new_dict.set(b"V", Object::String(value.as_bytes().to_vec(), lopdf::StringFormat::Literal));
                            new_dict.remove(b"AP"); // Remove appearance to force regeneration
                            doc.objects.insert(*field_ref, Object::Dictionary(new_dict));
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    /// Mark checkbox fields in the PDF by setting their value to "Yes"
    fn mark_checkboxes(
        &self,
        doc: &mut Document,
        checkbox_fields: &HashMap<String, bool>,
    ) -> Result<(), PdfError> {
        for (object_id, object) in doc.objects.clone() {
            if let Object::Dictionary(dict) = object {
                if let Ok(Object::String(field_name_bytes, _)) = dict.get(b"T") {
                    let field_name = String::from_utf8_lossy(&field_name_bytes);
                    
                    if let Some(&should_mark) = checkbox_fields.get(field_name.as_ref()) {
                        if should_mark {
                            let mut new_dict = dict.clone();
                            
                            // Use Name objects instead of String for checkbox values
                            // This is required by PDF spec for button fields
                            new_dict.set(b"V", Object::Name(b"Yes".to_vec()));
                            
                            // Set appearance state to match the value
                            // This tells PDF viewers which appearance to display
                            new_dict.set(b"AS", Object::Name(b"Yes".to_vec()));
                            
                            doc.objects.insert(object_id, Object::Dictionary(new_dict));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Determine appropriate font size based on field name
    fn get_font_size_for_field(&self, field_name: &str) -> u8 {
        // Character name and player name - larger for readability
        if field_name.contains("CharacterName") || field_name.contains("PlayerName") {
            return 12;
        }
        
        // Spell fields - smaller to fit more text
        if field_name.starts_with("Spells") {
            return 8;
        }
        
        // Narrative fields - slightly smaller
        if field_name.contains("Traits") || 
           field_name.contains("Features") || 
           field_name.contains("Ideals") ||
           field_name.contains("Bonds") ||
           field_name.contains("Flaws") {
            return 9;
        }
        
        // Default for all other fields
        10
    }

    /// Create PDF default appearance string with font and size
    fn create_default_appearance(&self, field_name: &str) -> String {
        let font_size = self.get_font_size_for_field(field_name);
        // Format: /FontName FontSize Tf Color
        // /Helv = Helvetica, Tf = Text Font operator, 0 g = black
        format!("/Helv {} Tf 0 g", font_size)
    }

    #[allow(dead_code)]
    fn debug_spell_fields(&self, doc: &Document) -> Result<(), PdfError> {
        println!("=== SPELL FIELD NAMES IN PDF ===");

        for (_page_id, page_obj) in doc.get_pages() {
            if let Ok(page) = doc.get_object(page_obj) {
                if let Object::Dictionary(page_dict) = page {
                    if let Ok(Object::Reference(annots_ref)) = page_dict.get(b"Annots") {
                        if let Ok(Object::Array(annots)) = doc.get_object(*annots_ref) {
                            for annot_ref in annots {
                                if let Object::Reference(ref_id) = annot_ref {
                                    if let Ok(Object::Dictionary(field_dict)) =
                                        doc.get_object(*ref_id)
                                    {
                                        if let Ok(Object::Name(subtype)) =
                                            field_dict.get(b"Subtype")
                                        {
                                            if subtype == b"Widget" {
                                                if let Ok(Object::String(field_name, _)) =
                                                    field_dict.get(b"T")
                                                {
                                                    let name_str =
                                                        String::from_utf8_lossy(field_name);
                                                    println!("Field: '{}'", name_str);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("=== END SPELL FIELDS ===");
        Ok(())
    }

    pub fn get_field_values(&self, character_data: &CharacterData) -> HashMap<String, String> {
        let mut fields = HashMap::new();

        // Basic character info using field mapper
        let character = &character_data.character;
        if let Some(field_name) = self.field_mapper.get_pdf_field_name("character_name") {
            fields.insert(field_name.clone(), character.name.clone());
        }
        if let Some(field_name) = self.field_mapper.get_pdf_field_name("class_level") {
            fields.insert(
                field_name.clone(),
                format!("{} {}", character.class, character.level),
            );
        }
        if let Some(field_name) = self.field_mapper.get_pdf_field_name("race") {
            fields.insert(field_name.clone(), character.race.clone());
        }
        if let Some(background) = &character.background {
            if let Some(field_name) = self.field_mapper.get_pdf_field_name("background") {
                fields.insert(field_name.clone(), background.clone());
            }
        }
        if let Some(player_name) = &character.player_name {
            if let Some(field_name) = self.field_mapper.get_pdf_field_name("player_name") {
                fields.insert(field_name.clone(), player_name.clone());
            }
        }
        if let Some(alignment) = &character.alignment {
            if let Some(field_name) = self.field_mapper.get_pdf_field_name("alignment") {
                fields.insert(field_name.clone(), alignment.clone());
            }
        }
        if let Some(xp) = character.experience_points {
            if let Some(field_name) = self.field_mapper.get_pdf_field_name("experience_points") {
                fields.insert(field_name.clone(), xp.to_string());
            }
        }

        // Ability scores and modifiers using field mapper
        let abilities = &character_data.abilities;

        if let Some(field_name) = self.field_mapper.get_pdf_field_name("strength") {
            fields.insert(field_name.clone(), abilities.strength.to_string());
        }
        if let Some(field_name) = self.field_mapper.get_pdf_field_name("strength_modifier") {
            fields.insert(
                field_name.clone(),
                self.calculate_modifier(abilities.strength).to_string(),
            );
        }

        if let Some(field_name) = self.field_mapper.get_pdf_field_name("dexterity") {
            fields.insert(field_name.clone(), abilities.dexterity.to_string());
        }
        if let Some(field_name) = self.field_mapper.get_pdf_field_name("dexterity_modifier") {
            fields.insert(
                field_name.clone(),
                self.calculate_modifier(abilities.dexterity).to_string(),
            );
        }

        if let Some(field_name) = self.field_mapper.get_pdf_field_name("constitution") {
            fields.insert(field_name.clone(), abilities.constitution.to_string());
        }
        if let Some(field_name) = self
            .field_mapper
            .get_pdf_field_name("constitution_modifier")
        {
            fields.insert(
                field_name.clone(),
                self.calculate_modifier(abilities.constitution).to_string(),
            );
        }

        if let Some(field_name) = self.field_mapper.get_pdf_field_name("intelligence") {
            fields.insert(field_name.clone(), abilities.intelligence.to_string());
        }
        if let Some(field_name) = self
            .field_mapper
            .get_pdf_field_name("intelligence_modifier")
        {
            fields.insert(
                field_name.clone(),
                self.calculate_modifier(abilities.intelligence).to_string(),
            );
        }

        if let Some(field_name) = self.field_mapper.get_pdf_field_name("wisdom") {
            fields.insert(field_name.clone(), abilities.wisdom.to_string());
        }
        if let Some(field_name) = self.field_mapper.get_pdf_field_name("wisdom_modifier") {
            fields.insert(
                field_name.clone(),
                self.calculate_modifier(abilities.wisdom).to_string(),
            );
        }

        if let Some(field_name) = self.field_mapper.get_pdf_field_name("charisma") {
            fields.insert(field_name.clone(), abilities.charisma.to_string());
        }
        if let Some(field_name) = self.field_mapper.get_pdf_field_name("charisma_modifier") {
            fields.insert(
                field_name.clone(),
                self.calculate_modifier(abilities.charisma).to_string(),
            );
        }

        // Combat stats using field mapper
        if let Some(combat) = &character_data.combat {
            if let Some(ac) = combat.armor_class {
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("armor_class") {
                    fields.insert(field_name.clone(), ac.to_string());
                }
            }
            if let Some(hp) = combat.hit_point_maximum {
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("hit_point_maximum")
                {
                    fields.insert(field_name.clone(), hp.to_string());
                }
            }
            if let Some(current_hp) = combat.current_hit_points {
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("current_hit_points")
                {
                    fields.insert(field_name.clone(), current_hp.to_string());
                }
            }
            if let Some(speed) = combat.speed {
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("speed") {
                    fields.insert(field_name.clone(), speed.to_string());
                }
            }
            if let Some(initiative) = combat.initiative {
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("initiative") {
                    fields.insert(field_name.clone(), initiative.to_string());
                }
            }
        }

        // Proficiency bonus using field mapper
        let prof_bonus = self.calculate_proficiency_bonus(character.level);
        if let Some(field_name) = self.field_mapper.get_pdf_field_name("proficiency_bonus") {
            fields.insert(field_name.clone(), format!("+{}", prof_bonus));
        }

        // Passive Perception calculation
        if let Some(proficiencies) = &character_data.proficiencies {
            // Validate Wisdom score is within D&D 5e range (1-30)
            if abilities.wisdom >= 1 && abilities.wisdom <= 30 {
                let is_perception_proficient = proficiencies.skills.contains(&"perception".to_string());
                let passive_perception = 10 + self.calculate_modifier(abilities.wisdom) + 
                    if is_perception_proficient { prof_bonus } else { 0 };
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("passive_perception") {
                    fields.insert(field_name.clone(), passive_perception.to_string());
                }
            }
            // If Wisdom is invalid, passive perception field remains empty (graceful handling)
        }

        // Hit Dice calculation
        let hit_die_type = match character.class.as_str() {
            "Barbarian" => "d12",
            "Fighter" | "Paladin" | "Ranger" => "d10",
            "Bard" | "Cleric" | "Druid" | "Monk" | "Rogue" | "Warlock" => "d8",
            "Sorcerer" | "Wizard" => "d6",
            _ => "d8", // Default fallback
        };
        if let Some(field_name) = self.field_mapper.get_pdf_field_name("hit_dice_total") {
            fields.insert(field_name.clone(), character.level.to_string());
        }
        if let Some(field_name) = self.field_mapper.get_pdf_field_name("hit_dice_type") {
            fields.insert(field_name.clone(), hit_die_type.to_string());
        }

        // Currency fields
        if let Some(equipment) = &character_data.equipment {
            if let Some(currency) = &equipment.currency {
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("copper_pieces") {
                    fields.insert(field_name.clone(), currency.cp.to_string());
                }
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("silver_pieces") {
                    fields.insert(field_name.clone(), currency.sp.to_string());
                }
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("electrum_pieces") {
                    fields.insert(field_name.clone(), currency.ep.to_string());
                }
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("gold_pieces") {
                    fields.insert(field_name.clone(), currency.gp.to_string());
                }
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("platinum_pieces") {
                    fields.insert(field_name.clone(), currency.pp.to_string());
                }
            }
        }

        // Features & Traits field
        if let Some(features_traits) = &character_data.features_traits {
            let mut lines = Vec::new();
            
            if let Some(features) = &features_traits.features {
                if !features.is_empty() {
                    lines.push("Features:".to_string());
                    for feature in features {
                        lines.push(format!("- {}", feature));
                    }
                }
            }
            
            if let Some(traits) = &features_traits.traits {
                if !traits.is_empty() {
                    if !lines.is_empty() {
                        lines.push("".to_string()); // Empty line separator
                    }
                    lines.push("Traits:".to_string());
                    for trait_item in traits {
                        lines.push(format!("- {}", trait_item));
                    }
                }
            }
            
            if !lines.is_empty() {
                let features_text = lines.join("\n");
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("features_traits") {
                    fields.insert(field_name.clone(), features_text);
                }
            }
        }

        // Spells using field mapper
        if let Some(spells) = &character_data.spells {
            // Spell attack bonus and save DC
            let spell_mod = match spells.spellcasting_ability.as_deref() {
                Some("Intelligence") => self.calculate_modifier(abilities.intelligence),
                Some("Wisdom") => self.calculate_modifier(abilities.wisdom),
                Some("Charisma") => self.calculate_modifier(abilities.charisma),
                _ => 0,
            };
            let spell_attack = spell_mod + prof_bonus;
            let spell_save_dc = 8 + spell_mod + prof_bonus;

            if let Some(field_name) = self.field_mapper.get_pdf_field_name("spell_attack_bonus") {
                fields.insert(field_name.clone(), format!("+{}", spell_attack));
            }
            if let Some(field_name) = self.field_mapper.get_pdf_field_name("spell_save_dc") {
                fields.insert(field_name.clone(), spell_save_dc.to_string());
            }

            // Spellcasting header fields
            if let Some(field_name) = self.field_mapper.get_pdf_field_name("spellcasting_class") {
                fields.insert(field_name.clone(), character.class.clone());
            }
            if let Some(ability_name) = &spells.spellcasting_ability {
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("spellcasting_ability") {
                    fields.insert(field_name.clone(), ability_name.clone());
                }
            }

            // Individual spells by level using proper field mapper
            self.add_spell_fields_with_mapper(&mut fields, 0, &spells.cantrips);
            self.add_spell_fields_with_mapper(&mut fields, 1, &spells.first_level);
            self.add_spell_fields_with_mapper(&mut fields, 2, &spells.second_level);
            self.add_spell_fields_with_mapper(&mut fields, 3, &spells.third_level);
            self.add_spell_fields_with_mapper(&mut fields, 4, &spells.fourth_level);
            self.add_spell_fields_with_mapper(&mut fields, 5, &spells.fifth_level);
            self.add_spell_fields_with_mapper(&mut fields, 6, &spells.sixth_level);
            self.add_spell_fields_with_mapper(&mut fields, 7, &spells.seventh_level);
            self.add_spell_fields_with_mapper(&mut fields, 8, &spells.eighth_level);
            self.add_spell_fields_with_mapper(&mut fields, 9, &spells.ninth_level);
        } else {
            // Handle characters without existing spell data - determine spellcasting from class
            let spellcasting_ability = match character.class.as_str() {
                "Wizard" | "Eldritch Knight" | "Arcane Trickster" => Some("Intelligence"),
                "Cleric" | "Druid" | "Ranger" => Some("Wisdom"),
                "Sorcerer" | "Bard" | "Paladin" => Some("Charisma"),
                _ => None, // Non-spellcaster
            };

            if let Some(ability) = spellcasting_ability {
                // Calculate spellcasting stats
                let spell_mod = match ability {
                    "Intelligence" => self.calculate_modifier(abilities.intelligence),
                    "Wisdom" => self.calculate_modifier(abilities.wisdom),
                    "Charisma" => self.calculate_modifier(abilities.charisma),
                    _ => 0,
                };
                let spell_attack = spell_mod + prof_bonus;
                let spell_save_dc = 8 + spell_mod + prof_bonus;

                // Populate spellcasting header fields
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("spellcasting_class") {
                    fields.insert(field_name.clone(), character.class.clone());
                }
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("spellcasting_ability") {
                    fields.insert(field_name.clone(), ability.to_string());
                }
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("spell_attack_bonus") {
                    fields.insert(field_name.clone(), format!("+{}", spell_attack));
                }
                if let Some(field_name) = self.field_mapper.get_pdf_field_name("spell_save_dc") {
                    fields.insert(field_name.clone(), spell_save_dc.to_string());
                }
            }
            // Non-spellcasters: leave spellcasting header fields empty (no action needed)
        }

        // Character narrative using narrative handler
        if let Some(narrative) = &character_data.narrative {
            let narrative_fields = NarrativeHandler::format_narrative_fields(narrative);
            for (field_key, field_value) in narrative_fields {
                if let Some(pdf_field_name) = self.field_mapper.get_pdf_field_name(&field_key) {
                    // Truncate long text to fit PDF fields (max 500 chars)
                    let truncated_value = NarrativeHandler::truncate_if_needed(&field_value, 500);
                    fields.insert(pdf_field_name.clone(), truncated_value);
                }
            }
        }

        // Spell slot calculation and display
        let spell_slots = crate::spell_system::SpellSystem::calculate_spell_slots(character_data);
        for (logical_name, count) in spell_slots {
            if let Some(field_name) = self.field_mapper.get_pdf_field_name(&logical_name) {
                if count > 0 {  // Only show non-zero slots
                    fields.insert(field_name.clone(), count.to_string());
                }
            }
        }

        // Saving throw bonuses
        if let Some(proficiencies) = &character_data.proficiencies {
            // Calculate all six saving throws
            let saves = [
                ("strength", abilities.strength),
                ("dexterity", abilities.dexterity), 
                ("constitution", abilities.constitution),
                ("intelligence", abilities.intelligence),
                ("wisdom", abilities.wisdom),
                ("charisma", abilities.charisma),
            ];
            
            for (ability_name, ability_score) in saves {
                let is_proficient = proficiencies.saving_throws.contains(&ability_name.to_string());
                let save_field_key = format!("{}_save", ability_name);
                if let Some(field_name) = self.field_mapper.get_pdf_field_name(&save_field_key) {
                    let bonus_str = self.calculate_saving_throw_safe(ability_score, character.level, is_proficient, ability_name);
                    fields.insert(field_name.clone(), bonus_str);
                }
            }

            // Skill bonuses
            let skills = [
                ("acrobatics", abilities.dexterity, "Dexterity"),
                ("animal_handling", abilities.wisdom, "Wisdom"),
                ("arcana", abilities.intelligence, "Intelligence"),
                ("athletics", abilities.strength, "Strength"),
                ("deception", abilities.charisma, "Charisma"),
                ("history", abilities.intelligence, "Intelligence"),
                ("insight", abilities.wisdom, "Wisdom"),
                ("intimidation", abilities.charisma, "Charisma"),
                ("investigation", abilities.intelligence, "Intelligence"),
                ("medicine", abilities.wisdom, "Wisdom"),
                ("nature", abilities.intelligence, "Intelligence"),
                ("perception", abilities.wisdom, "Wisdom"),
                ("performance", abilities.charisma, "Charisma"),
                ("persuasion", abilities.charisma, "Charisma"),
                ("religion", abilities.intelligence, "Intelligence"),
                ("sleight_of_hand", abilities.dexterity, "Dexterity"),
                ("stealth", abilities.dexterity, "Dexterity"),
                ("survival", abilities.wisdom, "Wisdom"),
            ];

            for (skill_name, ability_score, ability_name) in skills {
                let is_proficient = proficiencies.skills.contains(&skill_name.to_string());
                if let Some(field_name) = self.field_mapper.get_pdf_field_name(skill_name) {
                    let bonus_str = self.calculate_skill_safe(ability_score, character.level, is_proficient, ability_name);
                    fields.insert(field_name.clone(), bonus_str);
                }
            }
        }

        fields
    }

    pub fn get_checkbox_fields(&self, character_data: &CharacterData) -> HashMap<String, bool> {
        let mut checkboxes = HashMap::new();

        // Saving throw proficiency checkboxes
        if let Some(proficiencies) = &character_data.proficiencies {
            let saves = [
                ("strength", "strength_save_prof"),
                ("dexterity", "dexterity_save_prof"), 
                ("constitution", "constitution_save_prof"),
                ("intelligence", "intelligence_save_prof"),
                ("wisdom", "wisdom_save_prof"),
                ("charisma", "charisma_save_prof"),
            ];
            
            for (ability_name, checkbox_key) in saves {
                let is_proficient = proficiencies.saving_throws.contains(&ability_name.to_string());
                if let Some(checkbox_field) = self.field_mapper.get_pdf_field_name(checkbox_key) {
                    checkboxes.insert(checkbox_field.clone(), is_proficient);
                }
            }

            // Skill proficiency checkboxes
            let skills = [
                "acrobatics", "animal_handling", "arcana", "athletics", "deception",
                "history", "insight", "intimidation", "investigation", "medicine",
                "nature", "perception", "performance", "persuasion", "religion",
                "sleight_of_hand", "stealth", "survival"
            ];

            for skill_name in skills {
                let is_proficient = proficiencies.skills.contains(&skill_name.to_string());
                let checkbox_key = format!("{}_prof", skill_name);
                if let Some(checkbox_field) = self.field_mapper.get_pdf_field_name(&checkbox_key) {
                    checkboxes.insert(checkbox_field.clone(), is_proficient);
                }
            }
        }

        checkboxes
    }

    fn add_spell_fields_with_mapper(
        &self,
        fields: &mut HashMap<String, String>,
        level: u8,
        spells: &[crate::character_model::Spell],
    ) {
        // Define maximum spell slots per level based on typical character sheet layout
        let max_spells_per_level = match level {
            0 => 8,  // Cantrips
            1..=9 => 13, // Spell levels 1-9
            _ => 0,
        };

        for (index, spell) in spells.iter().enumerate() {
            // Validate spell level matches expected level
            if spell.level != level {
                // Skip invalid spells but continue processing others
                continue;
            }

            // Handle overflow: only process spells that fit in available slots
            if index >= max_spells_per_level {
                break; // Stop processing when we exceed available slots
            }

            let spell_field = self.field_mapper.get_spell_field_name(level, index);
            fields.insert(spell_field, spell.name.clone());

            // Only mark preparation checkboxes for levels 1-9 (cantrips don't need preparation)
            if spell.prepared && level > 0 {
                let prepared_field = self.field_mapper.get_spell_prepared_checkbox(level, index);
                fields.insert(prepared_field, "Yes".to_string());
            }
        }
    }

    fn calculate_proficiency_bonus(&self, level: u8) -> i8 {
        match level {
            1..=4 => 2,
            5..=8 => 3,
            9..=12 => 4,
            13..=16 => 5,
            17..=20 => 6,
            _ => 2,
        }
    }

    fn calculate_modifier(&self, ability_score: u8) -> i8 {
        ((ability_score as i16 - 10) / 2) as i8
    }

    /// Calculate ability modifier with validation, returns "ERROR" string for invalid scores
    fn calculate_modifier_safe(&self, ability_score: u8, ability_name: &str) -> String {
        if let Err(_) = crate::dnd_validator::DndValidator::validate_ability_score_range(ability_score, ability_name) {
            "ERROR".to_string()
        } else {
            let modifier = self.calculate_modifier(ability_score);
            if modifier >= 0 {
                format!("+{}", modifier)
            } else {
                modifier.to_string()
            }
        }
    }

    /// Calculate saving throw bonus with error handling
    fn calculate_saving_throw_safe(&self, ability_score: u8, level: u8, is_proficient: bool, ability_name: &str) -> String {
        if let Err(_) = crate::dnd_validator::DndValidator::validate_ability_score_range(ability_score, ability_name) {
            "ERROR".to_string()
        } else {
            let bonus = crate::character_model::calculate_saving_throw_bonus(ability_score, level, is_proficient);
            if bonus >= 0 {
                format!("+{}", bonus)
            } else {
                bonus.to_string()
            }
        }
    }

    /// Calculate skill bonus with error handling
    fn calculate_skill_safe(&self, ability_score: u8, level: u8, is_proficient: bool, ability_name: &str) -> String {
        if let Err(_) = crate::dnd_validator::DndValidator::validate_ability_score_range(ability_score, ability_name) {
            "ERROR".to_string()
        } else {
            let bonus = crate::character_model::calculate_skill_bonus(ability_score, level, is_proficient);
            if bonus >= 0 {
                format!("+{}", bonus)
            } else {
                bonus.to_string()
            }
        }
    }

    #[allow(dead_code)]
    fn generate_character_sheet_text(&self, character_data: &CharacterData) -> String {
        let mut content = String::new();

        content.push_str("=== D&D 5e CHARACTER SHEET ===\n\n");

        // Basic character info
        let character = &character_data.character;
        content.push_str(&format!("Character Name: {}\n", character.name));
        content.push_str(&format!(
            "Class & Level: {} {}\n",
            character.class, character.level
        ));
        content.push_str(&format!("Race: {}\n", character.race));

        if let Some(background) = &character.background {
            content.push_str(&format!("Background: {}\n", background));
        }
        if let Some(alignment) = &character.alignment {
            content.push_str(&format!("Alignment: {}\n", alignment));
        }
        if let Some(xp) = character.experience_points {
            content.push_str(&format!("Experience Points: {}\n", xp));
        }

        // Ability scores
        content.push_str("\n=== ABILITY SCORES ===\n");
        let abilities = &character_data.abilities;
        content.push_str(&format!(
            "Strength: {} ({})\n",
            abilities.strength,
            abilities.modifier("strength")
        ));
        content.push_str(&format!(
            "Dexterity: {} ({})\n",
            abilities.dexterity,
            abilities.modifier("dexterity")
        ));
        content.push_str(&format!(
            "Constitution: {} ({})\n",
            abilities.constitution,
            abilities.modifier("constitution")
        ));
        content.push_str(&format!(
            "Intelligence: {} ({})\n",
            abilities.intelligence,
            abilities.modifier("intelligence")
        ));
        content.push_str(&format!(
            "Wisdom: {} ({})\n",
            abilities.wisdom,
            abilities.modifier("wisdom")
        ));
        content.push_str(&format!(
            "Charisma: {} ({})\n",
            abilities.charisma,
            abilities.modifier("charisma")
        ));

        // Combat stats
        if let Some(combat) = &character_data.combat {
            content.push_str("\n=== COMBAT STATS ===\n");
            if let Some(ac) = combat.armor_class {
                content.push_str(&format!("Armor Class: {}\n", ac));
            }
            if let Some(hp_max) = combat.hit_point_maximum {
                content.push_str(&format!("Hit Points: {}\n", hp_max));
            }
            if let Some(speed) = combat.speed {
                content.push_str(&format!("Speed: {} ft\n", speed));
            }
        }

        // Spells
        if let Some(spells) = &character_data.spells {
            content.push_str("\n=== SPELLS ===\n");
            if let Some(class) = &spells.spellcasting_class {
                content.push_str(&format!("Spellcasting Class: {}\n", class));
            }
            if let Some(ability) = &spells.spellcasting_ability {
                content.push_str(&format!("Spellcasting Ability: {}\n", ability));
            }

            let organized_spells = SpellSystem::organize_spells_by_level(spells);
            for (level, spell_list) in organized_spells {
                let level_name = if level == 0 {
                    "Cantrips".to_string()
                } else {
                    format!("Level {} Spells", level)
                };
                content.push_str(&format!("\n{}:\n", level_name));
                for spell in spell_list {
                    let prepared = if spell.prepared { " (Prepared)" } else { "" };
                    content.push_str(&format!("  - {}{}\n", spell.name, prepared));
                }
            }
        }

        // Narrative
        if let Some(narrative) = &character_data.narrative {
            content.push_str("\n=== CHARACTER DETAILS ===\n");
            if let Some(traits) = &narrative.personality_traits {
                content.push_str(&format!("Personality Traits: {}\n", traits));
            }
            if let Some(ideals) = &narrative.ideals {
                content.push_str(&format!("Ideals: {}\n", ideals));
            }
            if let Some(bonds) = &narrative.bonds {
                content.push_str(&format!("Bonds: {}\n", bonds));
            }
            if let Some(flaws) = &narrative.flaws {
                content.push_str(&format!("Flaws: {}\n", flaws));
            }
        }

        // Equipment
        if let Some(equipment) = &character_data.equipment {
            content.push_str("\n=== EQUIPMENT ===\n");
            if let Some(currency) = &equipment.currency {
                content.push_str(&format!(
                    "Currency: {} cp, {} sp, {} ep, {} gp, {} pp\n",
                    currency.cp, currency.sp, currency.ep, currency.gp, currency.pp
                ));
            }
            if let Some(items) = &equipment.items {
                content.push_str(&format!("Items: {}\n", items));
            }
        }

        // Calculated values
        content.push_str("\n=== CALCULATED VALUES ===\n");
        let calculated = self.calculate_derived_values(character_data);
        for (key, value) in calculated {
            content.push_str(&format!(
                "{}: {}\n",
                key.replace("_", " ").to_uppercase(),
                value
            ));
        }

        content
    }

    fn calculate_derived_values(&self, character_data: &CharacterData) -> HashMap<String, String> {
        let mut calculated = HashMap::new();
        let abilities = &character_data.abilities;

        calculated.insert(
            "strength_modifier".to_string(),
            abilities.modifier("strength").to_string(),
        );
        calculated.insert(
            "dexterity_modifier".to_string(),
            abilities.modifier("dexterity").to_string(),
        );
        calculated.insert(
            "constitution_modifier".to_string(),
            abilities.modifier("constitution").to_string(),
        );
        calculated.insert(
            "intelligence_modifier".to_string(),
            abilities.modifier("intelligence").to_string(),
        );
        calculated.insert(
            "wisdom_modifier".to_string(),
            abilities.modifier("wisdom").to_string(),
        );
        calculated.insert(
            "charisma_modifier".to_string(),
            abilities.modifier("charisma").to_string(),
        );

        let proficiency_bonus = DndValidator::proficiency_bonus(character_data.character.level);
        calculated.insert(
            "proficiency_bonus".to_string(),
            proficiency_bonus.to_string(),
        );

        if let Some(spells) = &character_data.spells {
            if let Some(ability) = &spells.spellcasting_ability {
                let ability_mod =
                    SpellSystem::get_spellcasting_ability_modifier(ability, abilities);
                let spell_attack =
                    SpellSystem::calculate_spell_attack_bonus(ability_mod, proficiency_bonus);
                let spell_dc = SpellSystem::calculate_spell_save_dc(ability_mod, proficiency_bonus);

                calculated.insert("spell_attack_bonus".to_string(), spell_attack.to_string());
                calculated.insert("spell_save_dc".to_string(), spell_dc.to_string());
            }
        }

        calculated
    }

}

#[derive(Debug)]
pub struct FillResult {
    #[allow(dead_code)]
    pub success: bool,
    pub output_file: String,
    pub validation_errors: Vec<ValidationError>,
    pub calculated_fields: HashMap<String, String>,
}
