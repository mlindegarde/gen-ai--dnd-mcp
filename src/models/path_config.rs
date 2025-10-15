use std::path::{Path, PathBuf};
use std::fs;
use crate::lib::errors::SaveError;

#[derive(Debug, Clone)]
pub struct OutputPathConfig {
    pub target_path: PathBuf,
    pub parent_dir: PathBuf,
    pub filename: String,
    pub exists: bool,
}

impl OutputPathConfig {
    pub fn new(output_path: &str) -> Result<Self, SaveError> {
        let path = Path::new(output_path);
        
        // Convert to absolute path
        let target_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()
                .map_err(|e| SaveError::InvalidPath(format!("Cannot get current directory: {}", e)))?
                .join(path)
        };
        
        // Validate path format
        Self::validate_path_format(&target_path)?;
        
        // Extract parent directory and filename
        let parent_dir = target_path.parent()
            .ok_or_else(|| SaveError::InvalidPath("Path has no parent directory".to_string()))?
            .to_path_buf();
            
        let filename = target_path.file_name()
            .ok_or_else(|| SaveError::InvalidPath("Path has no filename".to_string()))?
            .to_string_lossy()
            .to_string();
            
        // Check if file already exists
        let exists = target_path.exists();
        
        Ok(OutputPathConfig {
            target_path,
            parent_dir,
            filename,
            exists,
        })
    }
    
    pub fn validate_permissions(&self) -> Result<(), SaveError> {
        // Check if parent directory exists or can be created
        if !self.parent_dir.exists() {
            // Try to create parent directories
            fs::create_dir_all(&self.parent_dir)
                .map_err(|e| SaveError::DirectoryCreationFailed(
                    format!("Cannot create directory {}: {}", self.parent_dir.display(), e)
                ))?;
        }
        
        // Check write permissions on parent directory
        let test_file = self.parent_dir.join(".write_test");
        match fs::write(&test_file, b"test") {
            Ok(_) => {
                let _ = fs::remove_file(&test_file); // Clean up
                Ok(())
            }
            Err(e) => Err(SaveError::PermissionDenied(
                format!("Cannot write to directory {}: {}", self.parent_dir.display(), e)
            ))
        }
    }
    
    pub fn check_disk_space(&self) -> Result<(), SaveError> {
        // Simple check - try to get available space
        // This is a basic implementation; more sophisticated checks could be added
        match fs::metadata(&self.parent_dir) {
            Ok(_) => Ok(()), // Directory exists and is accessible
            Err(e) => Err(SaveError::DiskSpaceFull(
                format!("Cannot access directory {}: {}", self.parent_dir.display(), e)
            ))
        }
    }
    
    fn validate_path_format(path: &Path) -> Result<(), SaveError> {
        let path_str = path.to_string_lossy();
        
        // Check for invalid characters (basic check)
        if path_str.contains('\0') {
            return Err(SaveError::InvalidPath("Path contains null character".to_string()));
        }
        
        // Check path length (reasonable limit)
        if path_str.len() > 4096 {
            return Err(SaveError::InvalidPath("Path is too long".to_string()));
        }
        
        // Ensure it ends with .pdf
        if !path_str.to_lowercase().ends_with(".pdf") {
            return Err(SaveError::InvalidPath("Output file must have .pdf extension".to_string()));
        }
        
        // Check filename is not empty
        if let Some(filename) = path.file_name() {
            if filename.is_empty() {
                return Err(SaveError::InvalidPath("Filename cannot be empty".to_string()));
            }
        }
        
        Ok(())
    }
}
