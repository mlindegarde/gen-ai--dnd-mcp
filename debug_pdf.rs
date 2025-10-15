use lopdf::{Document, Object};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doc = Document::load("test_character_sheet.pdf")?;
    
    println!("=== PDF FORM FIELDS ===");
    
    // Check AcroForm fields
    if let Ok(catalog) = doc.trailer.get(b"Root") {
        if let Ok(catalog_ref) = catalog.as_reference() {
            if let Ok(Object::Dictionary(catalog_dict)) = doc.get_object(catalog_ref) {
                if let Ok(acroform) = catalog_dict.get(b"AcroForm") {
                    if let Ok(acroform_ref) = acroform.as_reference() {
                        if let Ok(Object::Dictionary(acroform_dict)) = doc.get_object(acroform_ref) {
                            if let Ok(fields) = acroform_dict.get(b"Fields") {
                                if let Ok(fields_ref) = fields.as_reference() {
                                    if let Ok(Object::Array(fields_array)) = doc.get_object(fields_ref) {
                                        println!("Found {} form fields", fields_array.len());
                                        
                                        for (i, field_obj) in fields_array.iter().enumerate().take(10) {
                                            if let Object::Reference(field_ref) = field_obj {
                                                if let Ok(Object::Dictionary(field_dict)) = doc.get_object(*field_ref) {
                                                    if let Ok(Object::String(field_name_bytes, _)) = field_dict.get(b"T") {
                                                        let field_name = String::from_utf8_lossy(&field_name_bytes);
                                                        
                                                        let field_value = if let Ok(Object::String(value_bytes, _)) = field_dict.get(b"V") {
                                                            String::from_utf8_lossy(&value_bytes).to_string()
                                                        } else {
                                                            "No value".to_string()
                                                        };
                                                        
                                                        println!("Field {}: '{}' = '{}'", i + 1, field_name, field_value);
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
