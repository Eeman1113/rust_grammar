use std::path::PathBuf;
use thiserror::Error;

/// Custom error types for the text analyzer
#[derive(Error, Debug)]
pub enum AnalysisError {
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Input validation error: {0}")]
    ValidationError(String),

    #[error("UTF-8 encoding error: {0}")]
    EncodingError(#[from] std::string::FromUtf8Error),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("YAML serialization error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Analysis timeout: processing exceeded {0} seconds")]
    TimeoutError(u64),

    #[error("Regex compilation error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("File too large: {size} bytes (max: {max} bytes)")]
    FileTooLarge { size: u64, max: u64 },

    #[error("Document too short: {words} words (min: {min} words)")]
    DocumentTooShort { words: usize, min: usize },

    #[error("Empty input: the provided text is empty")]
    EmptyInput,

    #[error("Invalid file path: {0}")]
    InvalidPath(PathBuf),

    #[error("Processing error: {0}")]
    ProcessingError(String),
}

pub type Result<T> = std::result::Result<T, AnalysisError>;

/// Input validation constraints
pub struct ValidationConfig {
    pub max_file_size: u64,
    pub min_words: usize,
    pub max_words: Option<usize>,
    pub timeout_seconds: u64,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_file_size: 10 * 1024 * 1024, // 10MB
            min_words: 10,
            max_words: None,
            timeout_seconds: 300, // 5 minutes
        }
    }
}

impl ValidationConfig {
    /// Validate input text according to configuration
    pub fn validate_text(&self, text: &str) -> Result<()> {
        // Check if empty
        if text.trim().is_empty() {
            return Err(AnalysisError::EmptyInput);
        }

        // Check file size
        let size = text.len() as u64;
        if size > self.max_file_size {
            return Err(AnalysisError::FileTooLarge {
                size,
                max: self.max_file_size,
            });
        }

        // Check word count
        let word_count = text
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .count();

        if word_count < self.min_words {
            return Err(AnalysisError::DocumentTooShort {
                words: word_count,
                min: self.min_words,
            });
        }

        if let Some(max) = self.max_words {
            if word_count > max {
                return Err(AnalysisError::ValidationError(format!(
                    "Document too long: {} words (max: {} words)",
                    word_count, max
                )));
            }
        }

        // Validate UTF-8
        if !text.is_utf8_valid() {
            return Err(AnalysisError::ValidationError(
                "Invalid UTF-8 encoding detected".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate file path
    pub fn validate_path(&self, path: &PathBuf) -> Result<()> {
        if !path.exists() {
            return Err(AnalysisError::InvalidPath(path.clone()));
        }

        if !path.is_file() {
            return Err(AnalysisError::ValidationError(format!(
                "Path is not a file: {}",
                path.display()
            )));
        }

        Ok(())
    }
}

trait Utf8Validator {
    fn is_utf8_valid(&self) -> bool;
}

impl Utf8Validator for str {
    fn is_utf8_valid(&self) -> bool {
        std::str::from_utf8(self.as_bytes()).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input_validation() {
        let config = ValidationConfig::default();
        let result = config.validate_text("");
        assert!(matches!(result, Err(AnalysisError::EmptyInput)));
    }

    #[test]
    fn test_minimum_words_validation() {
        let config = ValidationConfig {
            min_words: 5,
            ..Default::default()
        };
        let result = config.validate_text("one two three");
        assert!(matches!(result, Err(AnalysisError::DocumentTooShort { .. })));
    }

    #[test]
    fn test_valid_text() {
        let config = ValidationConfig::default();
        let result = config.validate_text("This is a valid text with more than ten words for sure.");
        assert!(result.is_ok());
    }

    #[test]
    fn test_file_size_validation() {
        let config = ValidationConfig {
            max_file_size: 100,
            ..Default::default()
        };
        let large_text = "a".repeat(200);
        let result = config.validate_text(&large_text);
        assert!(matches!(result, Err(AnalysisError::FileTooLarge { .. })));
    }
}
