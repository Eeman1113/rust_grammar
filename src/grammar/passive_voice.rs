use crate::dictionaries::irregular_verbs::{
    is_irregular_past_participle, is_adjective_exception, is_linking_verb,
};
use crate::error::Result;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    /// Auxiliary verbs that form passive voice
    static ref PASSIVE_AUXILIARIES: Vec<&'static str> = vec![
        "am", "is", "are", "was", "were", "be", "been", "being",
        "get", "gets", "got", "gotten", "getting", // "get" passives
    ];

    /// Regex for regular -ed past participles
    static ref REGULAR_PARTICIPLE: Regex = Regex::new(r"\b\w+ed\b").unwrap();
    
    /// Regex for detecting "by" phrases (passive indicator)
    static ref BY_PHRASE: Regex = Regex::new(r"\bby\s+(?:the\s+)?[a-z]+").unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub struct PassiveVoiceMatch {
    pub text: String,
    pub confidence: f64, // 0.0 to 1.0
    pub position: usize,
    pub auxiliary: String,
    pub participle: String,
    pub has_by_phrase: bool,
    pub start_index: usize,
    pub end_index: usize,
    pub length: usize,
}

pub struct PassiveVoiceDetector {
    min_confidence: f64,
}

impl Default for PassiveVoiceDetector {
    fn default() -> Self {
        Self {
            min_confidence: 0.6, // Only report matches with >60% confidence
        }
    }
}

impl PassiveVoiceDetector {
    pub fn new(min_confidence: f64) -> Self {
        Self { min_confidence }
    }

    /// Detect passive voice in text with confidence scoring
    pub fn detect(&self, text: &str) -> Result<Vec<PassiveVoiceMatch>> {
        let mut matches = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();

        // Build character position map
        let mut char_positions = Vec::new();
        let mut search_from = 0;
        for word in &words {
            if let Some(pos) = text[search_from..].find(word) {
                let start = search_from + pos;
                let end = start + word.len();
                char_positions.push((start, end));
                search_from = end;
            } else {
                char_positions.push((search_from, search_from));
            }
        }

        for i in 0..words.len().saturating_sub(1) {
            let word1 = words[i].to_lowercase();
            let word2 = words[i + 1].to_lowercase();

            // Check if word1 is a passive auxiliary
            if PASSIVE_AUXILIARIES.contains(&word1.as_str()) {
                // Get the next word (potential participle)
                let participle = word2.trim_end_matches(|c: char| !c.is_alphanumeric());

                // Check if it's a valid past participle
                if self.is_likely_past_participle(participle) {
                    // Calculate confidence
                    let confidence = self.calculate_confidence(
                        &word1,
                        participle,
                        &words,
                        i,
                    );

                    if confidence >= self.min_confidence {
                        // Check for "by" phrase
                        let has_by_phrase = self.has_by_phrase_nearby(&words, i);

                        let match_text = format!("{} {}", words[i], words[i + 1]);
                        let start_index = char_positions[i].0;
                        let end_index = char_positions[i + 1].1;
                        let length = end_index - start_index;
                        
                        matches.push(PassiveVoiceMatch {
                            text: match_text,
                            confidence,
                            position: i,
                            auxiliary: word1,
                            participle: participle.to_string(),
                            has_by_phrase,
                            start_index,
                            end_index,
                            length,
                        });
                    }
                }
            }
        }

        Ok(matches)
    }

    /// Check if a word is likely a past participle
    fn is_likely_past_participle(&self, word: &str) -> bool {
        // Check irregular past participles
        if is_irregular_past_participle(word) {
            return true;
        }

        // Check regular -ed endings
        if word.ends_with("ed") && word.len() > 3 {
            // Exclude adjective exceptions
            if !is_adjective_exception(word) {
                return true;
            }
        }

        false
    }

