use crate::error::{AnalysisError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub validation: ValidationSettings,
    pub analysis: AnalysisSettings,
    pub thresholds: ThresholdSettings,
    pub features: FeatureToggles,
    pub output: OutputSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSettings {
    pub max_file_size_mb: u64,
    pub min_words: usize,
    pub max_words: Option<usize>,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSettings {
    pub parallel_processing: bool,
    pub cache_results: bool,
    pub document_type: DocumentType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DocumentType {
    General,
    Academic,
    Fiction,
    Business,
    Technical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdSettings {
    /// Sticky sentences threshold (percentage of glue words)
    pub sticky_sentence_threshold: f64,
    
    /// Overused word frequency threshold (percentage)
    pub overused_word_threshold: f64,
    
    /// Echo detection distance (words apart)
    pub echo_distance: usize,
    
    /// Very long sentence threshold (word count)
    pub very_long_sentence: usize,
    
    /// Complex paragraph sentence length threshold
    pub complex_paragraph_sentence_length: f64,
    
    /// Complex paragraph syllables per word threshold
    pub complex_paragraph_syllables: f64,
    
    /// Passive voice severity threshold
    pub passive_voice_max: usize,
    
    /// Adverb count severity threshold
    pub adverb_max: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureToggles {
    pub grammar_check: bool,
    pub style_check: bool,
    pub readability_check: bool,
    pub consistency_check: bool,
    pub sensory_analysis: bool,
    pub cliche_detection: bool,
    pub jargon_detection: bool,
    pub echo_detection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputSettings {
    pub format: OutputFormat,
    pub verbosity: Verbosity,
    pub color: bool,
    pub show_progress: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Text,
    Json,
    Yaml,
    Html,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
    Debug,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            validation: ValidationSettings {
                max_file_size_mb: 10,
                min_words: 10,
                max_words: None,
                timeout_seconds: 300,
            },
            analysis: AnalysisSettings {
                parallel_processing: true,
                cache_results: false,
                document_type: DocumentType::General,
            },
            thresholds: ThresholdSettings {
                sticky_sentence_threshold: 40.0,
                overused_word_threshold: 0.5,
                echo_distance: 20,
                very_long_sentence: 30,
                complex_paragraph_sentence_length: 20.0,
                complex_paragraph_syllables: 1.8,
                passive_voice_max: 10,
                adverb_max: 20,
            },
            features: FeatureToggles {
                grammar_check: true,
                style_check: true,
                readability_check: true,
                consistency_check: true,
                sensory_analysis: true,
                cliche_detection: true,
                jargon_detection: true,
                echo_detection: true,
            },
            output: OutputSettings {
                format: OutputFormat::Text,
                verbosity: Verbosity::Normal,
                color: true,
                show_progress: true,
            },
        }
    }
}

impl Config {
    /// Load configuration from YAML file
    pub fn from_yaml<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| AnalysisError::ConfigError(format!("Failed to read config file: {}", e)))?;
        
        serde_yaml::from_str(&content)
            .map_err(|e| AnalysisError::ConfigError(format!("Failed to parse YAML config: {}", e)))
    }

    /// Load configuration from TOML file
    pub fn from_toml<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| AnalysisError::ConfigError(format!("Failed to read config file: {}", e)))?;
        
        toml::from_str(&content)
            .map_err(|e| AnalysisError::ConfigError(format!("Failed to parse TOML config: {}", e)))
    }

    /// Save configuration to YAML file
    pub fn save_yaml<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let yaml = serde_yaml::to_string(self)
            .map_err(|e| AnalysisError::ConfigError(format!("Failed to serialize config: {}", e)))?;
        
        fs::write(path.as_ref(), yaml)
            .map_err(|e| AnalysisError::ConfigError(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }

    /// Save configuration to TOML file
    pub fn save_toml<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| AnalysisError::ConfigError(format!("Failed to serialize config: {}", e)))?;
        
        fs::write(path.as_ref(), toml_str)
            .map_err(|e| AnalysisError::ConfigError(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }

    /// Get preset configuration for document type
    pub fn preset(doc_type: DocumentType) -> Self {
        let mut config = Self::default();
        config.analysis.document_type = doc_type.clone();

        match doc_type {
            DocumentType::Academic => {
                config.thresholds.passive_voice_max = 20; // More lenient
                config.thresholds.complex_paragraph_sentence_length = 25.0;
                config.features.jargon_detection = false;
            }
            DocumentType::Fiction => {
                config.features.sensory_analysis = true;
                config.thresholds.sticky_sentence_threshold = 35.0; // Stricter
                config.features.jargon_detection = false;
            }
            DocumentType::Business => {
                config.features.jargon_detection = true;
                config.thresholds.sticky_sentence_threshold = 45.0; // More lenient
            }
            DocumentType::Technical => {
                config.thresholds.complex_paragraph_sentence_length = 25.0;
                config.thresholds.passive_voice_max = 25;
                config.features.jargon_detection = false;
            }
            DocumentType::General => {}
        }

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.validation.min_words, 10);
        assert!(config.features.grammar_check);
    }

    #[test]
    fn test_yaml_serialization() {
        let config = Config::default();
        let yaml = serde_yaml::to_string(&config).unwrap();
        let deserialized: Config = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(config.validation.min_words, deserialized.validation.min_words);
    }

    #[test]
    fn test_preset_configs() {
        let academic = Config::preset(DocumentType::Academic);
        assert_eq!(academic.analysis.document_type, DocumentType::Academic);
        assert_eq!(academic.thresholds.passive_voice_max, 20);

        let fiction = Config::preset(DocumentType::Fiction);
        assert_eq!(fiction.analysis.document_type, DocumentType::Fiction);
        assert_eq!(fiction.thresholds.sticky_sentence_threshold, 35.0);
    }

    #[test]
    fn test_save_and_load_yaml() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let config = Config::default();
        
        config.save_yaml(temp_file.path()).unwrap();
        let loaded = Config::from_yaml(temp_file.path()).unwrap();
        
        assert_eq!(config.validation.min_words, loaded.validation.min_words);
    }

    #[test]
    fn test_save_and_load_toml() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let config = Config::default();
        
        config.save_toml(temp_file.path()).unwrap();
        let loaded = Config::from_toml(temp_file.path()).unwrap();
        
        assert_eq!(config.validation.min_words, loaded.validation.min_words);
    }
}
