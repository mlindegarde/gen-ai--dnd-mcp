use lopdf::Document;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <pdf_file>", args[0]);
        return Ok(());
    }

    let doc = Document::load(&args[1])?;
    
    println!("=== CHECKBOX STATES ON PAGE 1 ===");
    
    // Get page 1 (use first page)
    let pages = doc.get_pages();
    if let Some((_page_id, page_obj)) = pages.iter().next() {
        if let Ok(page) = doc.get_object(*page_obj) {
            if let lopdf::Object::Dictionary(page_dict) = page {
                // Look for annotations (form fields)
                if let Ok(lopdf::Object::Reference(annots_ref)) = page_dict.get(b"Annots") {
                    if let Ok(lopdf::Object::Array(annots)) = doc.get_object(*annots_ref) {
                        for annot_ref in annots {
                            if let lopdf::Object::Reference(ref_id) = annot_ref {
                                if let Ok(lopdf::Object::Dictionary(field_dict)) = doc.get_object(*ref_id) {
                                    // Check if it's a form field widget
                                    if let Ok(lopdf::Object::Name(subtype)) = field_dict.get(b"Subtype") {
                                        if subtype == b"Widget" {
                                            // Get field name
                                            if let Ok(lopdf::Object::String(field_name_bytes, _)) = field_dict.get(b"T") {
                                                let field_name = String::from_utf8_lossy(field_name_bytes);
                                                
                                                // Get field value
                                                let value = if let Ok(lopdf::Object::String(value_bytes, _)) = field_dict.get(b"V") {
                                                    String::from_utf8_lossy(value_bytes).to_string()
                                                } else if let Ok(lopdf::Object::Name(value_name)) = field_dict.get(b"V") {
                                                    String::from_utf8_lossy(value_name).to_string()
                                                } else {
                                                    "".to_string()
                                                };
                                                
                                                // Get appearance state (for checkboxes)
                                                let appearance = if let Ok(lopdf::Object::Name(as_name)) = field_dict.get(b"AS") {
                                                    String::from_utf8_lossy(as_name).to_string()
                                                } else {
                                                    "".to_string()
                                                };
                                                
                                                // Show all checkbox-like fields and their states
                                                if field_name.contains("Check") || field_name.contains("Box") || 
                                                   !value.is_empty() || (appearance != "Off" && !appearance.is_empty()) {
                                                    println!("Field: '{}' | Value: '{}' | Appearance: '{}'", 
                                                           field_name, value, appearance);
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
    
    Ok(())
}