    /// Calculate confidence score for passive voice detection
    fn calculate_confidence(
        &self,
        auxiliary: &str,
        participle: &str,
        words: &[&str],
        position: usize,
    ) -> f64 {
        let mut confidence: f64 = 0.5; // Base confidence

        // Higher confidence for typical passive auxiliaries
        if matches!(auxiliary, "was" | "were" | "been") {
            confidence += 0.2;
        }

        // Higher confidence for irregular participles (less ambiguous)
        if is_irregular_past_participle(participle) {
            confidence += 0.2;
        }

        // Lower confidence if it's a known adjective exception
        if is_adjective_exception(participle) {
            confidence -= 0.3;
        }

        // Check if followed by "by" phrase (strong passive indicator)
        if self.has_by_phrase_nearby(words, position) {
            confidence += 0.3;
        }

        // Check if auxiliary might be a linking verb
        if is_linking_verb(auxiliary) {
            confidence -= 0.2;
        }

        // Check context: is there a subject before the auxiliary?
        if position > 0 {
            let prev_word = words[position - 1].to_lowercase();
            // Common subjects increase confidence
            if matches!(
                prev_word.as_str(),
                "the" | "a" | "an" | "this" | "that" | "these" | "those" | "my" | "your" | "his" | "her" | "its" | "our" | "their"
            ) {
                confidence += 0.1;
            }
        }

        // Clamp confidence to [0.0, 1.0]
        confidence.max(0.0).min(1.0)
    }

    /// Check if there's a "by" phrase nearby (within 5 words)
    fn has_by_phrase_nearby(&self, words: &[&str], position: usize) -> bool {
        let start = position + 2;
        let end = (position + 7).min(words.len());

        for i in start..end {
            if words[i].to_lowercase() == "by" {
                // Make sure it's not just "by" at the end
                if i + 1 < words.len() {
                    let next_word = words[i + 1].to_lowercase();
                    // Check if followed by a noun-like word
                    if !matches!(next_word.as_str(), "the" | "a" | "an") {
                        return true;
                    }
                    // Check "by the/a/an NOUN"
                    if i + 2 < words.len() {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Count passive voice instances (above confidence threshold)
    pub fn count_passive_voice(&self, text: &str) -> Result<usize> {
        Ok(self.detect(text)?.len())
    }

    /// Get passive voice percentage in text
    pub fn passive_voice_percentage(&self, text: &str, total_sentences: usize) -> Result<f64> {
        if total_sentences == 0 {
            return Ok(0.0);
        }

        let count = self.count_passive_voice(text)?;
        Ok((count as f64 / total_sentences as f64) * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_passive_voice() {
        let detector = PassiveVoiceDetector::default();
        let text = "The ball was thrown by John.";
        let matches = detector.detect(text).unwrap();
        
        assert!(matches.len() > 0);
        assert!(matches[0].has_by_phrase);
        assert!(matches[0].confidence > 0.7);
    }

    #[test]
    fn test_irregular_participle() {
        let detector = PassiveVoiceDetector::default();
        let text = "The book was written by the author.";
        let matches = detector.detect(text).unwrap();
        
        assert!(matches.len() > 0);
        assert_eq!(matches[0].participle, "written");
    }

    #[test]
    fn test_adjective_exception() {
        let detector = PassiveVoiceDetector::default();
        let text = "She was tired after work.";
        let matches = detector.detect(text).unwrap();
        
        // Should have low confidence or no matches due to "tired" being an adjective
        if !matches.is_empty() {
            assert!(matches[0].confidence < 0.6);
        }
    }

    #[test]
    fn test_get_passive() {
        let detector = PassiveVoiceDetector::default();
        let text = "The document got reviewed by the team.";
        let matches = detector.detect(text).unwrap();
        
        assert!(matches.len() > 0);
    }

    #[test]
    fn test_no_passive() {
        let detector = PassiveVoiceDetector::default();
        let text = "John threw the ball.";
        let matches = detector.detect(text).unwrap();
        
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_linking_verb() {
        let detector = PassiveVoiceDetector::default();
        let text = "The soup seems cooked.";
        let matches = detector.detect(text).unwrap();
        
        // Should have low confidence due to "seems" being a linking verb
        if !matches.is_empty() {
            assert!(matches[0].confidence < 0.7);
        }
    }

    #[test]
    fn test_confidence_scoring() {
        let detector = PassiveVoiceDetector::default();
        
        // High confidence: clear passive with "by" phrase
        let text1 = "The house was built by workers.";
        let matches1 = detector.detect(text1).unwrap();
        assert!(matches1[0].confidence > 0.8);
        
        // Lower confidence: no "by" phrase
        let text2 = "The house was built.";
        let matches2 = detector.detect(text2).unwrap();
        if !matches2.is_empty() {
            assert!(matches2[0].confidence < matches1[0].confidence);
        }
    }
}
