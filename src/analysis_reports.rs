use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// All report structures for comprehensive analysis

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickySentence {
    pub sentence_num: usize,
    pub glue_percentage: f64,
    pub sentence: String,
    pub start_index: usize,
    pub end_index: usize,
    pub length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickySentencesReport {
    pub overall_glue_index: f64,
    pub glue_index: f64,  // Alias for overall_glue_index for backward compatibility
    pub sticky_sentence_count: usize,
    pub sticky_sentences: Vec<StickySentence>,
    pub semi_sticky_sentences: Vec<StickySentence>,  // Sentences with 35-45% glue words
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacingDistribution {
    pub fast: usize,    // <10 words
    pub medium: usize,  // 10-20 words
    pub slow: usize,    // >20 words
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacingReport {
    pub fast_paced_percentage: f64,
    pub medium_paced_percentage: f64,
    pub slow_paced_percentage: f64,
    pub pacing_distribution: PacingDistribution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceLengthReport {
    pub avg_length: f64,
    pub std_deviation: f64,
    pub variety_score: f64,
    pub shortest: usize,
    pub longest: usize,
    pub very_long_sentences: usize,
    pub very_long_details: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionFound {
    pub transition: String,
    pub sentence_num: usize,
    pub start_index: usize,
    pub end_index: usize,
    pub length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionReport {
    pub sentences_with_transitions: usize,
    pub transition_percentage: f64,
    pub total_transitions_used: usize,
    pub unique_transitions: usize,
    pub most_common_transitions: Vec<(String, usize)>,
    pub all_transitions: Vec<TransitionFound>, // Detailed positions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordOccurrence {
    pub start_index: usize,
    pub end_index: usize,
    pub length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverusedWord {
    pub word: String,
    pub count: usize,
    pub frequency: f64,
    pub occurrences: Vec<WordOccurrence>, // Positions of each occurrence
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverusedWordsReport {
    pub overused_words: Vec<OverusedWord>,
    pub total_unique_words: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeatedPhrase {
    pub phrase: String,
    pub count: usize,
    pub occurrences: Vec<WordOccurrence>, // Positions of each occurrence
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeatedPhrasesReport {
    pub total_repeated_phrases: usize,
    pub most_repeated: Vec<RepeatedPhrase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Echo {
    pub word: String,
    pub paragraph: usize,
    pub distance: usize,
    pub occurrences: usize,
    pub positions: Vec<WordOccurrence>, // Exact character positions of each occurrence
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EchoesReport {
    pub total_echoes: usize,
    pub echoes: Vec<Echo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenseData {
    pub count: usize,
    pub percentage: f64,
    pub unique_words: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensoryReport {
    pub sensory_word_count: usize,
    pub sensory_percentage: f64,
    pub by_sense: HashMap<String, SenseData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VagueWord {
    pub word: String,
    pub count: usize,
    pub occurrences: Vec<WordOccurrence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionReport {
    pub total_vague_words: usize,
    pub unique_vague_words: usize,
    pub most_common_vague: Vec<VagueWord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClicheFound {
    pub cliche: String,
    pub count: usize,
    pub occurrences: Vec<WordOccurrence>, // Positions of each occurrence
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClichesReport {
    pub total_cliches: usize,
    pub cliches: Vec<ClicheFound>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyReport {
    pub total_issues: usize,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcronymReport {
    pub total_acronyms: usize,
    pub unique_acronyms: usize,
    pub acronym_list: Vec<(String, usize)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConjunctionStartsReport {
    pub count: usize,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JargonFound {
    pub jargon: String,
    pub count: usize,
    pub occurrences: Vec<WordOccurrence>, // Positions of each occurrence
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessJargonReport {
    pub total_jargon: usize,
    pub unique_jargon_phrases: usize,
    pub jargon_list: Vec<JargonFound>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexParagraph {
    pub paragraph_num: usize,
    pub avg_sentence_length: f64,
    pub avg_syllables: f64,
    pub start_index: usize,
    pub end_index: usize,
    pub length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexParagraphsReport {
    pub complex_paragraph_count: usize,
    pub percentage: f64,
    pub complex_paragraphs: Vec<ComplexParagraph>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleReport {
    pub passive_voice_count: usize,
    pub adverb_count: usize,
    pub hidden_verbs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullAnalysisReport {
    pub word_count: usize,
    pub sentence_count: usize,
    pub paragraph_count: usize,
    pub style_score: i32,
    pub style: StyleReport,
    pub sticky_sentences: StickySentencesReport,
    pub pacing: PacingReport,
    pub sentence_length: SentenceLengthReport,
    pub transitions: TransitionReport,
    pub overused_words: OverusedWordsReport,
    pub repeated_phrases: RepeatedPhrasesReport,
    pub echoes: EchoesReport,
    pub sensory: SensoryReport,
    pub diction: DictionReport,
    pub cliches: ClichesReport,
    pub consistency: ConsistencyReport,
    pub acronyms: AcronymReport,
    pub conjunction_starts: ConjunctionStartsReport,
    pub business_jargon: BusinessJargonReport,
    pub complex_paragraphs: ComplexParagraphsReport,
}
