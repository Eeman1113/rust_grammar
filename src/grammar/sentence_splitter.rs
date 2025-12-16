use crate::dictionaries::abbreviations::is_abbreviation;
use crate::error::Result;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    /// Regex for decimal numbers (3.14, 2.5, etc.)
    static ref DECIMAL_PATTERN: Regex = Regex::new(r"\d+\.\d+").unwrap();
    
    /// Regex for URLs
    static ref URL_PATTERN: Regex = Regex::new(
        r"(?:https?://|www\.)[^\s]+"
    ).unwrap();
    
    /// Regex for email addresses
    static ref EMAIL_PATTERN: Regex = Regex::new(
        r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b"
    ).unwrap();
    
    /// Regex for initials (J.K., U.S.A., etc.)
    static ref INITIALS_PATTERN: Regex = Regex::new(
        r"\b[A-Z]\.(?:[A-Z]\.)*"
    ).unwrap();
    
    /// Regex for ellipsis
    static ref ELLIPSIS_PATTERN: Regex = Regex::new(r"\.{3,}").unwrap();
}

/// Advanced sentence splitter with proper abbreviation, decimal, URL, and email handling
pub struct SentenceSplitter {
    min_sentence_length: usize,
}

impl Default for SentenceSplitter {
    fn default() -> Self {
        Self {
            min_sentence_length: 3,
        }
    }
}

impl SentenceSplitter {
    pub fn new(min_sentence_length: usize) -> Self {
        Self {
            min_sentence_length,
        }
    }

    /// Split text into sentences with comprehensive boundary detection
    pub fn split(&self, text: &str) -> Result<Vec<String>> {
        if text.trim().is_empty() {
            return Ok(Vec::new());
        }

        let mut sentences = Vec::new();
        let mut current_sentence = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];
            current_sentence.push(ch);

            // Check if this might be a sentence boundary
            if self.is_sentence_terminator(ch) {
                let context = self.extract_context(&chars, i);
                
                if self.is_sentence_boundary(&context, &current_sentence) {
                    // This is a real sentence boundary
                    let sentence = current_sentence.trim().to_string();
                    if sentence.len() >= self.min_sentence_length {
                        sentences.push(sentence);
                    }
                    current_sentence.clear();
                }
            }

            i += 1;
        }

        // Add any remaining text as a sentence
        let sentence = current_sentence.trim().to_string();
        if sentence.len() >= self.min_sentence_length {
            sentences.push(sentence);
        }

        Ok(sentences)
    }

    /// Check if character is a potential sentence terminator
    fn is_sentence_terminator(&self, ch: char) -> bool {
        matches!(ch, '.' | '!' | '?')
    }

    /// Extract context around a potential sentence boundary
    fn extract_context(&self, chars: &[char], pos: usize) -> SentenceContext {
        // Get word before the punctuation
        let before = self.get_word_before(chars, pos);
        
        // Get text after the punctuation (skip whitespace)
        let mut after_start = pos + 1;
        while after_start < chars.len() && chars[after_start].is_whitespace() {
            after_start += 1;
        }
        
        let after_char = if after_start < chars.len() {
            Some(chars[after_start])
        } else {
            None
        };

        // Check what comes after (next few characters)
        let after_text: String = chars[after_start..]
            .iter()
            .take(20)
            .collect();

        SentenceContext {
            punctuation: chars[pos],
            word_before: before,
            char_after: after_char,
            text_after: after_text,
            is_end_of_text: pos == chars.len() - 1,
        }
    }

    /// Get the word immediately before a position
    fn get_word_before(&self, chars: &[char], pos: usize) -> String {
        let mut i = pos;
        
        // Skip back past the punctuation and any whitespace
        while i > 0 {
            i -= 1;
            if !chars[i].is_whitespace() && chars[i] != '.' {
                break;
            }
        }
        
        // Collect the word
        let mut word_chars = Vec::new();
        while i > 0 && (chars[i].is_alphanumeric() || chars[i] == '.') {
            word_chars.push(chars[i]);
            if i == 0 {
                break;
            }
            i -= 1;
        }
        
        // Add the first character if we stopped mid-word
        if i == 0 && (chars[i].is_alphanumeric() || chars[i] == '.') {
            word_chars.push(chars[i]);
        }
        
        word_chars.reverse();
        word_chars.iter().collect()
    }

    /// Determine if this is a true sentence boundary
    fn is_sentence_boundary(&self, context: &SentenceContext, current_sentence: &str) -> bool {
        // Always boundary if end of text
        if context.is_end_of_text {
            return true;
        }

        // Always boundary for ! and ?
        if context.punctuation == '!' || context.punctuation == '?' {
            return self.check_next_char_capitalization(context);
        }

        // For periods, we need more sophisticated checks
        
        // Check for abbreviations
        if self.is_likely_abbreviation(&context.word_before) {
            return false;
        }

        // Check for initials (J.K., U.S.A.)
        if self.is_likely_initial(&context.word_before) {
            return false;
        }

        // Check for decimal numbers
        if self.is_decimal_number(current_sentence, context) {
            return false;
        }

        // Check for ellipsis
        if self.is_ellipsis(current_sentence) {
            return false;
        }

        // Check for URLs or emails
        if self.contains_url_or_email(current_sentence) {
            return false;
        }

        // Check if next character is uppercase (strong indicator of sentence boundary)
        if let Some(next_char) = context.char_after {
            if next_char.is_uppercase() {
                return true;
            }
            
            // Next char is lowercase - probably not a sentence boundary
            // unless it's a special case
            if next_char.is_lowercase() {
                return false;
            }
        }

        // Default to boundary if we're not sure
        true
    }

    /// Check if next character is capitalized (or other boundary indicators)
    fn check_next_char_capitalization(&self, context: &SentenceContext) -> bool {
        if let Some(next_char) = context.char_after {
            // Uppercase = sentence boundary
            if next_char.is_uppercase() {
                return true;
            }
            
            // Opening quote followed by uppercase = boundary
            if next_char == '"' || next_char == '\'' {
                // Look ahead for uppercase
                return context.text_after.chars().nth(1)
                    .map(|c| c.is_uppercase())
                    .unwrap_or(false);
            }
        }
        
        // No clear next character = boundary
        true
    }

    /// Check if word is likely an abbreviation
    fn is_likely_abbreviation(&self, word: &str) -> bool {
        if word.is_empty() {
            return false;
        }

        let word_clean = word.trim_end_matches('.');
        
        // Check against known abbreviations
        if is_abbreviation(word_clean) {
            return true;
        }

        // Single letters followed by period are likely initials/abbreviations
        if word_clean.len() == 1 && word_clean.chars().next().unwrap().is_uppercase() {
            return true;
        }

        false
    }

    /// Check if word is likely an initial (J., K., U.S., etc.)
    fn is_likely_initial(&self, word: &str) -> bool {
        if word.is_empty() {
            return false;
        }

        // Pattern: single uppercase letter followed by period
        if word.len() == 2 && word.chars().nth(0).unwrap().is_uppercase() && word.ends_with('.') {
            return true;
        }

        // Pattern: multiple single letters with periods (U.S.A.)
        INITIALS_PATTERN.is_match(word)
    }

    /// Check if this is part of a decimal number
    fn is_decimal_number(&self, sentence: &str, _context: &SentenceContext) -> bool {
        // Look for pattern like "3.14" in the last part of the sentence
        let last_part: String = sentence.chars().rev().take(10).collect::<String>()
            .chars().rev().collect();
        
        DECIMAL_PATTERN.is_match(&last_part)
    }

    /// Check for ellipsis
    fn is_ellipsis(&self, sentence: &str) -> bool {
        sentence.ends_with("...") || ELLIPSIS_PATTERN.is_match(&sentence.chars().rev().take(5).collect::<String>())
    }

    /// Check if sentence contains URL or email (don't split on their periods)
    fn contains_url_or_email(&self, sentence: &str) -> bool {
        let last_part: String = sentence.chars().rev().take(50).collect::<String>()
            .chars().rev().collect();
            
        URL_PATTERN.is_match(&last_part) || EMAIL_PATTERN.is_match(&last_part)
    }
}

