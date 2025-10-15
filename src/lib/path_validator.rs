use std::path::{Path, PathBuf};
use std::fs;
use crate::lib::errors::SaveError;

pub struct PathValidator;

impl PathValidator {
    /// Validates a file path for PDF output operations
    pub fn validate_output_path(path: &str) -> Result<PathBuf, SaveError> {
        let path_obj = Path::new(path);
        
        // Convert to absolute path
        let absolute_path = if path_obj.is_absolute() {
            path_obj.to_path_buf()
        } else {
            std::env::current_dir()
                .map_err(|e| SaveError::InvalidPath(format!("Cannot get current directory: {}", e)))?
                .join(path_obj)
        };
        
        // Validate path components
        Self::validate_path_format(&absolute_path)?;
        Self::validate_filename(&absolute_path)?;
        Self::validate_path_length(&absolute_path)?;
        
        Ok(absolute_path)
    }
    
    /// Checks if the parent directory exists or can be created
    pub fn ensure_parent_directory(path: &Path) -> Result<(), SaveError> {
        let parent = path.parent()
            .ok_or_else(|| SaveError::InvalidPath("Path has no parent directory".to_string()))?;
            
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| SaveError::DirectoryCreationFailed(
                    format!("Cannot create directory {}: {}", parent.display(), e)
                ))?;
        }
        
        Ok(())
    }
    
    /// Checks write permissions for the target directory
    pub fn check_write_permissions(path: &Path) -> Result<(), SaveError> {
        let parent = path.parent()
            .ok_or_else(|| SaveError::InvalidPath("Path has no parent directory".to_string()))?;
            
        // Test write permissions by creating a temporary file
        let test_file = parent.join(".pdf_write_test");
        match fs::write(&test_file, b"test") {
            Ok(_) => {
                let _ = fs::remove_file(&test_file); // Clean up
                Ok(())
            }
            Err(e) => Err(SaveError::PermissionDenied(
                format!("Cannot write to directory {}: {}", parent.display(), e)
            ))
        }
    }
    
    /// Estimates available disk space (basic check)
    pub fn check_disk_space(path: &Path, estimated_size: u64) -> Result<(), SaveError> {
        let parent = path.parent()
            .ok_or_else(|| SaveError::InvalidPath("Path has no parent directory".to_string()))?;
            
        // Basic check - ensure directory is accessible
        match fs::metadata(parent) {
            Ok(_) => {
                // For a more sophisticated implementation, we could check actual available space
                // For now, we'll assume if we can access the directory, we have space
                // unless the estimated size is unreasonably large
                if estimated_size > 100_000_000 { // 100MB limit
                    return Err(SaveError::DiskSpaceFull(
                        "Estimated file size exceeds reasonable limits".to_string()
                    ));
                }
                Ok(())
            }
            Err(e) => Err(SaveError::DiskSpaceFull(
                format!("Cannot access directory {}: {}", parent.display(), e)
            ))
        }
    }
    
    fn validate_path_format(path: &Path) -> Result<(), SaveError> {
        let path_str = path.to_string_lossy();
        
        // Check for null characters
        if path_str.contains('\0') {
            return Err(SaveError::InvalidPath("Path contains null character".to_string()));
        }
        
        // Check for other problematic characters on different platforms
        #[cfg(windows)]
        {
            let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
            for &ch in &invalid_chars {
                if path_str.contains(ch) {
                    return Err(SaveError::InvalidPath(
                        format!("Path contains invalid character: {}", ch)
                    ));
                }
            }
        }
        
        Ok(())
    }
    
    fn validate_filename(path: &Path) -> Result<(), SaveError> {
        let filename = path.file_name()
            .ok_or_else(|| SaveError::InvalidPath("Path has no filename".to_string()))?
            .to_string_lossy();
            
        // Check filename is not empty
        if filename.is_empty() {
            return Err(SaveError::InvalidPath("Filename cannot be empty".to_string()));
        }
        
        // Check for PDF extension
        if !filename.to_lowercase().ends_with(".pdf") {
            return Err(SaveError::InvalidPath("Output file must have .pdf extension".to_string()));
        }
        
        // Check filename length
        if filename.len() > 255 {
            return Err(SaveError::InvalidPath("Filename is too long".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_path_length(path: &Path) -> Result<(), SaveError> {
        let path_str = path.to_string_lossy();
        
        // Check overall path length
        if path_str.len() > 4096 {
            return Err(SaveError::InvalidPath("Path is too long".to_string()));
        }
        
        Ok(())
    }
}
