use lopdf::Document;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <pdf_file>", args[0]);
        return Ok(());
    }

    let doc = Document::load(&args[1])?;
    
    println!("=== ALL CHECKBOX FIELDS ===");
    
    // Get the catalog
    if let Ok(catalog) = doc.catalog() {
        if let Ok(acroform) = catalog.get(b"AcroForm") {
            if let Ok(acroform_dict) = acroform.as_dict() {
                if let Ok(fields) = acroform_dict.get(b"Fields") {
                    if let Ok(fields_array) = fields.as_array() {
                        for field_ref in fields_array {
                            if let Ok(field_obj) = doc.get_object(field_ref.as_reference()?) {
                                if let Ok(field_dict) = field_obj.as_dict() {
                                    // Get field name
                                    if let Ok(name_obj) = field_dict.get(b"T") {
                                        if let Ok(name) = name_obj.as_str() {
                                            let name_str = String::from_utf8_lossy(name);
                                            
                                            // Look for checkbox fields
                                            if name_str.contains("Check Box") {
                                                // Get field value
                                                let value = if let Ok(value_obj) = field_dict.get(b"V") {
                                                    if let Ok(value_str) = value_obj.as_str() {
                                                        String::from_utf8_lossy(value_str).to_string()
                                                    } else {
                                                        "".to_string()
                                                    }
                                                } else {
                                                    "".to_string()
                                                };
                                                
                                                // Get appearance state
                                                let appearance = if let Ok(as_obj) = field_dict.get(b"AS") {
                                                    if let Ok(as_name) = as_obj.as_name() {
                                                        String::from_utf8_lossy(as_name).to_string()
                                                    } else {
                                                        "".to_string()
                                                    }
                                                } else {
                                                    "".to_string()
                                                };
                                                
                                                println!("{}: value='{}', appearance='{}'", name_str, value, appearance);
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
