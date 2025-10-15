use std::fs;
use serde_json;

mod character_model;
mod pdf_filler;
mod field_mapper;
mod dnd_validator;
mod errors;
mod narrative_handler;
mod spell_system;

use character_model::CharacterData;
use pdf_filler::PdfFiller;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load sample character data
    let character_json = fs::read_to_string("tests/fixtures/sample-character.json")?;
    let character_data: CharacterData = serde_json::from_str(&character_json)?;
    
    println!("Loaded character: {}", character_data.character.name);
    
    // Create PDF filler
    let pdf_filler = PdfFiller::new(false);
    
    // Test with a blank PDF template (you'll need to provide this)
    let template_path = "blank_character_sheet.pdf";
    let output_path = "filled_character_sheet.pdf";
    
    if !std::path::Path::new(template_path).exists() {
        println!("❌ Template PDF not found: {}", template_path);
        println!("Please provide a blank D&D 5e character sheet PDF as '{}'", template_path);
        return Ok(());
    }
    
    match pdf_filler.fill_character_sheet(&character_data, template_path, output_path) {
        Ok(result) => {
            println!("✅ PDF filled successfully!");
            println!("Output file: {}", result.output_file);
            println!("Validation errors: {:?}", result.validation_errors);
            println!("Calculated fields: {:?}", result.calculated_fields);
        }
        Err(e) => {
            println!("❌ Error filling PDF: {:?}", e);
        }
    }
    
    Ok(())
}
