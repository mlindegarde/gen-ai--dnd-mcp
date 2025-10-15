use lopdf::{Document, Object};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doc = Document::load("docs/debug-sheet.pdf")?;
    let mut field_values = HashMap::new();
    
    // Extract all form field values
    for (_page_id, page_obj) in doc.get_pages() {
        if let Ok(page) = doc.get_object(page_obj) {
            if let Object::Dictionary(page_dict) = page {
                if let Ok(Object::Reference(annots_ref)) = page_dict.get(b"Annots") {
                    if let Ok(Object::Array(annots)) = doc.get_object(*annots_ref) {
                        for annot_ref in annots {
                            if let Object::Reference(ref_id) = annot_ref {
                                if let Ok(Object::Dictionary(field_dict)) = doc.get_object(*ref_id) {
                                    if let Ok(Object::Name(subtype)) = field_dict.get(b"Subtype") {
                                        if subtype == b"Widget" {
                                            if let Ok(Object::String(field_name, _)) = field_dict.get(b"T") {
                                                let name_str = String::from_utf8_lossy(field_name);
                                                
                                                // Get field value
                                                if let Ok(Object::String(field_value, _)) = field_dict.get(b"V") {
                                                    let value_str = String::from_utf8_lossy(field_value);
                                                    if !value_str.is_empty() {
                                                        field_values.insert(name_str.to_string(), value_str.to_string());
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
    
    // Print spell slot mappings
    println!("\n=== SPELL SLOT MAPPINGS ===");
    for level in 1..=9 {
        for (field_name, value) in &field_values {
            if field_name == &format!("L{}", level) {
                println!("Field '{}' = '{}'", field_name, value);
            }
        }
    }
    
    Ok(())
}
