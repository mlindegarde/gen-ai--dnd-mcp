use lopdf::Document;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <pdf_file>", args[0]);
        return Ok(());
    }

    let doc = Document::load(&args[1])?;
    
    println!("=== FILLED CHECKBOXES ===");
    
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
                                            
                                            // Get field value if present
                                            if let Ok(value_obj) = field_dict.get(b"V") {
                                                if let Ok(value_str) = value_obj.as_str() {
                                                    let value = String::from_utf8_lossy(value_str);
                                                    
                                                    // Look for checkbox fields with values
                                                    if name_str.contains("Check Box") && !value.is_empty() {
                                                        println!("FILLED: {} = '{}'", name_str, value);
                                                    }
                                                }
                                            }
                                            
                                            // Also check for /AS (appearance state) which might indicate checked state
                                            if let Ok(as_obj) = field_dict.get(b"AS") {
                                                if let Ok(as_name) = as_obj.as_name() {
                                                    let as_str = String::from_utf8_lossy(as_name);
                                                    if name_str.contains("Check Box") && as_str != "Off" {
                                                        println!("CHECKED: {} (appearance: {})", name_str, as_str);
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
    
    Ok(())
}