#[derive(Debug)]
struct SentenceContext {
    punctuation: char,
    word_before: String,
    char_after: Option<char>,
    text_after: String,
    is_end_of_text: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_sentences() {
        let splitter = SentenceSplitter::default();
        let text = "This is a sentence. This is another sentence.";
        let sentences = splitter.split(text).unwrap();
        assert_eq!(sentences.len(), 2);
        assert_eq!(sentences[0], "This is a sentence.");
        assert_eq!(sentences[1], "This is another sentence.");
    }

    #[test]
    fn test_abbreviations() {
        let splitter = SentenceSplitter::default();
        let text = "Dr. Smith went to the store. He bought milk.";
        let sentences = splitter.split(text).unwrap();
        assert_eq!(sentences.len(), 2);
        assert!(sentences[0].contains("Dr. Smith"));
    }

    #[test]
    fn test_initials() {
        let splitter = SentenceSplitter::default();
        let text = "J.K. Rowling wrote Harry Potter. It was very popular.";
        let sentences = splitter.split(text).unwrap();
        assert_eq!(sentences.len(), 2);
        assert!(sentences[0].contains("J.K. Rowling"));
    }

    #[test]
    fn test_decimal_numbers() {
        let splitter = SentenceSplitter::default();
        let text = "The price is 3.14 dollars. That's cheap.";
        let sentences = splitter.split(text).unwrap();
        assert_eq!(sentences.len(), 2);
        assert!(sentences[0].contains("3.14"));
    }

    #[test]
    fn test_ellipsis() {
        let splitter = SentenceSplitter::default();
        let text = "I was thinking... Maybe we should go. What do you think?";
        let sentences = splitter.split(text).unwrap();
        assert_eq!(sentences.len(), 2);
        assert!(sentences[0].contains("..."));
    }

    #[test]
    fn test_question_and_exclamation() {
        let splitter = SentenceSplitter::default();
        let text = "Are you serious? I can't believe it! This is amazing.";
        let sentences = splitter.split(text).unwrap();
        assert_eq!(sentences.len(), 3);
    }

    #[test]
    fn test_multiple_abbreviations() {
        let splitter = SentenceSplitter::default();
        let text = "Prof. Johnson, Ph.D., works at MIT. He is very smart.";
        let sentences = splitter.split(text).unwrap();
        assert_eq!(sentences.len(), 2);
    }

    #[test]
    fn test_empty_input() {
        let splitter = SentenceSplitter::default();
        let sentences = splitter.split("").unwrap();
        assert_eq!(sentences.len(), 0);
    }
}
