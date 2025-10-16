use lopdf::{Document, Object, Dictionary};
use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test Wisdom saving throw proficiency = Check Box 21
    let mut doc = Document::load("erilon_wizard_6.pdf")?;
    
    // Set Check Box 21 to "Yes" (checked)
    if let Some((_, form)) = doc.catalog()?.get(b"AcroForm")?.as_reference()? {
        if let Ok(Object::Dictionary(form_dict)) = doc.get_object_mut(form) {
            if let Ok(Object::Reference(fields_ref)) = form_dict.get(b"Fields") {
                if let Ok(Object::Array(fields)) = doc.get_object_mut(*fields_ref) {
                    for field_ref in fields {
                        if let Object::Reference(ref_id) = field_ref {
                            if let Ok(Object::Dictionary(field_dict)) = doc.get_object_mut(*ref_id) {
                                if let Ok(Object::String(name_bytes, _)) = field_dict.get(b"T") {
                                    let name = String::from_utf8_lossy(name_bytes);
                                    if name == "Check Box 21" {
                                        field_dict.set("V", Object::Name(b"Yes".to_vec()));
                                        field_dict.set("AS", Object::Name(b"Yes".to_vec()));
                                        println!("Set Check Box 21 to checked");
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    doc.save("test_wisdom_checked.pdf")?;
    println!("Saved test_wisdom_checked.pdf - check if Wisdom saving throw proficiency is marked");
    Ok(())
}
