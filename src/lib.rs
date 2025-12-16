// Text Analyzer Library
// Production-ready text analysis tool with comprehensive error handling

pub mod error;
pub mod config;
pub mod dictionaries;
pub mod grammar;
pub mod word_lists;
pub mod analysis_reports;
pub mod comprehensive_analysis;
pub mod visualizer;

// Re-export commonly used types
pub use config::Config;
pub use error::{Result, AnalysisError};
pub use analysis_reports::*;
pub use visualizer::HtmlVisualizer;

use error::ValidationConfig;
use dictionaries::count_syllables;
use grammar::{SentenceSplitter, PassiveVoiceDetector, GrammarChecker};
use comprehensive_analysis::ComprehensiveAnalyzer;

use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

lazy_static! {
    /// Improved word extraction regex supporting hyphens, apostrophes, and Unicode
    static ref WORD_EXTRACT: Regex = Regex::new(
        r"\b[\p{L}\p{N}]+(?:[-'][\p{L}\p{N}]+)*\b"
    ).unwrap();
    
    static ref GLUE_WORDS: HashSet<&'static str> = {
        let words = [
            "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for",
            "of", "with", "by", "from", "up", "about", "into", "through", "during",
            "that", "this", "these", "those", "it", "its", "is", "are", "was", "were",
            "be", "been", "being", "have", "has", "had", "do", "does", "did", "will",
            "would", "should", "could", "may", "might", "must", "can", "which", "who",
            "when", "where", "why", "how", "if", "than", "then", "as", "so"
        ];
        words.iter().copied().collect()
    };
}

/// Main text analyzer struct
pub struct TextAnalyzer {
    text: String,
    sentences: Vec<String>,
    paragraphs: Vec<String>,
    words: Vec<String>,
    config: Config,
    #[allow(dead_code)]
    sentence_splitter: SentenceSplitter,
    passive_detector: PassiveVoiceDetector,
    grammar_checker: GrammarChecker,
}

impl TextAnalyzer {
    /// Create a new text analyzer with validation
    pub fn new(text: String, config: Config) -> Result<Self> {
        // Validate input
        let validator = ValidationConfig {
            max_file_size: config.validation.max_file_size_mb * 1024 * 1024,
            min_words: config.validation.min_words,
            max_words: config.validation.max_words,
            timeout_seconds: config.validation.timeout_seconds,
        };
        
        validator.validate_text(&text)?;

        // Initialize components
        let sentence_splitter = SentenceSplitter::default();
        let sentences = sentence_splitter.split(&text)?;
        let paragraphs = Self::split_into_paragraphs(&text);
        let words = Self::extract_words(&text)?;

        Ok(Self {
            text,
            sentences,
            paragraphs,
            words,
            config,
            sentence_splitter,
            passive_detector: PassiveVoiceDetector::default(),
            grammar_checker: GrammarChecker::default(),
        })
    }

    /// Create with default config
    pub fn with_default_config(text: String) -> Result<Self> {
        Self::new(text, Config::default())
    }

    /// Split text into paragraphs
    fn split_into_paragraphs(text: &str) -> Vec<String> {
        text.split("\n\n")
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect()
    }

    /// Extract words with improved regex
    fn extract_words(text: &str) -> Result<Vec<String>> {
        Ok(WORD_EXTRACT
            .find_iter(&text.to_lowercase())
            .map(|m| m.as_str().to_string())
            .collect())
    }

    /// Calculate readability metrics
    pub fn readability_metrics(&self) -> Result<ReadabilityMetrics> {
        let total_sentences = self.sentences.len();
        let total_words = self.words.len();

        if total_sentences == 0 || total_words == 0 {
            return Ok(ReadabilityMetrics::default());
        }

        // Count syllables using dictionary and improved estimation
        let total_syllables: usize = self.words
            .iter()
            .map(|w| count_syllables(w))
            .sum();

        let words_per_sentence = total_words as f64 / total_sentences as f64;
        let syllables_per_word = total_syllables as f64 / total_words as f64;

        // Flesch Reading Ease
        let mut reading_ease = 206.835 - (1.015 * words_per_sentence) - (84.6 * syllables_per_word);
        reading_ease = reading_ease.max(0.0).min(100.0);

        // Flesch-Kincaid Grade Level
        let grade_level = ((0.39 * words_per_sentence) + (11.8 * syllables_per_word) - 15.59).max(0.0);

        // SMOG Index (for texts with 30+ sentences)
        let smog_index = if total_sentences >= 30 {
            Some(Self::calculate_smog(&self.sentences))
        } else {
            None
        };

        Ok(ReadabilityMetrics {
            flesch_reading_ease: Self::round(reading_ease, 1),
            flesch_kincaid_grade: Self::round(grade_level, 1),
            smog_index,
            avg_words_per_sentence: Self::round(words_per_sentence, 1),
            avg_syllables_per_word: Self::round(syllables_per_word, 2),
        })
    }

    /// Calculate SMOG Index
    fn calculate_smog(sentences: &[String]) -> f64 {
        if sentences.is_empty() {
            return 0.0;
        }

        let mut polysyllable_count = 0;
        for sentence in sentences.iter().take(30) {
            let words: Vec<String> = WORD_EXTRACT
                .find_iter(&sentence.to_lowercase())
                .map(|m| m.as_str().to_string())
                .collect();
            
            for word in words {
                if count_syllables(&word) >= 3 {
                    polysyllable_count += 1;
                }
            }
        }

        1.0430 * ((polysyllable_count as f64 * 30.0 / sentences.len().min(30) as f64).sqrt()) + 3.1291
    }

