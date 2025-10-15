use std::fmt;
use thiserror::Error;

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
    FileNotFound(String),
    ParseError(String),
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

// Enhanced error types for PDF saving operations
#[derive(Error, Debug, Clone)]
pub enum SaveError {
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Disk space full: {0}")]
    DiskSpaceFull(String),
    
    #[error("Directory creation failed: {0}")]
    DirectoryCreationFailed(String),
    
    #[error("PDF write failed: {0}")]
    PdfWriteFailed(String),
    
    #[error("Integrity check failed: {0}")]
    IntegrityCheckFailed(String),
}

impl SaveError {
    pub fn error_type(&self) -> &'static str {
        match self {
            SaveError::InvalidPath(_) => "InvalidPath",
            SaveError::PermissionDenied(_) => "PermissionDenied", 
            SaveError::DiskSpaceFull(_) => "DiskSpaceFull",
            SaveError::DirectoryCreationFailed(_) => "DirectoryCreationFailed",
            SaveError::PdfWriteFailed(_) => "PdfWriteFailed",
            SaveError::IntegrityCheckFailed(_) => "IntegrityCheckFailed",
        }
    }
    
    pub fn suggestions(&self) -> Vec<String> {
        match self {
            SaveError::InvalidPath(_) => vec![
                "Check that the path format is valid for your operating system".to_string(),
                "Ensure the path doesn't contain invalid characters".to_string(),
                "Try using an absolute path instead of a relative path".to_string(),
            ],
            SaveError::PermissionDenied(_) => vec![
                "Check that you have write permissions to the target directory".to_string(),
                "Try running with elevated permissions if necessary".to_string(),
                "Choose a different output location where you have write access".to_string(),
            ],
            SaveError::DiskSpaceFull(_) => vec![
                "Free up disk space on the target drive".to_string(),
                "Choose a different output location with more available space".to_string(),
                "Delete unnecessary files to make room for the PDF".to_string(),
            ],
            SaveError::DirectoryCreationFailed(_) => vec![
                "Check permissions on the parent directory".to_string(),
                "Ensure the parent path exists and is accessible".to_string(),
                "Try creating the directory manually first".to_string(),
            ],
            SaveError::PdfWriteFailed(_) => vec![
                "Verify that the character data is valid".to_string(),
                "Check that the PDF template file is not corrupted".to_string(),
                "Try the operation again - this may be a temporary issue".to_string(),
            ],
            SaveError::IntegrityCheckFailed(_) => vec![
                "Retry the save operation".to_string(),
                "Check available system resources (memory, disk space)".to_string(),
                "Verify the PDF template is not corrupted".to_string(),
            ],
        }
    }
}
