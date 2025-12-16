use crate::error::Result;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref SUBJECT_VERB_PATTERNS: Vec<(Regex, &'static str)> = vec![
        // Singular subject with plural verb
        (Regex::new(r"\b(he|she|it)\s+(are|were|have)\b").unwrap(),
         "Singular subject with plural verb"),
        (Regex::new(r"\b(the\s+\w+)\s+(are|were|have)\b").unwrap(),
         "Possible singular subject with plural verb"),
        
        // Plural subject with singular verb  
        (Regex::new(r"\b(they|we|you)\s+(is|was|has)\b").unwrap(),
         "Plural subject with singular verb"),
        (Regex::new(r"\b(the\s+\w+s)\s+(is|was|has)\b").unwrap(),
         "Possible plural subject with singular verb"),
    ];
    
    static ref DOUBLE_NEGATIVE: Regex = Regex::new(
        r"\b(don't|doesn't|didn't|won't|can't|couldn't|shouldn't|wouldn't)\s+\w+\s+(no|nothing|nobody|never|nowhere|neither)\b"
    ).unwrap();
    
    static ref RUN_ON_INDICATORS: Regex = Regex::new(
        r",\s+(and|but|or|so)\s+\w+\s+\w+\s+,\s+(and|but|or|so)"
    ).unwrap();
    
    static ref DOUBLE_SPACE: Regex = Regex::new(r"  +").unwrap();
}

