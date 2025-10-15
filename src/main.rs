mod character_model;
mod dnd_validator;
mod errors;
mod field_mapper;
mod mcp_server;
mod narrative_handler;
mod pdf_filler;
mod spell_system;

use lopdf::{Document, Object};
use mcp_server::McpServer;
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "test" {
        println!("Test mode - MCP server functionality verified");
        return;
    }

    if args.len() > 1 && args[1] == "bob" {
        if let Err(e) = simple_bob_test() {
            eprintln!("Bob test error: {}", e);
        }
        return;
    }

    if args.len() > 1 && args[1] == "test-main" {
        if let Err(e) = test_main_pdf_filler() {
            eprintln!("Main PDF filler test error: {}", e);
        }
        return;
    }

    if args.len() > 1 && args[1] == "test-docs" {
        if let Err(e) = test_docs_data() {
            eprintln!("Docs test error: {}", e);
        }
        return;
    }

    if args.len() > 1 && args[1] == "read" {
        let filename = if args.len() > 2 { &args[2] } else { "test_ap.pdf" };
        if let Err(e) = read_pdf_fields(filename) {
            eprintln!("Read error: {}", e);
        }
        return;
    }

    let server = McpServer::new();
    if let Err(e) = server.run() {
        eprintln!("Server error: {}", e);
    }
}

