use std::path::PathBuf;
use std::time::SystemTime;
use crate::lib::errors::SaveError;

#[derive(Debug, Clone)]
pub struct SaveOperationResult {
    pub success: bool,
    pub output_path: Option<PathBuf>,
    pub file_size_bytes: Option<u64>,
    pub created_at: Option<SystemTime>,
    pub error: Option<SaveError>,
    pub validation_passed: bool,
}

impl SaveOperationResult {
    pub fn success(output_path: PathBuf, file_size: u64) -> Self {
        SaveOperationResult {
            success: true,
            output_path: Some(output_path),
            file_size_bytes: Some(file_size),
            created_at: Some(SystemTime::now()),
            error: None,
            validation_passed: true,
        }
    }
    
    pub fn failure(error: SaveError) -> Self {
        SaveOperationResult {
            success: false,
            output_path: None,
            file_size_bytes: None,
            created_at: None,
            error: Some(error),
            validation_passed: false,
        }
    }
    
    pub fn error_type(&self) -> Option<&'static str> {
        self.error.as_ref().map(|e| e.error_type())
    }
    
    pub fn error_message(&self) -> String {
        match &self.error {
            Some(err) => err.to_string(),
            None => if self.success {
                "Operation completed successfully".to_string()
            } else {
                "Unknown error occurred".to_string()
            }
        }
    }
    
    pub fn suggestions(&self) -> Vec<String> {
        match &self.error {
            Some(err) => err.suggestions(),
            None => vec![]
        }
    }
    
    pub fn to_json_value(&self) -> serde_json::Value {
        use serde_json::json;
        
        let mut result = json!({
            "success": self.success,
            "validation_passed": self.validation_passed,
        });
        
        if let Some(path) = &self.output_path {
            result["output_path"] = json!(path.to_string_lossy());
        }
        
        if let Some(size) = self.file_size_bytes {
            result["file_size_bytes"] = json!(size);
        }
        
        if let Some(created) = self.created_at {
            if let Ok(duration) = created.duration_since(SystemTime::UNIX_EPOCH) {
                result["created_timestamp"] = json!(duration.as_secs());
            }
        }
        
        if let Some(error) = &self.error {
            result["error"] = json!({
                "type": error.error_type(),
                "message": error.to_string(),
                "suggestions": error.suggestions()
            });
        }
        
        result
    }
}