#[derive(Debug, Clone)]
pub struct GrammarIssue {
    pub issue_type: GrammarIssueType,
    pub message: String,
    pub sentence_num: usize,
    pub severity: Severity,
    pub start_index: usize,
    pub end_index: usize,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GrammarIssueType {
    SubjectVerbAgreement,
    DoubleNegative,
    RunOnSentence,
    SentenceFragment,
    CommaSplice,
    DoubleSpace,
    MissingPunctuation,
    PronoCase,
    VerbTense,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Severity {
    Low,
    Medium,
    High,
}

pub struct GrammarChecker {
    check_subject_verb: bool,
    check_double_negatives: bool,
    check_run_ons: bool,
}

impl Default for GrammarChecker {
    fn default() -> Self {
        Self {
            check_subject_verb: true,
            check_double_negatives: true,
            check_run_ons: true,
        }
    }
}

impl GrammarChecker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check(&self, sentences: &[String]) -> Result<Vec<GrammarIssue>> {
        let mut issues = Vec::new();
        
        // Reconstruct approximate positions (best effort without original text)
        let mut cumulative_pos = 0;

        for (i, sentence) in sentences.iter().enumerate() {
            let sentence_num = i + 1;
            let lower = sentence.to_lowercase();
            let sentence_start = cumulative_pos;
            let sentence_end = cumulative_pos + sentence.len();

            // Check for double spaces
            if let Some(mat) = DOUBLE_SPACE.find(sentence) {
                issues.push(GrammarIssue {
                    issue_type: GrammarIssueType::DoubleSpace,
                    message: "Double space detected".to_string(),
                    sentence_num,
                    severity: Severity::Low,
                    start_index: sentence_start + mat.start(),
                    end_index: sentence_start + mat.end(),
                    length: mat.end() - mat.start(),
                });
            }

            // Check for missing end punctuation
            if let Some(last_char) = sentence.trim().chars().last() {
                if !matches!(last_char, '.' | '!' | '?') {
                    let trimmed_len = sentence.trim().len();
                    issues.push(GrammarIssue {
                        issue_type: GrammarIssueType::MissingPunctuation,
                        message: "Missing end punctuation".to_string(),
                        sentence_num,
                        severity: Severity::Medium,
                        start_index: sentence_start + trimmed_len,
                        end_index: sentence_end,
                        length: sentence_end - (sentence_start + trimmed_len),
                    });
                }
            }

            // Check for subject-verb agreement
            if self.check_subject_verb {
                for (pattern, msg) in SUBJECT_VERB_PATTERNS.iter() {
                    if let Some(mat) = pattern.find(&lower) {
                        issues.push(GrammarIssue {
                            issue_type: GrammarIssueType::SubjectVerbAgreement,
                            message: msg.to_string(),
                            sentence_num,
                            severity: Severity::High,
                            start_index: sentence_start + mat.start(),
                            end_index: sentence_start + mat.end(),
                            length: mat.end() - mat.start(),
                        });
                    }
                }
            }

            // Check for double negatives
            if self.check_double_negatives {
                if let Some(mat) = DOUBLE_NEGATIVE.find(&lower) {
                    issues.push(GrammarIssue {
                        issue_type: GrammarIssueType::DoubleNegative,
                        message: "Double negative detected".to_string(),
                        sentence_num,
                        severity: Severity::High,
                        start_index: sentence_start + mat.start(),
                        end_index: sentence_start + mat.end(),
                        length: mat.end() - mat.start(),
                    });
                }
            }

            // Check for run-on sentences
            if self.check_run_ons {
                if let Some(mat) = RUN_ON_INDICATORS.find(&lower) {
                    issues.push(GrammarIssue {
                        issue_type: GrammarIssueType::RunOnSentence,
                        message: "Possible run-on sentence".to_string(),
                        sentence_num,
                        severity: Severity::Medium,
                        start_index: sentence_start + mat.start(),
                        end_index: sentence_start + mat.end(),
                        length: mat.end() - mat.start(),
                    });
                }
            }

            // Check for comma splices
            if self.check_comma_splice(sentence) {
                // Find comma position (approximation)
                if let Some(pos) = sentence.find(',') {
                    issues.push(GrammarIssue {
                        issue_type: GrammarIssueType::CommaSplice,
                        message: "Possible comma splice".to_string(),
                        sentence_num,
                        severity: Severity::Medium,
                        start_index: sentence_start + pos,
                        end_index: sentence_start + pos + 1,
                        length: 1,
                    });
                } else {
                    issues.push(GrammarIssue {
                        issue_type: GrammarIssueType::CommaSplice,
                        message: "Possible comma splice".to_string(),
                        sentence_num,
                        severity: Severity::Medium,
                        start_index: sentence_start,
                        end_index: sentence_end,
                        length: sentence.len(),
                    });
                }
            }
            
            // Update cumulative position for next sentence
            cumulative_pos = sentence_end + 1; // +1 for space/newline separator
        }

        Ok(issues)
    }

    fn check_comma_splice(&self, sentence: &str) -> bool {
        // Count independent clauses connected by commas
        let parts: Vec<&str> = sentence.split(',').collect();
        if parts.len() < 2 {
            return false;
        }

        // Check if there are multiple clauses with subjects and verbs
        let mut clause_count = 0;
        for part in &parts {
            if self.has_subject_and_verb(part) {
                clause_count += 1;
            }
        }

        // If more than 2 independent clauses, likely a comma splice
        clause_count > 2
    }

    fn has_subject_and_verb(&self, text: &str) -> bool {
        let words: Vec<&str> = text.split_whitespace().collect();
        words.len() >= 2 // Simplified check
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject_verb_agreement() {
        let checker = GrammarChecker::new();
        let sentences = vec!["He are going to the store.".to_string()];
        let issues = checker.check(&sentences).unwrap();
        
        assert!(!issues.is_empty());
        assert!(matches!(issues[0].issue_type, GrammarIssueType::SubjectVerbAgreement));
    }

    #[test]
    fn test_double_negative() {
        let checker = GrammarChecker::new();
        let sentences = vec!["I don't have nothing.".to_string()];
        let issues = checker.check(&sentences).unwrap();
        
        assert!(!issues.is_empty());
        assert!(matches!(issues[0].issue_type, GrammarIssueType::DoubleNegative));
    }

    #[test]
    fn test_double_space() {
        let checker = GrammarChecker::new();
        let sentences = vec!["This  has  double spaces.".to_string()];
        let issues = checker.check(&sentences).unwrap();
        
        assert!(!issues.is_empty());
    }

    #[test]
    fn test_missing_punctuation() {
        let checker = GrammarChecker::new();
        let sentences = vec!["This sentence has no ending".to_string()];
        let issues = checker.check(&sentences).unwrap();
        
        assert!(!issues.is_empty());
        assert!(matches!(issues[0].issue_type, GrammarIssueType::MissingPunctuation));
    }
}
