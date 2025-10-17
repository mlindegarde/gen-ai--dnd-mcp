use lopdf::{Document, Object};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let pdf_path = if args.len() > 1 { &args[1] } else { "docs/5E_CharacterSheet_Fillable.pdf" };
    
    let doc = Document::load(pdf_path)?;
    let mut field_names = Vec::new();
    
    // Extract all form field names
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
                                                field_names.push(name_str.to_string());
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
    
    // Sort and print all field names
    field_names.sort();
    println!("=== ALL PDF FIELD NAMES ===");
    for field_name in &field_names {
        println!("'{}'", field_name);
    }
    
    // Look for our specific fields
    println!("\n=== LOOKING FOR OUR FIELDS ===");
    let search_terms = ["personality", "passive", "hd", "cp", "sp", "ep", "gp", "pp", "features", "traits"];
    for term in &search_terms {
        println!("\nFields containing '{}':", term);
        for field_name in &field_names {
            if field_name.to_lowercase().contains(term) {
                println!("  '{}'", field_name);
            }
        }
    }
    
    Ok(())
}
