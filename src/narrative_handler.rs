use crate::character_model::CharacterNarrative;

pub struct NarrativeHandler;

impl NarrativeHandler {
    pub fn format_narrative_fields(narrative: &CharacterNarrative) -> Vec<(String, String)> {
        let mut fields = Vec::new();

        if let Some(traits) = &narrative.personality_traits {
            fields.push(("personality_traits".to_string(), traits.clone()));
        }

        if let Some(ideals) = &narrative.ideals {
            fields.push(("ideals".to_string(), ideals.clone()));
        }

        if let Some(bonds) = &narrative.bonds {
            fields.push(("bonds".to_string(), bonds.clone()));
        }

        if let Some(flaws) = &narrative.flaws {
            fields.push(("flaws".to_string(), flaws.clone()));
        }

        fields
    }

    pub fn truncate_if_needed(text: &str, max_length: usize) -> String {
        if text.len() <= max_length {
            text.to_string()
        } else {
            format!("{}...", &text[..max_length.saturating_sub(3)])
        }
    }
}
