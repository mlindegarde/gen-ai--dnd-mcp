use lopdf::Document;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <pdf_file>", args[0]);
        return Ok(());
    }

    let doc = Document::load(&args[1])?;
    
    println!("=== PDF FORM FIELDS ===");
    
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
                                            let value = if let Ok(value_obj) = field_dict.get(b"V") {
                                                if let Ok(value_str) = value_obj.as_str() {
                                                    String::from_utf8_lossy(value_str).to_string()
                                                } else {
                                                    "".to_string()
                                                }
                                            } else {
                                                "".to_string()
                                            };
                                            
                                            // Filter for relevant fields
                                            let name_lower = name_str.to_lowercase();
                                            if name_lower.contains("save") || 
                                               name_lower.contains("skill") ||
                                               name_lower.contains("str") ||
                                               name_lower.contains("dex") ||
                                               name_lower.contains("con") ||
                                               name_lower.contains("int") ||
                                               name_lower.contains("wis") ||
                                               name_lower.contains("cha") {
                                                println!("{}: '{}'", name_str, value);
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
