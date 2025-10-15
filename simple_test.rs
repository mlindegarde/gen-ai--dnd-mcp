use lopdf::{Document, Object};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the PDF
    let mut doc = Document::load("test_character_sheet.pdf")?;
    
    // Find and update the character name field
    for (object_id, object) in doc.objects.clone() {
        if let Object::Dictionary(dict) = object {
            if let Ok(Object::String(field_name_bytes, _)) = dict.get(b"T") {
                let field_name = String::from_utf8_lossy(&field_name_bytes);
                
                // Look for character name field (try common variations)
                if field_name == "CharacterName" || field_name == "Character Name" || field_name == "Name" {
                    println!("Found field: {}", field_name);
                    
                    // Update the field value
                    let mut new_dict = dict.clone();
                    new_dict.set(b"V", Object::String(b"Bob".to_vec(), lopdf::StringFormat::Literal));
                    doc.objects.insert(object_id, Object::Dictionary(new_dict));
                    
                    println!("Set {} to 'Bob'", field_name);
                }
            }
        }
    }
    
    // Save the updated PDF
    doc.save("bob_test.pdf")?;
    println!("Saved bob_test.pdf");
    
    Ok(())
}
