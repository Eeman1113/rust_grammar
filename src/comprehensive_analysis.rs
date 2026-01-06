use crate::word_lists::*;
use crate::dictionaries::count_syllables;
use crate::analysis_reports::*;
use crate::grammar::SentenceSplitter;
use crate::error::Result;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref ADVERBS: Regex = Regex::new(r"\b\w+ly\b").unwrap();
    static ref ACRONYMS: Regex = Regex::new(r"\b[A-Z]{2,}\b").unwrap();
    static ref FIRST_WORD: Regex = Regex::new(r"^\s*(\w+)").unwrap();
    static ref WORD_PATTERN: Regex = Regex::new(r"\b[\p{L}\p{N}]+(?:[-'][\p{L}\p{N}]+)*\b").unwrap();
}

pub struct ComprehensiveAnalyzer<'a> {
    text: &'a str,
    sentences: &'a [String],
    paragraphs: &'a [String],
    words: &'a [String],
}

impl<'a> ComprehensiveAnalyzer<'a> {
    pub fn new(
        text: &'a str,
        sentences: &'a [String],
        paragraphs: &'a [String],
        words: &'a [String],
    ) -> Self {
        Self {
            text,
            sentences,
            paragraphs,
            words,
        }
    }

    // ========== FEATURE 1: STICKY SENTENCES ==========
    pub fn analyze_sticky_sentences(&self) -> Result<StickySentencesReport> {
        let mut sticky_sentences = Vec::new();
        let mut semi_sticky_sentences = Vec::new();
        let mut total_glue = 0;
        let total_words = self.words.len();
        
        // Track cumulative position
        let mut cumulative_pos = 0;

        for (i, sentence) in self.sentences.iter().enumerate() {
            let sentence_start = cumulative_pos;
            let sentence_end = cumulative_pos + sentence.len();
            
            let words: Vec<String> = WORD_PATTERN
                .find_iter(&sentence.to_lowercase())
                .map(|m| m.as_str().to_string())
                .collect();

            if words.is_empty() {
                cumulative_pos = sentence_end + 1;
                continue;
            }

            let glue_count = words.iter().filter(|w| GLUE_WORDS.contains(w.as_str())).count();
            let glue_percentage = (glue_count as f64 / words.len() as f64) * 100.0;

            // Categorize: >45% = sticky, 35-45% = semi-sticky
            if glue_percentage > 45.0 {
                // Sticky sentence
                let truncated = if sentence.chars().count() > 100 {
                    let truncated: String = sentence.chars().take(100).collect();
                    format!("{}...", truncated)
                } else {
                    sentence.clone()
                };

                sticky_sentences.push(StickySentence {
                    sentence_num: i + 1,
                    glue_percentage: (glue_percentage * 10.0).round() / 10.0,
                    sentence: truncated,
                    start_index: sentence_start,
                    end_index: sentence_end,
                    length: sentence.len(),
                });
            } else if glue_percentage >= 35.0 && glue_percentage <= 45.0 {
                // Semi-sticky sentence
                let truncated = if sentence.chars().count() > 100 {
                    let truncated: String = sentence.chars().take(100).collect();
                    format!("{}...", truncated)
                } else {
                    sentence.clone()
                };

                semi_sticky_sentences.push(StickySentence {
                    sentence_num: i + 1,
                    glue_percentage: (glue_percentage * 10.0).round() / 10.0,
                    sentence: truncated,
                    start_index: sentence_start,
                    end_index: sentence_end,
                    length: sentence.len(),
                });
            }
            
            cumulative_pos = sentence_end + 1; // +1 for separator
        }

        // Calculate overall glue index
        for word in self.words {
            if GLUE_WORDS.contains(word.as_str()) {
                total_glue += 1;
            }
        }

        let overall_glue_index = if total_words > 0 {
            (total_glue as f64 / total_words as f64) * 100.0
        } else {
            0.0
        };

        let glue_index_rounded = (overall_glue_index * 10.0).round() / 10.0;

        Ok(StickySentencesReport {
            overall_glue_index: glue_index_rounded,
            glue_index: glue_index_rounded,  // Alias for backward compatibility
            sticky_sentence_count: sticky_sentences.len(),
            sticky_sentences,
            semi_sticky_sentences,
        })
    }