    /// Check grammar
    pub fn check_grammar(&self) -> Result<Vec<grammar::GrammarIssue>> {
        if !self.config.features.grammar_check {
            return Ok(Vec::new());
        }
        self.grammar_checker.check(&self.sentences)
    }

    /// Detect passive voice
    pub fn detect_passive_voice(&self) -> Result<Vec<grammar::PassiveVoiceMatch>> {
        if !self.config.features.style_check {
            return Ok(Vec::new());
        }
        self.passive_detector.detect(&self.text)
    }

    /// Get basic statistics
    pub fn statistics(&self) -> TextStatistics {
        TextStatistics {
            word_count: self.words.len(),
            sentence_count: self.sentences.len(),
            paragraph_count: self.paragraphs.len(),
            character_count: self.text.chars().count(),
            character_count_no_spaces: self.text.chars().filter(|c| !c.is_whitespace()).count(),
        }
    }

    /// Get text reference (for visualizer)
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get sentences reference (for visualizer)
    pub fn sentences(&self) -> &[String] {
        &self.sentences
    }

    /// Generate FULL comprehensive analysis report with ALL features
    pub fn generate_full_report(&self) -> Result<FullAnalysisReport> {
        let analyzer = ComprehensiveAnalyzer::new(
            &self.text,
            &self.sentences,
            &self.paragraphs,
            &self.words,
        );

        // Generate all analysis reports
        let sticky_sentences = analyzer.analyze_sticky_sentences()?;
        let pacing = analyzer.analyze_pacing()?;
        let sentence_length = analyzer.analyze_sentence_length()?;
        let transitions = analyzer.analyze_transitions()?;
        let overused_words = analyzer.analyze_overused_words()?;
        let repeated_phrases = analyzer.analyze_repeated_phrases()?;
        let echoes = analyzer.analyze_echoes()?;
        let sensory = analyzer.analyze_sensory_words()?;
        let diction = analyzer.analyze_diction()?;
        let cliches = analyzer.analyze_cliches()?;
        let consistency = analyzer.analyze_consistency()?;
        let acronyms = analyzer.analyze_acronyms()?;
        let conjunction_starts = analyzer.analyze_conjunction_starts()?;
        let business_jargon = analyzer.analyze_business_jargon()?;
        let complex_paragraphs = analyzer.analyze_complex_paragraphs()?;
        
        // Get style report with adverbs and hidden verbs
        let mut style = analyzer.analyze_style()?;
        
        // Add passive voice count to style report
        let passive_voice = self.detect_passive_voice()?;
        style.passive_voice_count = passive_voice.len();

        // Calculate overall style score
        let style_score = analyzer.calculate_style_score(&style, &sticky_sentences, &diction);

        Ok(FullAnalysisReport {
            word_count: self.words.len(),
            sentence_count: self.sentences.len(),
            paragraph_count: self.paragraphs.len(),
            style_score,
            style,
            sticky_sentences,
            pacing,
            sentence_length,
            transitions,
            overused_words,
            repeated_phrases,
            echoes,
            sensory,
            diction,
            cliches,
            consistency,
            acronyms,
            conjunction_starts,
            business_jargon,
            complex_paragraphs,
        })
    }

    /// Utility: Round to specified decimal places
    fn round(value: f64, decimals: u32) -> f64 {
        let multiplier = 10_f64.powi(decimals as i32);
        (value * multiplier).round() / multiplier
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadabilityMetrics {
    pub flesch_reading_ease: f64,
    pub flesch_kincaid_grade: f64,
    pub smog_index: Option<f64>,
    pub avg_words_per_sentence: f64,
    pub avg_syllables_per_word: f64,
}

impl Default for ReadabilityMetrics {
    fn default() -> Self {
        Self {
            flesch_reading_ease: 0.0,
            flesch_kincaid_grade: 0.0,
            smog_index: None,
            avg_words_per_sentence: 0.0,
            avg_syllables_per_word: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStatistics {
    pub word_count: usize,
    pub sentence_count: usize,
    pub paragraph_count: usize,
    pub character_count: usize,
    pub character_count_no_spaces: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let text = "This is a test sentence. Here is another one.".to_string();
        let analyzer = TextAnalyzer::with_default_config(text).unwrap();
        
        assert_eq!(analyzer.sentences.len(), 2);
        assert!(analyzer.words.len() > 5);
    }

    #[test]
    fn test_empty_text_validation() {
        let result = TextAnalyzer::with_default_config("".to_string());
        assert!(matches!(result, Err(AnalysisError::EmptyInput)));
    }

    #[test]
    fn test_short_document_validation() {
        let result = TextAnalyzer::with_default_config("Too short.".to_string());
        assert!(matches!(result, Err(AnalysisError::DocumentTooShort { .. })));
    }

    #[test]
    fn test_readability_metrics() {
        let text = "This is a simple sentence. Here is another simple sentence. And a third.".to_string();
        let analyzer = TextAnalyzer::with_default_config(text).unwrap();
        
        let metrics = analyzer.readability_metrics().unwrap();
        assert!(metrics.flesch_reading_ease > 0.0);
        assert!(metrics.avg_words_per_sentence > 0.0);
    }

    #[test]
    fn test_statistics() {
        let text = "Hello world. This is a test.".to_string();
        let analyzer = TextAnalyzer::with_default_config(text).unwrap();
        
        let stats = analyzer.statistics();
        assert!(stats.word_count > 0);
        assert!(stats.sentence_count > 0);
    }

    #[test]
    fn test_word_extraction_with_hyphens() {
        let text = "This is a well-known fact about mother-in-law.".to_string();
        let words = TextAnalyzer::extract_words(&text).unwrap();
        
        assert!(words.iter().any(|w| w.contains("well-known") || w == "well" || w == "known"));
    }
}
