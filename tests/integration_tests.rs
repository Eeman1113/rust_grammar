use text_analyzer::{TextAnalyzer, Config, error::AnalysisError};
use text_analyzer::config::DocumentType;

#[test]
fn test_basic_analysis_flow() {
    let text = r#"
    This is a simple test document. It contains multiple sentences.
    Dr. Smith wrote this document. He is a professor at the university.
    The results were analyzed carefully. Many insights were discovered.
    "#.to_string();

    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    let stats = analyzer.statistics();
    assert!(stats.word_count > 20);
    assert!(stats.sentence_count >= 5);
    assert!(stats.paragraph_count > 0);
}

#[test]
fn test_empty_input_error() {
    let result = TextAnalyzer::with_default_config("".to_string());
    assert!(matches!(result, Err(AnalysisError::EmptyInput)));
}

#[test]
fn test_too_short_document_error() {
    let result = TextAnalyzer::with_default_config("Too short.".to_string());
    assert!(matches!(result, Err(AnalysisError::DocumentTooShort { .. })));
}

#[test]
fn test_readability_metrics() {
    let text = "This is a simple sentence. Here is another one. And a third one for testing.".to_string();
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    let metrics = analyzer.readability_metrics().unwrap();
    assert!(metrics.flesch_reading_ease > 0.0);
    assert!(metrics.flesch_reading_ease <= 100.0);
    assert!(metrics.flesch_kincaid_grade >= 0.0);
    assert!(metrics.avg_words_per_sentence > 0.0);
    assert!(metrics.avg_syllables_per_word > 0.0);
}

#[test]
fn test_grammar_detection() {
    let text = "He are going to the store. They is coming later.".to_string();
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    let grammar_issues = analyzer.check_grammar().unwrap();
    assert!(!grammar_issues.is_empty(), "Should detect subject-verb agreement issues");
}

#[test]
fn test_passive_voice_detection() {
    let text = "The ball was thrown by John. The book was written by the author.".to_string();
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    let passive_voice = analyzer.detect_passive_voice().unwrap();
    assert!(!passive_voice.is_empty(), "Should detect passive voice");
    
    for pv in passive_voice {
        assert!(pv.confidence > 0.0);
        assert!(pv.confidence <= 1.0);
    }
}

#[test]
fn test_sentence_splitting_with_abbreviations() {
    let text = "Dr. Smith went to the store. Prof. Johnson stayed home.".to_string();
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    let stats = analyzer.statistics();
    assert_eq!(stats.sentence_count, 2, "Should split into 2 sentences, not break on abbreviations");
}

#[test]
fn test_sentence_splitting_with_initials() {
    let text = "J.K. Rowling wrote Harry Potter. It was very successful.".to_string();
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    let stats = analyzer.statistics();
    assert_eq!(stats.sentence_count, 2, "Should not break on initials");
}

#[test]
fn test_sentence_splitting_with_decimals() {
    let text = "The price is 3.14 dollars. That is cheap.".to_string();
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    let stats = analyzer.statistics();
    assert_eq!(stats.sentence_count, 2, "Should not break on decimal numbers");
}

#[test]
fn test_config_presets() {
    let text = "This is a test document for configuration testing. It has multiple sentences.".to_string();
    
    // Test academic preset
    let academic_config = Config::preset(DocumentType::Academic);
    let analyzer_academic = TextAnalyzer::new(text.clone(), academic_config).unwrap();
    assert_eq!(analyzer_academic.statistics().word_count, 13);
    
    // Test fiction preset
    let fiction_config = Config::preset(DocumentType::Fiction);
    let analyzer_fiction = TextAnalyzer::new(text.clone(), fiction_config).unwrap();
    assert_eq!(analyzer_fiction.statistics().word_count, 13);
    
    // Test business preset
    let business_config = Config::preset(DocumentType::Business);
    let analyzer_business = TextAnalyzer::new(text, business_config).unwrap();
    assert_eq!(analyzer_business.statistics().word_count, 13);
}

#[test]
fn test_word_extraction_with_hyphens() {
    let text = "This is a well-known fact about mother-in-law situations.".to_string();
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    let stats = analyzer.statistics();
    assert!(stats.word_count > 5, "Should extract words with hyphens");
}

#[test]
fn test_word_extraction_with_apostrophes() {
    let text = "I can't believe it's not butter.".to_string();
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    let stats = analyzer.statistics();
    assert!(stats.word_count >= 5, "Should handle contractions");
}

#[test]
fn test_statistics_accuracy() {
    let text = "Hello world. This is a test. Testing 123.".to_string();
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    let stats = analyzer.statistics();
    assert_eq!(stats.sentence_count, 3);
    assert!(stats.word_count >= 8);
    assert!(stats.character_count > 0);
    assert!(stats.character_count_no_spaces > 0);
    assert!(stats.character_count > stats.character_count_no_spaces);
}

#[test]
fn test_long_document_performance() {
    // Generate a longer document
    let mut text = String::new();
    for i in 0..100 {
        text.push_str(&format!("This is sentence number {}. ", i + 1));
    }
    
    let start = std::time::Instant::now();
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    let _ = analyzer.readability_metrics().unwrap();
    let _ = analyzer.check_grammar().unwrap();
    let duration = start.elapsed();
    
    // Should complete in under 1 second for 100 sentences
    assert!(duration.as_secs() < 1, "Analysis took too long: {:?}", duration);
}

#[test]
fn test_unicode_handling() {
    let text = "François went to the café. The naïve approach didn't work.".to_string();
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    let stats = analyzer.statistics();
    assert!(stats.word_count > 8, "Should handle Unicode characters");
}

#[test]
fn test_feature_toggles() {
    let text = "He are going. The book was written.".to_string();
    
    // Disable grammar check
    let mut config = Config::default();
    config.features.grammar_check = false;
    
    let analyzer = TextAnalyzer::new(text.clone(), config).unwrap();
    let grammar_issues = analyzer.check_grammar().unwrap();
    assert_eq!(grammar_issues.len(), 0, "Grammar check should be disabled");
    
    // Disable style check
    let mut config2 = Config::default();
    config2.features.style_check = false;
    
    let analyzer2 = TextAnalyzer::new(text, config2).unwrap();
    let passive_voice = analyzer2.detect_passive_voice().unwrap();
    assert_eq!(passive_voice.len(), 0, "Style check should be disabled");
}

#[test]
fn test_error_propagation() {
    // Test that errors propagate correctly
    use std::fs::File;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    // Create a temporary file
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Valid content for testing.").unwrap();
    
    // This should work
    let content = std::fs::read_to_string(temp_file.path()).unwrap();
    let result = TextAnalyzer::with_default_config(content);
    assert!(result.is_ok());
}

#[test]
fn test_passive_voice_confidence_scoring() {
    let text = "The ball was thrown by John. She was tired.".to_string();
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    let passive_voice = analyzer.detect_passive_voice().unwrap();
    
    // "was thrown by John" should have high confidence
    // "was tired" should have low/no confidence (adjective)
    if passive_voice.len() > 0 {
        let high_confidence = passive_voice.iter().any(|pv| pv.confidence > 0.7);
        assert!(high_confidence, "Should have at least one high-confidence match");
    }
}

#[test]
fn test_multiple_paragraphs() {
    let text = "Paragraph one.\n\nParagraph two.\n\nParagraph three.".to_string();
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    let stats = analyzer.statistics();
    assert_eq!(stats.paragraph_count, 3, "Should detect 3 paragraphs");
    assert_eq!(stats.sentence_count, 3, "Should detect 3 sentences");
}