    // ========== FEATURE 2: PACING REPORT ==========
    pub fn analyze_pacing(&self) -> Result<PacingReport> {
        let sentence_lengths: Vec<usize> = self
            .sentences
            .iter()
            .map(|s| WORD_PATTERN.find_iter(s).count())
            .collect();

        let fast_paced = sentence_lengths.iter().filter(|&&l| l < 10).count();
        let medium_paced = sentence_lengths.iter().filter(|&&l| l >= 10 && l <= 20).count();
        let slow_paced = sentence_lengths.iter().filter(|&&l| l > 20).count();

        let total = sentence_lengths.len();

        let fast_paced_percentage = if total > 0 {
            (fast_paced as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let medium_paced_percentage = if total > 0 {
            (medium_paced as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let slow_paced_percentage = if total > 0 {
            (slow_paced as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        Ok(PacingReport {
            fast_paced_percentage: (fast_paced_percentage * 10.0).round() / 10.0,
            medium_paced_percentage: (medium_paced_percentage * 10.0).round() / 10.0,
            slow_paced_percentage: (slow_paced_percentage * 10.0).round() / 10.0,
            pacing_distribution: PacingDistribution {
                fast: fast_paced,
                medium: medium_paced,
                slow: slow_paced,
            },
        })
    }

    // ========== FEATURE 3: SENTENCE LENGTH & VARIETY ==========
    pub fn analyze_sentence_length(&self) -> Result<SentenceLengthReport> {
        let sentence_lengths: Vec<usize> = self
            .sentences
            .iter()
            .map(|s| WORD_PATTERN.find_iter(s).count())
            .collect();

        if sentence_lengths.is_empty() {
            return Ok(SentenceLengthReport {
                avg_length: 0.0,
                std_deviation: 0.0,
                variety_score: 0.0,
                shortest: 0,
                longest: 0,
                very_long_sentences: 0,
                very_long_details: Vec::new(),
            });
        }

        let avg_length = sentence_lengths.iter().sum::<usize>() as f64 / sentence_lengths.len() as f64;

        let variance = sentence_lengths
            .iter()
            .map(|&len| {
                let diff = len as f64 - avg_length;
                diff * diff
            })
            .sum::<f64>()
            / sentence_lengths.len() as f64;

        let std_dev = variance.sqrt();
        let variety_score = (std_dev / 2.0).min(10.0);

        let shortest = *sentence_lengths.iter().min().unwrap_or(&0);
        let longest = *sentence_lengths.iter().max().unwrap_or(&0);

        let very_long: Vec<(usize, usize)> = sentence_lengths
            .iter()
            .enumerate()
            .filter(|(_, &len)| len > 30)
            .map(|(i, &len)| (i + 1, len))
            .collect();

        let very_long_sentences = very_long.len();

        Ok(SentenceLengthReport {
            avg_length: (avg_length * 10.0).round() / 10.0,
            std_deviation: (std_dev * 10.0).round() / 10.0,
            variety_score: (variety_score * 10.0).round() / 10.0,
            shortest,
            longest,
            very_long_sentences,
            very_long_details: very_long,
        })
    }

    // ========== FEATURE 4: TRANSITION ANALYSIS ==========
    pub fn analyze_transitions(&self) -> Result<TransitionReport> {
        let mut all_transitions = Vec::new();
        let mut sentences_with_transitions = 0;
        let mut transition_counts: HashMap<String, usize> = HashMap::new();
        
        // Track cumulative position for sentences
        let mut cumulative_pos = 0;

        for (sent_idx, sentence) in self.sentences.iter().enumerate() {
            let sentence_num = sent_idx + 1;
            let sentence_start = cumulative_pos;
            let sentence_lower = sentence.to_lowercase();
            let mut found_in_sentence = false;

            // Check single-word transitions
            for &transition in TRANSITION_WORDS.iter() {
                let mut start = 0;
                while let Some(pos) = sentence_lower[start..].find(transition) {
                    let actual_pos = start + pos;
                    // Make sure it's a whole word match
                    let is_word_boundary = (actual_pos == 0 || !sentence_lower.chars().nth(actual_pos - 1).unwrap_or(' ').is_alphanumeric())
                        && (actual_pos + transition.len() >= sentence_lower.len() 
                            || !sentence_lower.chars().nth(actual_pos + transition.len()).unwrap_or(' ').is_alphanumeric());
                    
                    if is_word_boundary {
                        found_in_sentence = true;
                        *transition_counts.entry(transition.to_string()).or_insert(0) += 1;
                        
                        all_transitions.push(TransitionFound {
                            transition: transition.to_string(),
                            sentence_num,
                            start_index: sentence_start + actual_pos,
                            end_index: sentence_start + actual_pos + transition.len(),
                            length: transition.len(),
                        });
                    }
                    start = actual_pos + 1;
                }
            }

            // Check multi-word transitions
            for &phrase in TRANSITION_PHRASES.iter() {
                let mut start = 0;
                while let Some(pos) = sentence_lower[start..].find(phrase) {
                    let actual_pos = start + pos;
                    found_in_sentence = true;
                    *transition_counts.entry(phrase.to_string()).or_insert(0) += 1;
                    
                    all_transitions.push(TransitionFound {
                        transition: phrase.to_string(),
                        sentence_num,
                        start_index: sentence_start + actual_pos,
                        end_index: sentence_start + actual_pos + phrase.len(),
                        length: phrase.len(),
                    });
                    
                    start = actual_pos + 1;
                }
            }

            if found_in_sentence {
                sentences_with_transitions += 1;
            }
            
            cumulative_pos = sentence_start + sentence.len() + 1; // +1 for separator
        }

        let total_sentences = self.sentences.len();
        let transition_percentage = if total_sentences > 0 {
            (sentences_with_transitions as f64 / total_sentences as f64) * 100.0
        } else {
            0.0
        };

        let unique_transitions = transition_counts.len();
        let mut most_common: Vec<(String, usize)> = transition_counts.into_iter().collect();
        most_common.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(TransitionReport {
            sentences_with_transitions,
            transition_percentage: (transition_percentage * 10.0).round() / 10.0,
            total_transitions_used: all_transitions.len(),
            unique_transitions,
            most_common_transitions: most_common,
            all_transitions,
        })
    }

    // ========== FEATURE 5: OVERUSED WORDS ==========
    pub fn analyze_overused_words(&self) -> Result<OverusedWordsReport> {
        let mut word_counts: HashMap<String, Vec<usize>> = HashMap::new();
        let text_lower = self.text.to_lowercase();
        
        // Track each word position
        let words_with_pos: Vec<_> = WORD_PATTERN
            .find_iter(&text_lower)
            .map(|m| (m.as_str().to_string(), m.start(), m.end()))
            .collect();
        
        // Build word -> positions map
        for (word, start, _end) in words_with_pos {
            word_counts
                .entry(word)
                .or_insert_with(Vec::new)
                .push(start);
        }

        let total_words = self.words.len();
        let mut overused = Vec::new();
        let total_unique_words = word_counts.len();

        for (word, positions) in word_counts {
            let count = positions.len();
            if !GLUE_WORDS.contains(word.as_str()) && word.len() > 3 {
                let frequency = (count as f64 / total_words as f64) * 100.0;
                if frequency > 0.5 {
                    let occurrences: Vec<WordOccurrence> = positions
                        .iter()
                        .map(|&start| {
                            let length = word.len();
                            WordOccurrence {
                                start_index: start,
                                end_index: start + length,
                                length,
                            }
                        })
                        .collect();
                    
                    overused.push(OverusedWord {
                        word: word.clone(),
                        count,
                        frequency: (frequency * 100.0).round() / 100.0,
                        occurrences,
                    });
                }
            }
        }

        Ok(OverusedWordsReport {
            overused_words: overused,
            total_unique_words,
        })
    }

    // ========== FEATURE 6: REPEATED PHRASES ==========
    pub fn analyze_repeated_phrases(&self) -> Result<RepeatedPhrasesReport> {
        let text_lower = self.text.to_lowercase();
        let mut phrase_positions: HashMap<String, Vec<usize>> = HashMap::new();
        
        // Extract words with positions
        let words_with_pos: Vec<_> = WORD_PATTERN
            .find_iter(&text_lower)
            .map(|m| (m.as_str().to_string(), m.start(), m.end()))
            .collect();

        // Check 2-word, 3-word, and 4-word phrases
        for phrase_length in 2..=4 {
            // Skip if we don't have enough words for this phrase length
            if words_with_pos.len() < phrase_length {
                continue;
            }
            
            for i in 0..=words_with_pos.len() - phrase_length {
                let phrase_words: Vec<String> = words_with_pos[i..i + phrase_length]
                    .iter()
                    .map(|(w, _, _)| w.clone())
                    .collect();
                let phrase = phrase_words.join(" ");
                let start_pos = words_with_pos[i].1;
                
                phrase_positions
                    .entry(phrase)
                    .or_insert_with(Vec::new)
                    .push(start_pos);
            }
        }

        // Filter to only those that appear more than once
        let mut repeats: Vec<(String, Vec<usize>)> = phrase_positions
            .into_iter()
            .filter(|(_, positions)| positions.len() > 1)
            .collect();

        repeats.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        let total_repeated_phrases = repeats.len();
        
        // Convert to RepeatedPhrase struct with positions
        let most_repeated: Vec<RepeatedPhrase> = repeats
            .into_iter()
            .take(50)
            .map(|(phrase, positions)| {
                let phrase_len = phrase.len();
                let occurrences: Vec<WordOccurrence> = positions
                    .iter()
                    .map(|&start| WordOccurrence {
                        start_index: start,
                        end_index: start + phrase_len,
                        length: phrase_len,
                    })
                    .collect();
                
                RepeatedPhrase {
                    phrase,
                    count: occurrences.len(),
                    occurrences,
                }
            })
            .collect();

        Ok(RepeatedPhrasesReport {
            total_repeated_phrases,
            most_repeated,
        })
    }

    // ========== FEATURE 7: ECHOES ==========
    pub fn analyze_echoes(&self) -> Result<EchoesReport> {
        let mut echoes = Vec::new();

        for (para_num, paragraph) in self.paragraphs.iter().enumerate() {
            let para_lower = paragraph.to_lowercase();
            
            // Extract words with their character positions
            let words_with_pos: Vec<_> = WORD_PATTERN
                .find_iter(&para_lower)
                .filter(|m| m.as_str().len() >= 4)
                .map(|m| (m.as_str().to_string(), m.start(), m.end()))
                .collect();

            // Group by word
            let mut word_positions: HashMap<String, Vec<(usize, usize, usize)>> = HashMap::new();
            for (word_idx, (word, start, end)) in words_with_pos.iter().enumerate() {
                word_positions
                    .entry(word.clone())
                    .or_insert_with(Vec::new)
                    .push((word_idx, *start, *end));
            }

            for (word, positions) in word_positions.into_iter() {
                if positions.len() > 1 && !GLUE_WORDS.contains(word.as_str()) {
                    for i in 0..positions.len() - 1 {
                        let (idx1, _, _) = positions[i];
                        let (idx2, _, _) = positions[i + 1];
                        let distance = idx2 - idx1;
                        
                        if distance < 20 {
                            // Create WordOccurrence for each position
                            let char_positions: Vec<WordOccurrence> = positions
                                .iter()
                                .map(|(_, start, end)| WordOccurrence {
                                    start_index: *start,
                                    end_index: *end,
                                    length: end - start,
                                })
                                .collect();
                            
                            echoes.push(Echo {
                                word,
                                paragraph: para_num + 1,
                                distance,
                                occurrences: positions.len(),
                                positions: char_positions,
                            });
                            break;
                        }
                    }
                }
            }
        }

        echoes.sort_by_key(|e| e.distance);
        let total_echoes = echoes.len();

        Ok(EchoesReport {
            total_echoes,
            echoes: echoes.into_iter().take(50).collect(),
        })
    }

    // ========== FEATURE 8: SENSORY REPORT ==========
    pub fn analyze_sensory_words(&self) -> Result<SensoryReport> {
        let mut sensory_usage: HashMap<String, usize> = HashMap::new();
        let mut sensory_words_found: HashMap<String, std::collections::HashSet<String>> =
            HashMap::new();

        for (sense, words) in SENSORY_WORDS.iter() {
            sensory_usage.insert(sense.to_string(), 0);
            sensory_words_found.insert(sense.to_string(), std::collections::HashSet::new());

            for word in self.words {
                if words.contains(word.as_str()) {
                    *sensory_usage.get_mut(*sense).unwrap() += 1;
                    sensory_words_found
                        .get_mut(*sense)
                        .unwrap()
                        .insert(word.clone());
                }
            }
        }

        let total_sensory: usize = sensory_usage.values().sum();
        let total_words = self.words.len();
        let sensory_percentage = if total_words > 0 {
            (total_sensory as f64 / total_words as f64) * 100.0
        } else {
            0.0
        };

        let mut by_sense = HashMap::new();
        for (sense, &count) in &sensory_usage {
            let percentage = if total_sensory > 0 {
                (count as f64 / total_sensory as f64) * 100.0
            } else {
                0.0
            };

            by_sense.insert(
                sense.clone(),
                SenseData {
                    count,
                    percentage: (percentage * 10.0).round() / 10.0,
                    unique_words: sensory_words_found[sense].len(),
                },
            );
        }

        Ok(SensoryReport {
            sensory_word_count: total_sensory,
            sensory_percentage: (sensory_percentage * 100.0).round() / 100.0,
            by_sense,
        })
    }

    // ========== FEATURE 9: DICTION (VAGUE WORDS) ==========
    pub fn analyze_diction(&self) -> Result<DictionReport> {
        let text_lower = self.text.to_lowercase();
        let mut vague_positions: HashMap<String, Vec<usize>> = HashMap::new();

        // Check single words with positions
        for mat in WORD_PATTERN.find_iter(&text_lower) {
            let word = mat.as_str();
            if VAGUE_WORDS.contains(word) {
                vague_positions
                    .entry(word.to_string())
                    .or_insert_with(Vec::new)
                    .push(mat.start());
            }
        }

        // Check phrases with positions
        for &phrase in VAGUE_PHRASES.iter() {
            let mut start = 0;
            while let Some(pos) = text_lower[start..].find(phrase) {
                let actual_pos = start + pos;
                vague_positions
                    .entry(phrase.to_string())
                    .or_insert_with(Vec::new)
                    .push(actual_pos);
                start = actual_pos + 1;
            }
        }

        let total_vague_words: usize = vague_positions.values().map(|v| v.len()).sum();
        let unique_vague_words = vague_positions.len();
        
        // Convert to VagueWord struct with positions
        let mut most_common_vague: Vec<VagueWord> = vague_positions
            .into_iter()
            .map(|(word, positions)| {
                let word_len = word.len();
                let occurrences: Vec<WordOccurrence> = positions
                    .iter()
                    .map(|&start| WordOccurrence {
                        start_index: start,
                        end_index: start + word_len,
                        length: word_len,
                    })
                    .collect();
                
                VagueWord {
                    word,
                    count: occurrences.len(),
                    occurrences,
                }
            })
            .collect();
        
        most_common_vague.sort_by(|a, b| b.count.cmp(&a.count));

        Ok(DictionReport {
            total_vague_words,
            unique_vague_words,
            most_common_vague,
        })
    }

    // ========== FEATURE 10: CLICHÉS ==========
    pub fn analyze_cliches(&self) -> Result<ClichesReport> {
        let text_lower = self.text.to_lowercase();
        let mut cliches_found = Vec::new();

        for &cliche in CLICHES.iter() {
            let mut positions = Vec::new();
            let mut start = 0;
            
            // Find all occurrences of this cliché
            while let Some(pos) = text_lower[start..].find(cliche) {
                let actual_pos = start + pos;
                positions.push(actual_pos);
                start = actual_pos + 1;
            }
            
            if !positions.is_empty() {
                let cliche_len = cliche.len();
                let occurrences: Vec<WordOccurrence> = positions
                    .iter()
                    .map(|&start| WordOccurrence {
                        start_index: start,
                        end_index: start + cliche_len,
                        length: cliche_len,
                    })
                    .collect();
                
                cliches_found.push(ClicheFound {
                    cliche: cliche.to_string(),
                    count: occurrences.len(),
                    occurrences,
                });
            }
        }

        let total_cliches = cliches_found.len();

        Ok(ClichesReport {
            total_cliches,
            cliches: cliches_found,
        })
    }

    // ========== FEATURE 11: CONSISTENCY CHECK ==========
    pub fn analyze_consistency(&self) -> Result<ConsistencyReport> {
        let mut issues = Vec::new();
        let text_lower = self.text.to_lowercase();

        // Check for US vs UK spelling variations
        for (us_word, uk_word) in US_UK_PAIRS.iter() {
            let has_us = text_lower.contains(us_word);
            let has_uk = text_lower.contains(uk_word);
            if has_us && has_uk {
                issues.push(format!(
                    "Mixed spelling: Both '{}' (US) and '{}' (UK) found",
                    us_word, uk_word
                ));
            }
        }

        // Check for inconsistent hyphenation
        for (word1, word2) in HYPHEN_PATTERNS.iter() {
            let has_word1 = text_lower.contains(word1);
            let has_word2 = text_lower.contains(word2);
            if has_word1 && has_word2 {
                issues.push(format!(
                    "Inconsistent hyphenation: Both '{}' and '{}' found",
                    word1, word2
                ));
            }
        }

        // Check for inconsistent capitalization
        let words_in_text: Vec<String> = WORD_PATTERN
            .find_iter(self.text)
            .map(|m| m.as_str().to_string())
            .collect();

        let mut word_variations: HashMap<String, std::collections::HashSet<String>> =
            HashMap::new();

        for word in words_in_text {
            word_variations
                .entry(word.to_lowercase())
                .or_insert_with(std::collections::HashSet::new)
                .insert(word);
        }

        for (lower_word, variations) in word_variations {
            if variations.len() > 1 && lower_word.len() > 3 {
                let non_capitalized: Vec<_> = variations
                    .iter()
                    .filter(|v| !v.chars().next().unwrap().is_uppercase())
                    .collect();
                let capitalized: Vec<_> = variations
                    .iter()
                    .filter(|v| {
                        let first = v.chars().next().unwrap();
                        first.is_uppercase() && !v.chars().all(|c| c.is_uppercase())
                    })
                    .collect();

                if !non_capitalized.is_empty() && !capitalized.is_empty() {
                    let mut sorted_variations: Vec<_> = variations.iter().collect();
                    sorted_variations.sort();
                    issues.push(format!(
                        "Inconsistent capitalization: {}",
                        sorted_variations
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }
        }

        let total_issues = issues.len();

        Ok(ConsistencyReport {
            total_issues,
            issues,
        })
    }

    // ========== FEATURE 12: ACRONYM REPORT ==========
    pub fn analyze_acronyms(&self) -> Result<AcronymReport> {
        let acronyms: Vec<String> = ACRONYMS
            .find_iter(self.text)
            .map(|m| m.as_str().to_string())
            .collect();

        let mut acronym_counts: HashMap<String, usize> = HashMap::new();
        for acronym in &acronyms {
            *acronym_counts.entry(acronym.clone()).or_insert(0) += 1;
        }

        let mut acronym_list: Vec<(String, usize)> = acronym_counts.into_iter().collect();
        acronym_list.sort_by(|a, b| b.1.cmp(&a.1));

        let total_acronyms = acronyms.len();
        let unique_acronyms = acronym_list.len();

        Ok(AcronymReport {
            total_acronyms,
            unique_acronyms,
            acronym_list,
        })
    }

    // ========== FEATURE 13: BUSINESS JARGON ==========
    pub fn analyze_business_jargon(&self) -> Result<BusinessJargonReport> {
        let mut jargon_found = Vec::new();
        let text_lower = self.text.to_lowercase();

        // Check single-word jargon with positions
        for &jargon in BUSINESS_JARGON.iter() {
            let mut positions = Vec::new();
            let mut start = 0;
            
            while let Some(pos) = text_lower[start..].find(jargon) {
                let actual_pos = start + pos;
                positions.push(actual_pos);
                start = actual_pos + 1;
            }
            
            if !positions.is_empty() {
                let jargon_len = jargon.len();
                let occurrences: Vec<WordOccurrence> = positions
                    .iter()
                    .map(|&start| WordOccurrence {
                        start_index: start,
                        end_index: start + jargon_len,
                        length: jargon_len,
                    })
                    .collect();
                
                jargon_found.push(JargonFound {
                    jargon: jargon.to_string(),
                    count: occurrences.len(),
                    occurrences,
                });
            }
        }

        // Check multi-word jargon phrases with positions
        for &phrase in BUSINESS_JARGON_PHRASES.iter() {
            let mut positions = Vec::new();
            let mut start = 0;
            
            while let Some(pos) = text_lower[start..].find(phrase) {
                let actual_pos = start + pos;
                positions.push(actual_pos);
                start = actual_pos + 1;
            }
            
            if !positions.is_empty() {
                let phrase_len = phrase.len();
                let occurrences: Vec<WordOccurrence> = positions
                    .iter()
                    .map(|&start| WordOccurrence {
                        start_index: start,
                        end_index: start + phrase_len,
                        length: phrase_len,
                    })
                    .collect();
                
                jargon_found.push(JargonFound {
                    jargon: phrase.to_string(),
                    count: occurrences.len(),
                    occurrences,
                });
            }
        }

        let total_jargon: usize = jargon_found.iter().map(|j| j.count).sum();
        let unique_jargon_phrases = jargon_found.len();

        Ok(BusinessJargonReport {
            total_jargon,
            unique_jargon_phrases,
            jargon_list: jargon_found,
        })
    }

    // ========== FEATURE 14: COMPLEX PARAGRAPHS ==========
    pub fn analyze_complex_paragraphs(&self) -> Result<ComplexParagraphsReport> {
        let mut complex_paragraphs = Vec::new();
        let splitter = SentenceSplitter::default();
        
        // Track cumulative position for paragraphs
        let mut cumulative_pos = 0;

        for (i, paragraph) in self.paragraphs.iter().enumerate() {
            let para_start = cumulative_pos;
            let para_end = cumulative_pos + paragraph.len();
            
            let sentences = splitter.split(paragraph)?;
            let words: Vec<String> = WORD_PATTERN
                .find_iter(&paragraph.to_lowercase())
                .map(|m| m.as_str().to_string())
                .collect();

            if !sentences.is_empty() && !words.is_empty() {
                let avg_sentence_length = words.len() as f64 / sentences.len() as f64;
                let syllables: usize = words.iter().map(|w| count_syllables(w)).sum();
                let avg_syllables = syllables as f64 / words.len() as f64;

                if avg_sentence_length > 20.0 && avg_syllables > 1.8 {
                    complex_paragraphs.push(ComplexParagraph {
                        paragraph_num: i + 1,
                        avg_sentence_length: (avg_sentence_length * 10.0).round() / 10.0,
                        avg_syllables: (avg_syllables * 100.0).round() / 100.0,
                        start_index: para_start,
                        end_index: para_end,
                        length: paragraph.len(),
                    });
                }
            }
            
            cumulative_pos = para_end + 2; // +2 for paragraph separator (usually \n\n)
        }

        let total_paragraphs = self.paragraphs.len();
        let percentage = if total_paragraphs > 0 {
            (complex_paragraphs.len() as f64 / total_paragraphs as f64) * 100.0
        } else {
            0.0
        };

        let complex_paragraph_count = complex_paragraphs.len();

        Ok(ComplexParagraphsReport {
            complex_paragraph_count,
            percentage: (percentage * 10.0).round() / 10.0,
            complex_paragraphs,
        })
    }

    // ========== FEATURE 15: CONJUNCTION STARTS ==========
    pub fn analyze_conjunction_starts(&self) -> Result<ConjunctionStartsReport> {
        let mut conjunction_starts = 0;

        for sentence in self.sentences {
            if let Some(caps) = FIRST_WORD.captures(&sentence.to_lowercase()) {
                if let Some(first_word) = caps.get(1) {
                    if CONJUNCTIONS.contains(first_word.as_str()) {
                        conjunction_starts += 1;
                    }
                }
            }
        }

        let percentage = if !self.sentences.is_empty() {
            (conjunction_starts as f64 / self.sentences.len() as f64) * 100.0
        } else {
            0.0
        };

        Ok(ConjunctionStartsReport {
            count: conjunction_starts,
            percentage: (percentage * 10.0).round() / 10.0,
        })
    }

    // ========== FEATURE 16-17: ADVERBS & HIDDEN VERBS (Style Report) ==========
    pub fn analyze_style(&self) -> Result<StyleReport> {
        // Count adverbs
        let adverb_count = ADVERBS.find_iter(&self.text.to_lowercase()).count();

        // Find hidden verbs
        let mut hidden_verbs = Vec::new();
        let text_lower = self.text.to_lowercase();
        for (noun, verb) in HIDDEN_VERBS.iter() {
            let count = text_lower.matches(noun).count();
            if count > 0 {
                hidden_verbs.push(format!(
                    "'{}' appears {} time(s) - consider using '{}'",
                    noun, count, verb
                ));
            }
        }

        Ok(StyleReport {
            passive_voice_count: 0, // Will be filled by passive voice detector
            adverb_count,
            hidden_verbs,
        })
    }

    // ========== FEATURE 18: STYLE SCORE ==========
    pub fn calculate_style_score(
        &self,
        style: &StyleReport,
        sticky: &StickySentencesReport,
        diction: &DictionReport,
    ) -> i32 {
        let mut score = 100;

        // Deduct for style issues
        score -= (style.passive_voice_count * 2).min(20) as i32;
        score -= ((style.adverb_count as f64 * 0.5) as usize).min(15) as i32;
        score -= (style.hidden_verbs.len() * 2).min(10) as i32;

        // Deduct for readability issues
        if sticky.overall_glue_index > 25.0 {
            score -= (sticky.overall_glue_index - 25.0).min(15.0) as i32;
        }

        // Deduct for vague language
        score -= ((diction.total_vague_words as f64 * 0.5) as usize).min(10) as i32;

        score.max(0)
    }
}