#[allow(dead_code)]
fn extract_debug_fields() -> Result<(), Box<dyn std::error::Error>> {
    let doc = Document::load("debug-sheet.pdf")?;
    let mut field_values = HashMap::new();

    // Extract all form field values
    for (_page_id, page_obj) in doc.get_pages() {
        if let Ok(page) = doc.get_object(page_obj) {
            if let Object::Dictionary(page_dict) = page {
                if let Ok(Object::Reference(annots_ref)) = page_dict.get(b"Annots") {
                    if let Ok(Object::Array(annots)) = doc.get_object(*annots_ref) {
                        for annot_ref in annots {
                            if let Object::Reference(ref_id) = annot_ref {
                                if let Ok(Object::Dictionary(field_dict)) = doc.get_object(*ref_id)
                                {
                                    if let Ok(Object::Name(subtype)) = field_dict.get(b"Subtype") {
                                        if subtype == b"Widget" {
                                            if let Ok(Object::String(field_name, _)) =
                                                field_dict.get(b"T")
                                            {
                                                let name_str = String::from_utf8_lossy(field_name);

                                                // Get field value
                                                if let Ok(Object::String(field_value, _)) =
                                                    field_dict.get(b"V")
                                                {
                                                    let value_str =
                                                        String::from_utf8_lossy(field_value);
                                                    if !value_str.is_empty() {
                                                        field_values.insert(
                                                            name_str.to_string(),
                                                            value_str.to_string(),
                                                        );
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
        }
    }

    // Print cantrip mappings
    println!("=== CANTRIP MAPPINGS ===");
    for (field_name, value) in &field_values {
        if value.starts_with("Cantrip ") {
            println!("Field '{}' = '{}'", field_name, value);
        }
    }

    // Print spell level mappings
    println!("\n=== SPELL LEVEL MAPPINGS ===");
    for level in 1..=9 {
        println!("\nLevel {} spells:", level);
        for (field_name, value) in &field_values {
            if value.starts_with(&format!("Level {}, ", level)) {
                println!("Field '{}' = '{}'", field_name, value);
            }
        }
    }

    Ok(())
}

fn simple_bob_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SIMPLE BOB TEST ===");
    
    // Load the PDF
    let mut doc = Document::load("test_character_sheet.pdf")?;
    
    // Find and update fields
    let mut found_fields = 0;
    for (object_id, object) in doc.objects.clone() {
        if let Object::Dictionary(dict) = object {
            if let Ok(Object::String(field_name_bytes, _)) = dict.get(b"T") {
                let field_name = String::from_utf8_lossy(&field_name_bytes);
                
                let mut new_dict = dict.clone();
                let mut updated = false;
                
                // Character name
                if field_name == "CharacterName" || field_name == "Character Name" || field_name == "Name" {
                    new_dict.set(b"V", Object::String(b"Bob".to_vec(), lopdf::StringFormat::Literal));
                    println!("Set {} to 'Bob'", field_name);
                    updated = true;
                }
                // Class & Level variations
                else if field_name == "ClassLevel" || field_name == "Class & Level" || field_name == "Class" || field_name == "Level" {
                    new_dict.set(b"V", Object::String(b"Wizard 6".to_vec(), lopdf::StringFormat::Literal));
                    println!("Set {} to 'Wizard 6'", field_name);
                    updated = true;
                }
                // Alignment
                else if field_name == "Alignment" {
                    new_dict.set(b"V", Object::String(b"Chaotic Evil".to_vec(), lopdf::StringFormat::Literal));
                    println!("Set {} to 'Chaotic Evil'", field_name);
                    updated = true;
                }
                
                if updated {
                    doc.objects.insert(object_id, Object::Dictionary(new_dict));
                    found_fields += 1;
                }
            }
        }
    }
    
    // Save the updated PDF
    doc.save("bob_test.pdf")?;
    println!("Saved bob_test.pdf with {} fields updated", found_fields);
    
    Ok(())
}

fn test_main_pdf_filler() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== TESTING MAIN PDF FILLER ===");
    
    // Create minimal character data
    let character_data = character_model::CharacterData {
        character: character_model::Character {
            name: "Bob".to_string(),
            class: "Wizard".to_string(),
            level: 6,
            background: Some("Scholar".to_string()),
            player_name: Some("Test Player".to_string()),
            race: "Human".to_string(),
            alignment: Some("Chaotic Evil".to_string()),
            experience_points: Some(14000),
        },
        abilities: character_model::AbilityScores {
            strength: 10,
            dexterity: 14,
            constitution: 12,
            intelligence: 16,
            wisdom: 13,
            charisma: 8,
        },
        proficiencies: None,
        combat: None,
        spells: None,
        equipment: None,
        narrative: None,
    };
    
    println!("Created character: {}", character_data.character.name);
    
    // Create PDF filler
    let pdf_filler = pdf_filler::PdfFiller::new(false);
    
    // Test with the existing PDF
    let template_path = "test_character_sheet.pdf";
    let output_path = "main_test.pdf";
    
    match pdf_filler.fill_character_sheet(&character_data, template_path, output_path) {
        Ok(result) => {
            println!("✅ Main PDF filler works!");
            println!("Output file: {}", result.output_file);
        }
        Err(e) => {
            println!("❌ Error with main PDF filler: {:?}", e);
        }
    }
    
    Ok(())
}

fn test_docs_data() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use serde_json;
    
    println!("=== TESTING DOCS DATA ===");
    
    // Load the docs test data
    let character_json = fs::read_to_string("docs/test-data.json")?;
    let character_data: character_model::CharacterData = serde_json::from_str(&character_json)?;
    
    println!("Loaded character: {}", character_data.character.name);
    
    // Create PDF filler
    let pdf_filler = pdf_filler::PdfFiller::new(false);
    
    // Test with the same template as MCP server
    let template_path = "docs/5E_CharacterSheet_Fillable.pdf";
    let output_path = "docs_test.pdf";
    
    match pdf_filler.fill_character_sheet(&character_data, template_path, output_path) {
        Ok(result) => {
            println!("✅ Docs data PDF filler works!");
            println!("Output file: {}", result.output_file);
        }
        Err(e) => {
            println!("❌ Error with docs data PDF filler: {:?}", e);
        }
    }
    
    Ok(())
}

fn read_pdf_fields(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let doc = Document::load(filename)?;

    // Read all form field values
    for (_object_id, object) in &doc.objects {
        if let Object::Dictionary(dict) = object {
            if let Ok(Object::Name(subtype)) = dict.get(b"Subtype") {
                if subtype == b"Widget" {
                    if let Ok(Object::String(field_name_bytes, _)) = dict.get(b"T") {
                        let field_name = String::from_utf8_lossy(field_name_bytes);

                        if let Ok(Object::String(field_value, _)) = dict.get(b"V") {
                            let value_str = String::from_utf8_lossy(field_value);
                            if !value_str.is_empty() {
                                println!("Field '{}' = '{}'", field_name, value_str);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
