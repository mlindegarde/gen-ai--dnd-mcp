use std::fmt;

#[derive(Debug, Clone)]
pub enum ValidationError {
    AbilityScore {
        ability: String,
        value: u8,
        message: String,
    },
    Level {
        level: u8,
        message: String,
    },
    ExperiencePoints {
        xp: u32,
        level: u8,
        required: u32,
    },
    Spell {
        spell: String,
        level: u8,
        message: String,
    },
    #[allow(dead_code)]
    Proficiency {
        skill: String,
        message: String,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::AbilityScore {
                ability,
                value,
                message,
            } => {
                write!(f, "{} score {} {}", ability, value, message)
            }
            ValidationError::Level { level, message } => {
                write!(f, "Character level {} {}", level, message)
            }
            ValidationError::ExperiencePoints {
                xp,
                level,
                required,
            } => {
                write!(
                    f,
                    "Experience points {} insufficient for level {} (requires {})",
                    xp, level, required
                )
            }
            ValidationError::Spell {
                spell,
                level,
                message,
            } => {
                write!(f, "Spell '{}' level {} {}", spell, level, message)
            }
            ValidationError::Proficiency { skill, message } => {
                write!(f, "Skill '{}' {}", skill, message)
            }
        }
    }
}

#[derive(Debug)]
pub enum PdfError {
    #[allow(dead_code)]
    FileNotFound(String),
    #[allow(dead_code)]
    ParseError(String),
    #[allow(dead_code)]
    FieldNotFound(String),
    WriteError(String),
}

impl fmt::Display for PdfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PdfError::FileNotFound(path) => write!(f, "PDF file not found: {}", path),
            PdfError::ParseError(msg) => write!(f, "PDF parse error: {}", msg),
            PdfError::FieldNotFound(field) => write!(f, "PDF field not found: {}", field),
            PdfError::WriteError(msg) => write!(f, "PDF write error: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}
impl std::error::Error for PdfError {}
