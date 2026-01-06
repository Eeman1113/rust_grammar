use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use Rust_Grammar::{Config, TextAnalyzer};
use Rust_Grammar::grammar::{PassiveVoiceMatch, GrammarIssue};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Build router
    let app = Router::new()
        .route("/analyze", post(analyze_text))
        .route("/score", post(get_scores_only))
        .route("/sentencelength", post(get_sentence_length))
        .route("/readability", post(get_readability))
        .route("/passivevoice", post(get_passive_voice))
        .route("/glueindex", post(get_glue_index))
        .layer(CorsLayer::permissive());

    // Bind to 0.0.0.0:2000
    // let addr = SocketAddr::from(([0, 0, 0, 0], 2000));
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("🚀 Text Analyzer API running on http://{}", addr);
    println!("📝 POST to http://{}/analyze with JSON body: {{\"text\": \"your text\"}}", addr);
    println!("📊 POST to http://{}/score for scores only", addr);
    println!("📏 POST to http://{}/sentencelength for sentence length analysis", addr);
    println!("📖 POST to http://{}/readability for readability analysis", addr);
    println!("🎯 POST to http://{}/passivevoice for passive voice analysis", addr);
    println!("🔗 POST to http://{}/glueindex for glue index analysis", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Request body structure
#[derive(Debug, Deserialize)]
struct AnalyzeRequest {
    text: String,
}

// Sentence length endpoint request
#[derive(Debug, Deserialize)]
struct SentenceLengthRequest {
    data: Vec<ParagraphData>,
}

#[derive(Debug, Deserialize)]
struct ParagraphData {
    text: String,
    key: String,
}

// Sentence length endpoint response
#[derive(Debug, Serialize)]
struct SentenceLengthResponse {
    score: i32,
    percentage: f64,
    message: String,
    #[serde(rename = "numWords")]
    num_words: usize,
    #[serde(rename = "numCharacters")]
    num_characters: usize,
    #[serde(rename = "avgSentenceLength")]
    avg_sentence_length: f64,
    #[serde(rename = "longestSentenceLength")]
    longest_sentence_length: usize,
    #[serde(rename = "targetRange")]
    target_range: String,
    #[serde(rename = "sentenceVariety")]
    sentence_variety: f64,
    #[serde(rename = "varietyTarget")]
    variety_target: String,
    #[serde(rename = "sentencesByWordCount")]
    sentences_by_word_count: SentencesByWordCount,
    #[serde(rename = "individualSentenceLengths")]
    individual_sentence_lengths: Vec<IndividualSentence>,
}

#[derive(Debug, Serialize)]
struct SentencesByWordCount {
    under10: usize,
    range10to19: usize,
    range20to29: usize,
    range30to39: usize,
    over40: usize,
}

#[derive(Debug, Serialize)]
struct IndividualSentence {
    start: usize,
    end: usize,
    length: usize,
    string: String,
    #[serde(rename = "wordCount")]
    word_count: usize,
    #[serde(rename = "paragraphKey")]
    paragraph_key: String,
    kind: String,
}


// Issue structure (original format for issues)
#[derive(Debug, Serialize)]
struct AnalysisIssue {
    #[serde(rename = "Id")]
    id: String,
    start: usize,
    length: usize,
    end: usize,
    #[serde(rename = "paragraphKey")]
    paragraph_key: String,
    string: String,
    #[serde(rename = "type")]
    issue_type: String,
    suggestions: Suggestions,
}

#[derive(Debug, Serialize)]
struct Suggestions {
    recommendation: Vec<String>,
}

// Comprehensive scores (matching the user's format)
#[derive(Debug, Serialize)]
struct ComprehensiveScores {
    #[serde(rename = "styleScore")]
    style_score: ScoreDetail,
    
    #[serde(rename = "styleGuideCompliance")]
    style_guide_compliance: ScoreDetail,
    
    #[serde(rename = "sentenceLength")]
    sentence_length: SentenceLengthDetail,
    
    #[serde(rename = "readabilityGrade")]
    readability_grade: ScoreDetail,
    
    #[serde(rename = "sentenceVariety")]
    sentence_variety: ScoreDetail,
    
    #[serde(rename = "glueIndex")]
    glue_index: PercentageScore,
    
    #[serde(rename = "passiveVoice")]
    passive_voice: CountScore,
    
    #[serde(rename = "businessJargon")]
    business_jargon: CountScore,
    
    #[serde(rename = "complexParagraphs")]
    complex_paragraphs: PercentageScore,
    
    #[serde(rename = "conjunctionStarts")]
    conjunction_starts: PercentageScore,
    
    #[serde(rename = "slowPacing")]
    slow_pacing: PercentageScore,
    
    #[serde(rename = "veryLongSentences")]
    very_long_sentences: PercentageScore,
    
    #[serde(rename = "emotionTells")]
    emotion_tells: CountScore,
    
    #[serde(rename = "ingStarts")]
    ing_starts: PercentageScore,
    
    #[serde(rename = "dialogueTags")]
    dialogue_tags: PercentageScore,
    
    #[serde(rename = "unusualDialogueTags")]
    unusual_dialogue_tags: PercentageScore,
    
    #[serde(rename = "dialogueTagsWithAdverbs")]
    dialogue_tags_with_adverbs: PercentageScore,
    
    #[serde(rename = "weakAdverbs")]
    weak_adverbs: CountScore,
}

#[derive(Debug, Serialize)]
struct ScoreDetail {
    score: i32,
    percentage: f64,
    message: Option<String>,
}

#[derive(Debug, Serialize)]
struct SentenceLengthDetail {
    score: i32,
    percentage: f64,
    message: Option<String>,
    #[serde(rename = "numWords")]
    num_words: usize,
    #[serde(rename = "numCharacters")]
    num_characters: usize,
    #[serde(rename = "avgSentenceLength")]
    avg_sentence_length: f64,
    #[serde(rename = "targetRange")]
    target_range: String,
    #[serde(rename = "sentenceVariety")]
    sentence_variety: f64,
    #[serde(rename = "varietyTarget")]
    variety_target: String,
    #[serde(rename = "sentencesByWordCount")]
    sentences_by_word_count: SentenceDistribution,
    #[serde(rename = "individualSentenceLengths")]
    individual_sentence_lengths: Vec<SentenceOccurrence>,
}

#[derive(Debug, Serialize)]
struct SentenceDistribution {
    #[serde(rename = "under10")]
    under_10: usize,
    #[serde(rename = "range10to19")]
    range_10_to_19: usize,
    #[serde(rename = "range20to29")]
    range_20_to_29: usize,
    #[serde(rename = "range30to39")]
    range_30_to_39: usize,
    #[serde(rename = "over40")]
    over_40: usize,
}

#[derive(Debug, Serialize)]
struct Occurrence {
    start: usize,
    end: usize,
    length: usize,
    string: String,
    #[serde(rename = "paragraphKey")]
    paragraph_key: String,
}

#[derive(Debug, Serialize)]
struct SentenceOccurrence {
    start: usize,
    end: usize,
    length: usize,
    string: String,
    #[serde(rename = "wordCount")]
    word_count: usize,
    #[serde(rename = "paragraphKey")]
    paragraph_key: String,
}

#[derive(Debug, Serialize)]
struct PercentageScore {
    percentage: f64,
    count: usize,
    total: usize,
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    occurrences: Option<Vec<Occurrence>>,
}

#[derive(Debug, Serialize)]
struct CountScore {
    count: usize,
    percentage: Option<f64>,
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    occurrences: Option<Vec<Occurrence>>,
}

// Score-only response structures (for /score endpoint)
#[derive(Debug, Serialize)]
struct ScoreOnlyResponse {
    scores: UserFriendlyScores,
    #[serde(rename = "word_count")]
    word_count: usize,
    #[serde(rename = "sentence_count")]
    sentence_count: usize,
    #[serde(rename = "complex_words_count")]
    complex_words_count: usize,
}

#[derive(Debug, Serialize)]
struct UserFriendlyScores {
    #[serde(rename = "styleScore")]
    style_score: SimpleScore,
    
    #[serde(rename = "sentenceLength")]
    sentence_length: SimpleScore,
    
    #[serde(rename = "readability")]
    readability: SimpleScore,
    
    #[serde(rename = "sentenceVariety")]
    sentence_variety: SimpleScore,
    
    #[serde(rename = "glueIndex")]
    glue_index: SimpleScore,
    
    #[serde(rename = "passiveVoice")]
    passive_voice: SimpleScore,
    
    #[serde(rename = "businessJargon")]
    business_jargon: SimpleScore,
    
    #[serde(rename = "complexParagraphs")]
    complex_paragraphs: SimpleScore,
    
    #[serde(rename = "conjunctionStarts")]
    conjunction_starts: SimpleScore,
    
    #[serde(rename = "slowPacing")]
    slow_pacing: SimpleScore,
    
    #[serde(rename = "veryLongSentences")]
    very_long_sentences: SimpleScore,
    
    #[serde(rename = "emotionTells")]
    emotion_tells: SimpleScore,
    
    #[serde(rename = "ingStarts")]
    ing_starts: SimpleScore,
    
    #[serde(rename = "weakAdverbs")]
    weak_adverbs: SimpleScore,
    
    #[serde(rename = "dialogueTags")]
    dialogue_tags: SimpleScore,
}

#[derive(Debug, Serialize)]
struct SimpleScore {
    current: f64,
    ideal: String,
    status: String,  // "good", "fair", "needs improvement"
    message: String,
}

// Enhanced API response
#[derive(Debug, Serialize)]
struct AnalyzeResponse {
    scores: ComprehensiveScores,
    issues: Vec<AnalysisIssue>,
    summary: AnalysisSummary,
}

#[derive(Debug, Serialize)]
struct AnalysisSummary {
    total_issues: usize,
    word_count: usize,
    sentence_count: usize,
    paragraph_count: usize,
    character_count: usize,
}

// API handler
async fn analyze_text(
    Json(payload): Json<AnalyzeRequest>,
) -> Result<Json<AnalyzeResponse>, ApiError> {
    if payload.text.is_empty() {
        return Err(ApiError::EmptyText);
    }

    // Create analyzer with default config
    let config = Config::default();
    let analyzer = TextAnalyzer::new(payload.text.clone(), config)
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    // Run full analysis
    let full_report = analyzer
        .generate_full_report()
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    let passive_voice = analyzer
        .detect_passive_voice()
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    let grammar = analyzer
        .check_grammar()
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    let stats = analyzer.statistics();
    let readability = analyzer.readability_metrics()
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    // Calculate comprehensive scores
    let scores = calculate_comprehensive_scores(
        &full_report,
        &passive_voice,
        &grammar,
        &stats,
        &readability,
        &payload.text,
    );

    // Convert all issues to the requested format
    let issues = convert_to_issues(&full_report, &passive_voice, &grammar, &payload.text);

    let response = AnalyzeResponse {
        scores,
        issues,
        summary: AnalysisSummary {
            total_issues: passive_voice.len() + grammar.len() + 
                         full_report.sticky_sentences.sticky_sentences.len() +
                         full_report.overused_words.overused_words.len() +
                         full_report.cliches.cliches.len(),
            word_count: stats.word_count,
            sentence_count: stats.sentence_count,
            paragraph_count: stats.paragraph_count,
            character_count: stats.character_count,
        },
    };

    Ok(Json(response))
}

// Handler for /score endpoint - returns only scores with ideal values and messages
async fn get_scores_only(Json(payload): Json<AnalyzeRequest>) -> Result<Json<ScoreOnlyResponse>, ApiError> {
    // Validate input
    if payload.text.trim().is_empty() {
        return Err(ApiError::EmptyText);
    }

    // Create analyzer with default config
    let config = Config::default();
    let analyzer = TextAnalyzer::new(payload.text.clone(), config)
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    // Run full analysis
    let full_report = analyzer
        .generate_full_report()
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    let passive_voice = analyzer
        .detect_passive_voice()
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    let grammar = analyzer
        .check_grammar()
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    let stats = analyzer.statistics();
    let readability = analyzer.readability_metrics()
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    // Calculate comprehensive scores with user-friendly messages
    let scores = create_user_friendly_scores(
        &full_report,
        &passive_voice,
        &grammar,
        &stats,
        &readability,
        &payload.text,
    );

    let response = ScoreOnlyResponse {
        scores,
        word_count: stats.word_count,
        sentence_count: stats.sentence_count,
        complex_words_count: count_complex_words(&payload.text),
    };

    Ok(Json(response))
}

// Handler for /sentencelength endpoint
async fn get_sentence_length(Json(payload): Json<SentenceLengthRequest>) -> Result<Json<SentenceLengthResponse>, ApiError> {
    // Validate input
    if payload.data.is_empty() {
        return Err(ApiError::EmptyText);
    }

    // Combine all paragraph texts for overall analysis
    let combined_text: String = payload.data.iter()
        .map(|p| p.text.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    if combined_text.trim().is_empty() {
        return Err(ApiError::EmptyText);
    }

    // Count overall stats
    let num_words = combined_text.split_whitespace().count();
    
    // Use CHARACTER count, not byte length
    let character_array = combined_text.chars();
    let num_characters = character_array.count();

    // Split into sentences for overall analysis
    let all_sentences = split_into_sentences(&combined_text);
    let sentence_count = all_sentences.len().max(1);

    // Calculate average sentence length
    let avg_sentence_length = (num_words as f64 / sentence_count as f64 * 100.0).round() / 100.0;

    // Calculate sentence variety (standard deviation)
    let sentence_word_counts: Vec<usize> = all_sentences.iter()
        .map(|s| s.split_whitespace().count())
        .collect();
    
    let mean = sentence_word_counts.iter().sum::<usize>() as f64 / sentence_word_counts.len() as f64;
    let variance = sentence_word_counts.iter()
        .map(|&count| {
            let diff = count as f64 - mean;
            diff * diff
        })
        .sum::<f64>() / sentence_word_counts.len() as f64;
    let sentence_variety = (variance.sqrt() * 100.0).round() / 100.0;
    
    // Find the longest sentence (by word count)
    let longest_sentence_length = sentence_word_counts.iter().max().copied().unwrap_or(0);

    // Count sentences by word count
    let mut under10 = 0;
    let mut range10to19 = 0;
    let mut range20to29 = 0;
    let mut range30to39 = 0;
    let mut over40 = 0;

    for &word_count in &sentence_word_counts {
        match word_count {
            0..=9 => under10 += 1,
            10..=19 => range10to19 += 1,
            20..=29 => range20to29 += 1,
            30..=39 => range30to39 += 1,
            _ => over40 += 1,
        }
    }

    // Calculate score (0-100 based on avg sentence length)
    let score = if avg_sentence_length >= 15.0 && avg_sentence_length <= 20.0 {
        100
    } else if avg_sentence_length >= 12.0 && avg_sentence_length <= 23.0 {
        70
    } else if avg_sentence_length >= 10.0 && avg_sentence_length <= 25.0 {
        50
    } else {
        10
    };

    let percentage = score as f64;

    // Get message based on average length
    let message = if avg_sentence_length < 11.0 {
        "Use a few longer sentences to add more depth to your writing.".to_string()
    } else if avg_sentence_length > 18.0 {
        "Consider breaking up some longer sentences for better readability.".to_string()
    } else {
        "Great! Your sentence length is in the ideal range.".to_string()
    };

    // Process individual sentences for each paragraph
    let mut individual_sentence_lengths = Vec::new();

    for paragraph in &payload.data {
        // Split sentences and track positions manually
        let text = &paragraph.text;
        let sentences = split_sentences_with_positions(text);

        for (sentence_text, start, end) in sentences {
            let word_count = sentence_text.split_whitespace().count();
            
            // Use character-based length calculation
            let char_length = sentence_text.chars().count();
            
            // Determine sentence kind based on word count
            let kind = get_sentence_kind(word_count);

            individual_sentence_lengths.push(IndividualSentence {
                start,
                end,
                length: char_length,
                string: sentence_text.clone(),
                word_count,
                paragraph_key: paragraph.key.clone(),
                kind,
            });
        }
    }

    let response = SentenceLengthResponse {
        score,
        percentage,
        message,
        num_words,
        num_characters,
        avg_sentence_length,
        longest_sentence_length,
        target_range: "11 to 18".to_string(),
        sentence_variety,
        variety_target: "over 3".to_string(),
        sentences_by_word_count: SentencesByWordCount {
            under10,
            range10to19,
            range20to29,
            range30to39,
            over40,
        },
        individual_sentence_lengths,
    };

    Ok(Json(response))
}

// ============================================================================
// NEW ENDPOINTS: /readability, /passivevoice, /glueindex
// ============================================================================

// Response structures for new endpoints
#[derive(Debug, Serialize)]
struct ReadabilityResponse {
    #[serde(rename = "estimatedReadingTime")]
    estimated_reading_time: String,
    message: String,
    #[serde(rename = "fleschReadingEase")]
    flesch_reading_ease: f64,
    #[serde(rename = "fleschKincaidGrade")]
    flesch_kincaid_grade: f64,
    #[serde(rename = "colemanLiau")]
    coleman_liau: f64,
    #[serde(rename = "automatedReadabilityIndex")]
    automated_readability_index: f64,
    #[serde(rename = "difficultParagraphs")]
    difficult_paragraphs: Vec<DifficultParagraph>,
}

#[derive(Debug, Serialize)]
struct DifficultParagraph {
    difficulty: String,  // "very hard", "slightly difficult", etc.
    start: usize,  // Relative to paragraph
    end: usize,    // Relative to paragraph
    string: String,
    excerpt: String,  // First ~50 chars for display
    #[serde(rename = "paragraphKey")]
    paragraph_key: String,
}

#[derive(Debug, Serialize)]
struct PassiveVoiceResponse {
    #[serde(rename = "passiveVerbsFound")]
    passive_verbs_found: usize,
    #[serde(rename = "passiveVerbsMessage")]
    passive_verbs_message: String,
    #[serde(rename = "passiveVerbs")]
    passive_verbs: Vec<PassiveVerbOccurrence>,
    #[serde(rename = "hiddenVerbsFound")]
    hidden_verbs_found: usize,
    #[serde(rename = "hiddenVerbsMessage")]
    hidden_verbs_message: String,
    #[serde(rename = "hiddenVerbs")]
    hidden_verbs: Vec<HiddenVerbOccurrence>,
    #[serde(rename = "adverbsInDialogue")]
    adverbs_in_dialogue: usize,
    #[serde(rename = "adverbsOutsideDialogue")]
    adverbs_outside_dialogue: usize,
    #[serde(rename = "adverbsMessage")]
    adverbs_message: String,
    #[serde(rename = "adverbsList")]
    adverbs_list: Vec<AdverbOccurrence>,
    #[serde(rename = "readabilityEnhancementsFound")]
    readability_enhancements_found: usize,
    #[serde(rename = "readabilityEnhancementsMessage")]
    readability_enhancements_message: String,
    #[serde(rename = "readabilityEnhancements")]
    readability_enhancements: Vec<EnhancementOccurrence>,
    #[serde(rename = "inclusiveLanguageMessage")]
    inclusive_language_message: String,
    #[serde(rename = "inclusiveLanguageImprovements")]
    inclusive_language_improvements: Vec<EnhancementOccurrence>,
    #[serde(rename = "emotionTellsMessage")]
    emotion_tells_message: String,
    #[serde(rename = "emotionTells")]
    emotion_tells: Vec<EnhancementOccurrence>,
    #[serde(rename = "styleImprovementsMessage")]
    style_improvements_message: String,
    #[serde(rename = "styleImprovements")]
    style_improvements: Vec<EnhancementOccurrence>,
    #[serde(rename = "businessJargonMessage")]
    business_jargon_message: String,
    #[serde(rename = "businessJargon")]
    business_jargon: Vec<EnhancementOccurrence>,
    #[serde(rename = "longSubordinateClausesMessage")]
    long_subordinate_clauses_message: String,
    #[serde(rename = "longSubordinateClauses")]
    long_subordinate_clauses: Vec<EnhancementOccurrence>,
    #[serde(rename = "passiveIndex")]
    passive_index: f64,
    #[serde(rename = "passiveIndexMessage")]
    passive_index_message: String,
    #[serde(rename = "passiveIndexTarget")]
    passive_index_target: String,
    #[serde(rename = "repeatedSentenceStartsMessage")]
    repeated_sentence_starts_message: String,
    #[serde(rename = "repeatedSentenceStarts")]
    repeated_sentence_starts: Vec<RepeatedStart>,
    #[serde(rename = "styleGuideItemsMessage")]
    style_guide_items_message: String,
    #[serde(rename = "styleGuideItems")]
    style_guide_items: Vec<EnhancementOccurrence>,
}

#[derive(Debug, Serialize)]
struct PassiveVerbOccurrence {
    verb: String,
    count: usize,
    occurrences: Vec<OccurrenceDetail>,
}

#[derive(Debug, Serialize)]
struct HiddenVerbOccurrence {
    phrase: String,
    count: usize,
    occurrences: Vec<OccurrenceDetail>,
}

#[derive(Debug, Serialize)]
struct AdverbOccurrence {
    adverb: String,
    count: usize,
    occurrences: Vec<OccurrenceDetail>,
}

#[derive(Debug, Serialize)]
struct EnhancementOccurrence {
    phrase: String,
    count: usize,
    occurrences: Vec<OccurrenceDetail>,
}

#[derive(Debug, Serialize)]
struct RepeatedStart {
    start_word: String,
    count: usize,
    sentences: Vec<String>,
}

#[derive(Debug, Serialize)]
struct OccurrenceDetail {
    start: usize,  // Relative to paragraph
    end: usize,    // Relative to paragraph
    string: String,
    #[serde(rename = "paragraphKey")]
    paragraph_key: String,
    report: String,  // Which report this belongs to
}

#[derive(Debug, Serialize)]
struct GlueIndexResponse {
    #[serde(rename = "glueIndex")]
    glue_index: f64,
    #[serde(rename = "glueIndexTarget")]
    glue_index_target: String,
    #[serde(rename = "sentences")]
    sentences: Vec<StickySentence>,
}

#[derive(Debug, Serialize)]
struct StickySentence {
    start: usize,  // Relative to paragraph
    end: usize,    // Relative to paragraph
    string: String,
    excerpt: String,
    #[serde(rename = "gluePercentage")]
    glue_percentage: f64,
    category: String,  // "sticky" or "semi-sticky"
    #[serde(rename = "paragraphKey")]
    paragraph_key: String,
}

// Handler for /readability endpoint
async fn get_readability(
    Json(payload): Json<SentenceLengthRequest>,
) -> Result<Json<ReadabilityResponse>, ApiError> {
    // Work with whatever is given - no validation
    if payload.data.is_empty() {
        // Return default response instead of error
        return Ok(Json(ReadabilityResponse {
            estimated_reading_time: "0 min, 0 sec".to_string(),
            message: "No content to analyze.".to_string(),
            flesch_reading_ease: 0.0,
            flesch_kincaid_grade: 0.0,
            coleman_liau: 0.0,
            automated_readability_index: 0.0,
            difficult_paragraphs: Vec::new(),
        }));
    }

    // Combine all texts for overall metrics
    let combined_text: String = payload.data.iter()
        .map(|p| p.text.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    // Work with empty text too - just return default results
    if combined_text.trim().is_empty() {
        return Ok(Json(ReadabilityResponse {
            estimated_reading_time: "0 min, 0 sec".to_string(),
            message: "No content to analyze.".to_string(),
            flesch_reading_ease: 0.0,
            flesch_kincaid_grade: 0.0,
            coleman_liau: 0.0,
            automated_readability_index: 0.0,
            difficult_paragraphs: Vec::new(),
        }));
    }

    let config = Config::default();
    let analyzer = TextAnalyzer::new(combined_text.clone(), config)
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    let stats = analyzer.statistics();
    let readability = analyzer.readability_metrics()
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    // Calculate estimated reading time (assuming 200 words per minute)
    let minutes = stats.word_count / 200;
    let seconds = (stats.word_count % 200) * 60 / 200;
    let estimated_reading_time = format!("{} min, {} sec", minutes, seconds);

    // Analyze each paragraph for difficulty
    let mut difficult_paragraphs = Vec::new();

    for paragraph in &payload.data {
        let text = &paragraph.text;
        
        if text.trim().is_empty() {
            continue;
        }

        // Simple heuristic: paragraphs with average word length > 6 and sentence length > 25
        let words: Vec<&str> = text.split_whitespace().collect();
        let avg_word_len = if !words.is_empty() {
            words.iter().map(|w| w.len()).sum::<usize>() as f64 / words.len() as f64
        } else {
            0.0
        };

        let sentences = text.split(&['.', '!', '?'][..]).filter(|s| !s.trim().is_empty()).count().max(1);
        let avg_sentence_len = words.len() as f64 / sentences as f64;

        let difficulty = if avg_word_len > 6.5 && avg_sentence_len > 30.0 {
            Some("very hard")
        } else if avg_word_len > 5.5 && avg_sentence_len > 25.0 {
            Some("hard")
        } else if avg_sentence_len > 20.0 {
            Some("slightly difficult")
        } else {
            None
        };

        if let Some(diff) = difficulty {
            // Use character-based length
            let character_array = text.chars();
            let text_char_len = character_array.count();
            
            let excerpt = if text_char_len > 50 {
                text.chars().take(50).collect::<String>() + "..."
            } else {
                text.to_string()
            };

            difficult_paragraphs.push(DifficultParagraph {
                difficulty: diff.to_string(),
                start: 0,
                end: text_char_len,
                string: text.to_string(),
                excerpt,
                paragraph_key: paragraph.key.clone(),
            });
        }
    }

    // Round scores to 2 decimal places
    let flesch_ease = (readability.flesch_reading_ease * 100.0).round() / 100.0;
    let flesch_grade = (readability.flesch_kincaid_grade * 100.0).round() / 100.0;
    let coleman = (readability.smog_index.unwrap_or(0.0) * 100.0).round() / 100.0;
    let ari = (readability.avg_words_per_sentence * 100.0).round() / 100.0;

    // Generate intelligent message based on Flesch Reading Ease score
    let message = if flesch_ease >= 90.0 {
        "Excellent! Your document is very easy to read - perfect for a wide audience.".to_string()
    } else if flesch_ease >= 80.0 {
        "Great! Your document is easy to read and accessible to most readers.".to_string()
    } else if flesch_ease >= 70.0 {
        "Good! Your document is fairly easy to read - suitable for general audiences.".to_string()
    } else if flesch_ease >= 60.0 {
        "Your document has standard readability - appropriate for most audiences.".to_string()
    } else if flesch_ease >= 50.0 {
        "Your document is fairly difficult to read. Consider simplifying for broader audiences.".to_string()
    } else if flesch_ease >= 30.0 {
        "Your document is difficult to read. Consider breaking up complex sentences and using simpler words.".to_string()
    } else {
        "Your document is very difficult to read. Significant simplification recommended for better accessibility.".to_string()
    };

    let response = ReadabilityResponse {
        estimated_reading_time,
        message,
        flesch_reading_ease: flesch_ease,
        flesch_kincaid_grade: flesch_grade,
        coleman_liau: coleman,
        automated_readability_index: ari,
        difficult_paragraphs,
    };

    Ok(Json(response))
}

// Handler for /passivevoice endpoint
async fn get_passive_voice(
    Json(payload): Json<SentenceLengthRequest>,
) -> Result<Json<PassiveVoiceResponse>, ApiError> {
    // Work with whatever is given - no validation
    if payload.data.is_empty() {
        // Return empty response instead of error
        return Ok(Json(PassiveVoiceResponse {
            passive_verbs_found: 0,
            passive_verbs_message: "No content to analyze.".to_string(),
            passive_verbs: Vec::new(),
            hidden_verbs_found: 0,
            hidden_verbs_message: "No content to analyze.".to_string(),
            hidden_verbs: Vec::new(),
            adverbs_in_dialogue: 0,
            adverbs_outside_dialogue: 0,
            adverbs_message: "No content to analyze.".to_string(),
            adverbs_list: Vec::new(),
            readability_enhancements_found: 0,
            readability_enhancements_message: "No content to analyze.".to_string(),
            readability_enhancements: Vec::new(),
            inclusive_language_message: "No content to analyze.".to_string(),
            inclusive_language_improvements: Vec::new(),
            emotion_tells_message: "No content to analyze.".to_string(),
            emotion_tells: Vec::new(),
            style_improvements_message: "No content to analyze.".to_string(),
            style_improvements: Vec::new(),
            business_jargon_message: "No content to analyze.".to_string(),
            business_jargon: Vec::new(),
            long_subordinate_clauses_message: "No content to analyze.".to_string(),
            long_subordinate_clauses: Vec::new(),
            passive_index: 0.0,
            passive_index_message: "No content to analyze.".to_string(),
            passive_index_target: "up to 25".to_string(),
            repeated_sentence_starts_message: "No content to analyze.".to_string(),
            repeated_sentence_starts: Vec::new(),
            style_guide_items_message: "No content to analyze.".to_string(),
            style_guide_items: Vec::new(),
        }));
    }

    // Combine all texts for overall analysis
    let combined_text: String = payload.data.iter()
        .map(|p| p.text.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    // Work with empty text too - just return empty results
    if combined_text.trim().is_empty() {
        return Ok(Json(PassiveVoiceResponse {
            passive_verbs_found: 0,
            passive_verbs_message: "No content to analyze.".to_string(),
            passive_verbs: Vec::new(),
            hidden_verbs_found: 0,
            hidden_verbs_message: "No content to analyze.".to_string(),
            hidden_verbs: Vec::new(),
            adverbs_in_dialogue: 0,
            adverbs_outside_dialogue: 0,
            adverbs_message: "No content to analyze.".to_string(),
            adverbs_list: Vec::new(),
            readability_enhancements_found: 0,
            readability_enhancements_message: "No content to analyze.".to_string(),
            readability_enhancements: Vec::new(),
            inclusive_language_message: "No content to analyze.".to_string(),
            inclusive_language_improvements: Vec::new(),
            emotion_tells_message: "No content to analyze.".to_string(),
            emotion_tells: Vec::new(),
            style_improvements_message: "No content to analyze.".to_string(),
            style_improvements: Vec::new(),
            business_jargon_message: "No content to analyze.".to_string(),
            business_jargon: Vec::new(),
            long_subordinate_clauses_message: "No content to analyze.".to_string(),
            long_subordinate_clauses: Vec::new(),
            passive_index: 0.0,
            passive_index_message: "No content to analyze.".to_string(),
            passive_index_target: "up to 25".to_string(),
            repeated_sentence_starts_message: "No content to analyze.".to_string(),
            repeated_sentence_starts: Vec::new(),
            style_guide_items_message: "No content to analyze.".to_string(),
            style_guide_items: Vec::new(),
        }));
    }

    let config = Config::default();
    let analyzer = TextAnalyzer::new(combined_text.clone(), config)
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    let stats = analyzer.statistics();
    let sentence_count = stats.sentence_count.max(1);

    // Process each paragraph to find passive voice with paragraph-relative positions
    let mut all_passive_verbs: Vec<(String, String, usize, usize, String)> = Vec::new(); // (verb, key, start, end, string)

    for paragraph in &payload.data {
        let text = &paragraph.text;
        
        if text.trim().is_empty() {
            continue;
        }

        // Analyze this paragraph
        let para_config = Config::default();
        let para_analyzer = TextAnalyzer::new(text.clone(), para_config)
            .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

        let passive_matches = para_analyzer.detect_passive_voice()
            .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

        // Convert to character-based positions relative to paragraph
        for pv in passive_matches {
            // Find nearest valid character boundaries
            let safe_start = find_char_boundary(text, pv.start_index);
            let safe_end = find_char_boundary(text, pv.end_index);
            
            // Convert byte positions to character positions
            let char_start = text[..safe_start].chars().count();
            let char_end = text[..safe_end].chars().count();
            let text_chars: Vec<char> = text.chars().collect();
            
            // Safely extract substring
            let string: String = if char_end <= text_chars.len() {
                text_chars[char_start..char_end].iter().collect()
            } else {
                text_chars[char_start..].iter().collect()
            };

            all_passive_verbs.push((
                pv.text.clone(),
                paragraph.key.clone(),
                char_start,
                char_end,
                string,
            ));
        }
    }

    // Group by verb phrase
    let mut passive_verb_map: std::collections::HashMap<String, Vec<(String, usize, usize, String)>> = 
        std::collections::HashMap::new();
    
    for (verb, key, start, end, string) in all_passive_verbs {
        passive_verb_map.entry(verb).or_insert_with(Vec::new).push((key, start, end, string));
    }

    let mut passive_verbs = Vec::new();
    let total_passive_count = passive_verb_map.values().map(|v| v.len()).sum::<usize>();

    for (verb, occurrences_list) in passive_verb_map {
        let occurrences: Vec<OccurrenceDetail> = occurrences_list.into_iter().map(|(key, start, end, string)| {
            OccurrenceDetail {
                start,
                end,
                string,
                paragraph_key: key,
                report: "passiveVerbs".to_string(),
            }
        }).collect();

        passive_verbs.push(PassiveVerbOccurrence {
            verb,
            count: occurrences.len(),
            occurrences,
        });
    }

    // Calculate passive index
    let passive_index = (total_passive_count as f64 * 100.0 / sentence_count as f64 * 10.0).round() / 10.0;

    // Get full report for additional analysis
    let full_report = analyzer.generate_full_report()
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    // ========== 1. HIDDEN VERBS (Nominalizations) ==========
    let hidden_verb_patterns = vec![
        ("make a decision", "decide"), ("take action", "act"),
        ("give consideration", "consider"), ("make an assumption", "assume"),
        ("have a discussion", "discuss"), ("make a payment", "pay"),
        ("The illusion of", "illusion"), ("The idea of", "idea"),
        ("The concept of", "concept"), ("The notion of", "notion"),
        ("implementation of", "implement"), ("creation of", "create"),
        ("utilization of", "utilize"), ("examination of", "examine"),
    ];
    
    let mut hidden_verbs_map: std::collections::HashMap<String, Vec<(String, usize, usize, String)>> = 
        std::collections::HashMap::new();
    
    for paragraph in &payload.data {
        let text = &paragraph.text;
        let text_lower = text.to_lowercase();
        
        for (pattern, _replacement) in &hidden_verb_patterns {
            let pattern_lower = pattern.to_lowercase();
            let mut start_pos = 0;
            
            while let Some(pos) = text_lower[start_pos..].find(&pattern_lower) {
                let actual_pos = start_pos + pos;
                let end_pos = actual_pos + pattern.chars().count();  // ✅ FIXED: Use .chars().count()
                
                let safe_start = find_char_boundary(text, actual_pos);
                let safe_end = find_char_boundary(text, end_pos);
                
                let char_start = text[..safe_start].chars().count();
                let char_end = text[..safe_end].chars().count();
                let text_chars: Vec<char> = text.chars().collect();
                
                let string: String = if char_end <= text_chars.len() {
                    text_chars[char_start..char_end].iter().collect()
                } else {
                    text_chars[char_start..].iter().collect()
                };
                
                hidden_verbs_map.entry(pattern.to_string())
                    .or_insert_with(Vec::new)
                    .push((paragraph.key.clone(), char_start, char_end, string));
                
                start_pos = end_pos;
            }
        }
    }
    
    let hidden_verbs: Vec<HiddenVerbOccurrence> = hidden_verbs_map.into_iter().map(|(phrase, occurrences_list)| {
        let occurrences: Vec<OccurrenceDetail> = occurrences_list.into_iter().map(|(key, start, end, string)| {
            OccurrenceDetail { start, end, string, paragraph_key: key, report: "hiddenVerbs".to_string() }
        }).collect();
        
        HiddenVerbOccurrence {
            phrase,
            count: occurrences.len(),
            occurrences,
        }
    }).collect();
    
    let hidden_verbs_found = hidden_verbs.iter().map(|h| h.count).sum();

    // ========== 2. ADVERBS ==========
    let adverb_patterns = vec![
        "meticulously", "profoundly", "slightly", "potentially", "ultimately",
        "repeatedly", "fully", "strangely", "subtly", "primarily", "increasingly",
        "quickly", "slowly", "carefully", "barely", "certainly", "definitely",
        "easily", "hardly", "really", "simply", "basically", "literally",
        "actually", "suddenly", "immediately", "completely", "absolutely",
        "extremely", "very", "quite", "rather", "fairly", "pretty",
    ];
    
    let mut adverbs_map: std::collections::HashMap<String, Vec<(String, usize, usize, String)>> = 
        std::collections::HashMap::new();
    
    for paragraph in &payload.data {
        let text = &paragraph.text;
        let text_lower = text.to_lowercase();
        
        for pattern in &adverb_patterns {
            let pattern_lower = pattern.to_lowercase();
            let mut start_pos = 0;
            
            while let Some(pos) = text_lower[start_pos..].find(&pattern_lower) {
                let actual_pos = start_pos + pos;
                let end_pos = actual_pos + pattern.chars().count();  // ✅ FIXED: Use .chars().count()
                
                // Check if it's a whole word (not part of another word)
                let is_word_start = actual_pos == 0 || 
                    text_lower.chars().nth(actual_pos - 1).map_or(false, |c| !c.is_alphabetic());
                let is_word_end = end_pos >= text_lower.chars().count() ||  // ✅ FIXED: Use .chars().count()
                    text_lower.chars().nth(end_pos).map_or(false, |c| !c.is_alphabetic());
                
                if is_word_start && is_word_end {
                    let safe_start = find_char_boundary(text, actual_pos);
                    let safe_end = find_char_boundary(text, end_pos);
                    
                    // Convert to character positions
                    let char_start = text[..safe_start].chars().count();
                    let char_end = text[..safe_end].chars().count();
                    let text_chars: Vec<char> = text.chars().collect();
                    
                    // Extract the actual string
                    let string: String = if char_end <= text_chars.len() {
                        text_chars[char_start..char_end].iter().collect()
                    } else {
                        text_chars[char_start..].iter().collect()
                    };
                    
                    adverbs_map.entry(pattern_lower.clone())
                        .or_insert_with(Vec::new)
                        .push((paragraph.key.clone(), char_start, char_end, string));
                }
                
                start_pos = actual_pos + 1;
            }
        }
    }
    
    let adverbs_list: Vec<AdverbOccurrence> = adverbs_map.into_iter().map(|(adverb, occurrences_list)| {
        let occurrences: Vec<OccurrenceDetail> = occurrences_list.into_iter().map(|(key, start, end, string)| {
            OccurrenceDetail { start, end, string, paragraph_key: key, report: "adverbs".to_string() }
        }).collect();
        
        AdverbOccurrence {
            adverb,
            count: occurrences.len(),
            occurrences,
        }
    }).collect();
    
    let adverbs_outside_dialogue = adverbs_list.iter().map(|a| a.count).sum();

    // ========== 3. READABILITY ENHANCEMENTS ==========
    let readability_patterns = vec![
        "is one of", "are one of", "was one of", "were one of",
        "there is", "there are", "there was", "there were",
        "it is", "it was", "this is", "that is",
        "becomes particularly", "seems particularly", "appears particularly",
        "in order to", "due to the fact", "at this point in time",
    ];
    
    let mut readability_map: std::collections::HashMap<String, Vec<(String, usize, usize, String)>> = 
        std::collections::HashMap::new();
    
    for paragraph in &payload.data {
        let text = &paragraph.text;
        let text_lower = text.to_lowercase();
        
        for pattern in &readability_patterns {
            let pattern_lower = pattern.to_lowercase();
            let mut start_pos = 0;
            
            while let Some(pos) = text_lower[start_pos..].find(&pattern_lower) {
                let actual_pos = start_pos + pos;
                let end_pos = actual_pos + pattern.chars().count();  // ✅ FIXED: Use .chars().count()
                
                let safe_start = find_char_boundary(text, actual_pos);
                let safe_end = find_char_boundary(text, end_pos);
                
                let char_start = text[..safe_start].chars().count();
                let char_end = text[..safe_end].chars().count();
                let text_chars: Vec<char> = text.chars().collect();
                
                let string: String = if char_end <= text_chars.len() {
                    text_chars[char_start..char_end].iter().collect()
                } else {
                    text_chars[char_start..].iter().collect()
                };
                
                readability_map.entry(pattern.to_string())
                    .or_insert_with(Vec::new)
                    .push((paragraph.key.clone(), char_start, char_end, string));
                
                start_pos = end_pos;
            }
        }
    }
    
    let readability_enhancements: Vec<EnhancementOccurrence> = readability_map.into_iter().map(|(phrase, occurrences_list)| {
        let occurrences: Vec<OccurrenceDetail> = occurrences_list.into_iter().map(|(key, start, end, string)| {
            OccurrenceDetail { start, end, string, paragraph_key: key, report: "readabilityEnhancements".to_string() }
        }).collect();
        
        EnhancementOccurrence {
            phrase,
            count: occurrences.len(),
            occurrences,
        }
    }).collect();
    
    let readability_enhancements_found = readability_enhancements.iter().map(|r| r.count).sum();

    // ========== 4. INCLUSIVE LANGUAGE ==========
    let inclusive_patterns = vec![
        ("he or she", "they"), ("his or her", "their"), ("him or her", "them"),
        ("mankind", "humankind"), ("manpower", "workforce"), ("man-made", "artificial"),
        ("policeman", "police officer"), ("fireman", "firefighter"),
        ("chairman", "chairperson"), ("businessman", "businessperson"),
    ];
    
    let mut inclusive_map: std::collections::HashMap<String, Vec<(String, usize, usize, String)>> = 
        std::collections::HashMap::new();
    
    for paragraph in &payload.data {
        let text = &paragraph.text;
        let text_lower = text.to_lowercase();
        
        for (pattern, _suggestion) in &inclusive_patterns {
            let pattern_lower = pattern.to_lowercase();
            let mut start_pos = 0;
            
            while let Some(pos) = text_lower[start_pos..].find(&pattern_lower) {
                let actual_pos = start_pos + pos;
                let end_pos = actual_pos + pattern.chars().count();  // ✅ FIXED: Use .chars().count()
                
                let safe_start = find_char_boundary(text, actual_pos);
                let safe_end = find_char_boundary(text, end_pos);
                
                let char_start = text[..safe_start].chars().count();
                let char_end = text[..safe_end].chars().count();
                let text_chars: Vec<char> = text.chars().collect();
                
                let string: String = if char_end <= text_chars.len() {
                    text_chars[char_start..char_end].iter().collect()
                } else {
                    text_chars[char_start..].iter().collect()
                };
                
                inclusive_map.entry(pattern.to_string())
                    .or_insert_with(Vec::new)
                    .push((paragraph.key.clone(), char_start, char_end, string));
                
                start_pos = end_pos;
            }
        }
    }
    
    let inclusive_language_improvements: Vec<EnhancementOccurrence> = inclusive_map.into_iter().map(|(phrase, occurrences_list)| {
        let occurrences: Vec<OccurrenceDetail> = occurrences_list.into_iter().map(|(key, start, end, string)| {
            OccurrenceDetail { start, end, string, paragraph_key: key, report: "inclusiveLanguage".to_string() }
        }).collect();
        
        EnhancementOccurrence {
            phrase,
            count: occurrences.len(),
            occurrences,
        }
    }).collect();

    // ========== 5. EMOTION TELLS ==========
    let emotion_tell_words = vec![
        "felt", "seemed", "appeared", "looked", "sounded",
        "realized", "thought", "knew", "understood", "wondered",
        "decided", "noticed", "saw", "heard", "smelled",
    ];
    
    let mut emotion_map: std::collections::HashMap<String, Vec<(String, usize, usize, String)>> = 
        std::collections::HashMap::new();
    
    for paragraph in &payload.data {
        let text = &paragraph.text;
        let text_lower = text.to_lowercase();
        
        for pattern in &emotion_tell_words {
            let pattern_lower = pattern.to_lowercase();
            let mut start_pos = 0;
            
            while let Some(pos) = text_lower[start_pos..].find(&pattern_lower) {
                let actual_pos = start_pos + pos;
                let end_pos = actual_pos + pattern.chars().count();  // ✅ FIXED: Use .chars().count()
                
                // Check if it's a whole word (not part of another word)
                let is_word_start = actual_pos == 0 || 
                    text_lower.chars().nth(actual_pos - 1).map_or(false, |c| !c.is_alphabetic());
                let is_word_end = end_pos >= text_lower.chars().count() ||  // ✅ FIXED: Use .chars().count()
                    text_lower.chars().nth(end_pos).map_or(false, |c| !c.is_alphabetic());
                
                if is_word_start && is_word_end {
                    let safe_start = find_char_boundary(text, actual_pos);
                    let safe_end = find_char_boundary(text, end_pos);
                    
                    // Convert to character positions
                    let char_start = text[..safe_start].chars().count();
                    let char_end = text[..safe_end].chars().count();
                    let text_chars: Vec<char> = text.chars().collect();
                    
                    // Extract the actual string
                    let string: String = if char_end <= text_chars.len() {
                        text_chars[char_start..char_end].iter().collect()
                    } else {
                        text_chars[char_start..].iter().collect()
                    };
                    
                    emotion_map.entry(pattern_lower.clone())
                        .or_insert_with(Vec::new)
                        .push((paragraph.key.clone(), char_start, char_end, string));
                }
                
                start_pos = actual_pos + 1;
            }
        }
    }
    
    let emotion_tells: Vec<EnhancementOccurrence> = emotion_map.into_iter().map(|(phrase, occurrences_list)| {
        let occurrences: Vec<OccurrenceDetail> = occurrences_list.into_iter().map(|(key, start, end, string)| {
            OccurrenceDetail { start, end, string, paragraph_key: key, report: "emotionTells".to_string() }
        }).collect();
        
        EnhancementOccurrence {
            phrase,
            count: occurrences.len(),
            occurrences,
        }
    }).collect();

    // ========== 6. STYLE IMPROVEMENTS ==========
    let style_patterns = vec![
        "very", "really", "just", "actually", "basically", "literally",
        "quite", "rather", "somewhat", "pretty", "kind of", "sort of",
    ];
    
    let mut style_map: std::collections::HashMap<String, Vec<(String, usize, usize, String)>> = 
        std::collections::HashMap::new();
    
    for paragraph in &payload.data {
        let text = &paragraph.text;
        let text_lower = text.to_lowercase();
        
        for pattern in &style_patterns {
            let pattern_lower = pattern.to_lowercase();
            let mut start_pos = 0;
            
            while let Some(pos) = text_lower[start_pos..].find(&pattern_lower) {
                let actual_pos = start_pos + pos;
                let end_pos = actual_pos + pattern.len();
                
                // Convert to character positions for boundary checking
                let char_pos_start = text[..actual_pos].chars().count();
                let char_pos_end = text[..end_pos.min(text.len())].chars().count();
                let text_chars: Vec<char> = text.chars().collect();
                
                // Check if it's a whole word using character positions
                let is_word_start = char_pos_start == 0 || 
                    (char_pos_start > 0 && !text_chars[char_pos_start - 1].is_alphabetic());
                let is_word_end = char_pos_end >= text_chars.len() || 
                    !text_chars[char_pos_end].is_alphabetic();
                
                if is_word_start && is_word_end {
                    let safe_start = find_char_boundary(text, actual_pos);
                    let safe_end = find_char_boundary(text, end_pos);
                    
                    let char_start = text[..safe_start].chars().count();
                    let char_end = text[..safe_end].chars().count();
                    
                    let string: String = if char_end <= text_chars.len() {
                        text_chars[char_start..char_end].iter().collect()
                    } else {
                        text_chars[char_start..].iter().collect()
                    };
                    
                    style_map.entry(pattern.to_string())
                        .or_insert_with(Vec::new)
                        .push((paragraph.key.clone(), char_start, char_end, string));
                }
                
                start_pos = actual_pos + 1;
            }
        }
    }
    
    let style_improvements: Vec<EnhancementOccurrence> = style_map.into_iter().map(|(phrase, occurrences_list)| {
        let occurrences: Vec<OccurrenceDetail> = occurrences_list.into_iter().map(|(key, start, end, string)| {
            OccurrenceDetail { start, end, string, paragraph_key: key, report: "styleImprovements".to_string() }
        }).collect();
        
        EnhancementOccurrence {
            phrase,
            count: occurrences.len(),
            occurrences,
        }
    }).collect();

    // ========== 7. LONG SUBORDINATE CLAUSES ==========
    // Detect sentences with many commas (indicator of complex clauses)
    let mut long_clauses: Vec<EnhancementOccurrence> = Vec::new();
    
    for paragraph in &payload.data {
        let text = &paragraph.text;
        let text_chars: Vec<char> = text.chars().collect();
        let text_len = text_chars.len();
        
        let mut current_pos = 0;
        
        while current_pos < text_len {
            // Find the next sentence ending
            let mut sentence_end = current_pos;
            let mut found_delimiter = false;
            
            for i in current_pos..text_len {
                if text_chars[i] == '.' || text_chars[i] == '!' || text_chars[i] == '?' {
                    sentence_end = i + 1; // Include the delimiter
                    found_delimiter = true;
                    break;
                }
            }
            
            // If no delimiter found, take rest of text
            if !found_delimiter {
                sentence_end = text_len;
            }
            
            // Extract sentence
            let sentence_chars: Vec<char> = text_chars[current_pos..sentence_end].iter().cloned().collect();
            let sentence: String = sentence_chars.iter().collect();
            let sentence_trimmed = sentence.trim();
            
            if !sentence_trimmed.is_empty() {
                let comma_count = sentence.matches(',').count();
                
                if comma_count >= 3 {
                    let sentence_char_count = sentence_chars.len();
                    let excerpt = if sentence_char_count > 50 {
                        sentence_chars.iter().take(50).collect::<String>() + "..."
                    } else {
                        sentence.clone()
                    };
                    
                    long_clauses.push(EnhancementOccurrence {
                        phrase: format!("Complex sentence with {} commas", comma_count),
                        count: 1,
                        occurrences: vec![OccurrenceDetail {
                            start: current_pos,
                            end: sentence_end,  // Now includes the punctuation
                            string: excerpt,
                            paragraph_key: paragraph.key.clone(),
                            report: "longSubordinateClauses".to_string(),
                        }],
                    });
                }
            }
            
            current_pos = sentence_end;
        }
    }

    // ========== 8. REPEATED SENTENCE STARTS ==========
    let mut sentence_starts: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    
    for paragraph in &payload.data {
        let text = &paragraph.text;
        let sentences: Vec<&str> = text.split(&['.', '!', '?'][..])
            .filter(|s| !s.trim().is_empty())
            .collect();
        
        for sentence in sentences {
            let trimmed = sentence.trim();
            if let Some(first_word) = trimmed.split_whitespace().next() {
                let first_word_clean = first_word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
                
                if first_word_clean.len() > 2 {  // Ignore very short words
                    sentence_starts.entry(first_word_clean)
                        .or_insert_with(Vec::new)
                        .push(trimmed.chars().take(50).collect::<String>());
                }
            }
        }
    }
    
    let repeated_sentence_starts: Vec<RepeatedStart> = sentence_starts.into_iter()
        .filter(|(_, sentences)| sentences.len() > 2)  // Only if repeated 3+ times
        .map(|(start_word, sentences)| RepeatedStart {
            start_word,
            count: sentences.len(),
            sentences,
        })
        .collect();

    // ========== 9. BUSINESS JARGON ==========
    let business_jargon_items: Vec<EnhancementOccurrence> = full_report.business_jargon.jargon_list
        .into_iter()
        .map(|item| EnhancementOccurrence {
            phrase: item.jargon.clone(),
            count: item.count,
            occurrences: Vec::new(),
        })
        .collect();

    // ========== 10. STYLE GUIDE ITEMS ==========
    let style_guide_patterns = vec![
        ("alot", "a lot"), ("awhile", "a while"), ("irregardless", "regardless"),
        ("could of", "could have"), ("should of", "should have"),
        ("for all intensive purposes", "for all intents and purposes"),
    ];
    
    let mut style_guide_map: std::collections::HashMap<String, Vec<(String, usize, usize, String)>> = 
        std::collections::HashMap::new();
    
    for paragraph in &payload.data {
        let text = &paragraph.text;
        let text_lower = text.to_lowercase();
        
        for (pattern, _correction) in &style_guide_patterns {
            let pattern_lower = pattern.to_lowercase();
            let mut start_pos = 0;
            
            while let Some(pos) = text_lower[start_pos..].find(&pattern_lower) {
                let actual_pos = start_pos + pos;
                let end_pos = actual_pos + pattern.chars().count();  // ✅ FIXED: Use .chars().count()
                
                let safe_start = find_char_boundary(text, actual_pos);
                let safe_end = find_char_boundary(text, end_pos);
                
                let char_start = text[..safe_start].chars().count();
                let char_end = text[..safe_end].chars().count();
                let text_chars: Vec<char> = text.chars().collect();
                
                let string: String = if char_end <= text_chars.len() {
                    text_chars[char_start..char_end].iter().collect()
                } else {
                    text_chars[char_start..].iter().collect()
                };
                
                style_guide_map.entry(pattern.to_string())
                    .or_insert_with(Vec::new)
                    .push((paragraph.key.clone(), char_start, char_end, string));
                
                start_pos = end_pos;
            }
        }
    }
    
    let style_guide_items: Vec<EnhancementOccurrence> = style_guide_map.into_iter().map(|(phrase, occurrences_list)| {
        let occurrences: Vec<OccurrenceDetail> = occurrences_list.into_iter().map(|(key, start, end, string)| {
            OccurrenceDetail { start, end, string, paragraph_key: key, report: "styleGuideItems".to_string() }
        }).collect();
        
        EnhancementOccurrence {
            phrase,
            count: occurrences.len(),
            occurrences,
        }
    }).collect();

    // ========== GENERATE INTELLIGENT MESSAGES ==========
    
    // Passive Verbs Message
    let passive_verbs_message = if total_passive_count == 0 {
        "Excellent! No passive voice found.".to_string()
    } else if total_passive_count <= 2 {
        format!("{} passive verb{} found. Consider revising for stronger, more direct writing.", 
            total_passive_count, if total_passive_count == 1 { "" } else { "s" })
    } else if total_passive_count <= 5 {
        format!("{} passive verbs found. Try to use active voice where possible for clearer, more engaging prose.", total_passive_count)
    } else {
        format!("{} passive verbs found. High usage detected - revise to strengthen your writing with active voice.", total_passive_count)
    };
    
    // Hidden Verbs Message
    let hidden_verbs_message = if hidden_verbs_found == 0 {
        "No hidden verbs found.".to_string()
    } else if hidden_verbs_found == 1 {
        "1 hidden verb (nominalization) found. Consider using the direct verb form instead.".to_string()
    } else {
        format!("{} hidden verbs (nominalizations) found. Replace weak verb + noun constructions with stronger direct verbs.", hidden_verbs_found)
    };
    
    // Adverbs Message
    let adverbs_message = if adverbs_outside_dialogue == 0 {
        "No adverbs found outside dialogue.".to_string()
    } else if adverbs_outside_dialogue <= 5 {
        format!("{} adverb{} found outside dialogue. Use sparingly.", 
            adverbs_outside_dialogue, if adverbs_outside_dialogue == 1 { "" } else { "s" })
    } else if adverbs_outside_dialogue <= 15 {
        format!("{} adverbs found outside dialogue. Consider reducing - use stronger verbs instead of adverb + weak verb combinations.", adverbs_outside_dialogue)
    } else {
        format!("{} adverbs found outside dialogue. High usage detected - replace with stronger verbs for more impactful writing.", adverbs_outside_dialogue)
    };
    
    // Readability Enhancements Message
    let readability_enhancements_message = if readability_enhancements_found == 0 {
        "No weak constructions found.".to_string()
    } else if readability_enhancements_found <= 3 {
        format!("{} weak construction{} found. Consider revising for clarity.", 
            readability_enhancements_found, if readability_enhancements_found == 1 { "" } else { "s" })
    } else if readability_enhancements_found <= 10 {
        format!("{} weak constructions found. Remove 'there is/are' and 'it was/is' constructions for stronger prose.", readability_enhancements_found)
    } else {
        format!("{} weak constructions found. High usage - revise extensively for clearer, more direct writing.", readability_enhancements_found)
    };
    
    // Inclusive Language Message
    let inclusive_count = inclusive_language_improvements.len();
    let inclusive_language_message = if inclusive_count == 0 {
        "No non-inclusive language detected.".to_string()
    } else if inclusive_count == 1 {
        "1 instance of gendered/non-inclusive language found. Consider using gender-neutral alternatives.".to_string()
    } else {
        format!("{} instances of gendered/non-inclusive language found. Update to gender-neutral alternatives for broader inclusivity.", inclusive_count)
    };
    
    // Emotion Tells Message
    let emotion_tells_count = emotion_tells.iter().map(|e| e.count).sum::<usize>();
    let emotion_tells_message = if emotion_tells_count == 0 {
        "No emotion tells found.".to_string()
    } else if emotion_tells_count <= 5 {
        format!("{} emotion tell{} found (e.g., 'felt', 'seemed'). Show, don't tell emotions.", 
            emotion_tells_count, if emotion_tells_count == 1 { "" } else { "s" })
    } else if emotion_tells_count <= 15 {
        format!("{} emotion tells found. 'Show, don't tell' - replace filter words with vivid descriptions and actions.", emotion_tells_count)
    } else {
        format!("{} emotion tells found. High usage - extensively revise to show emotions through actions and descriptions rather than telling.", emotion_tells_count)
    };
    
    // Style Improvements Message
    let style_count = style_improvements.iter().map(|s| s.count).sum::<usize>();
    let style_improvements_message = if style_count == 0 {
        "No filler words found.".to_string()
    } else if style_count <= 5 {
        format!("{} filler word{} found (e.g., 'very', 'really'). Remove for stronger writing.", 
            style_count, if style_count == 1 { "" } else { "s" })
    } else {
        format!("{} filler words found. These weaken your prose - remove or replace with stronger word choices.", style_count)
    };
    
    // Business Jargon Message
    let jargon_count = business_jargon_items.iter().map(|b| b.count).sum::<usize>();
    let business_jargon_message = if jargon_count == 0 {
        "No business jargon found.".to_string()
    } else if jargon_count <= 3 {
        format!("{} instance{} of business jargon found. Consider using plain language.", 
            jargon_count, if jargon_count == 1 { "" } else { "s" })
    } else {
        format!("{} instances of business jargon found. Replace corporate buzzwords with clear, direct language.", jargon_count)
    };
    
    // Long Subordinate Clauses Message
    let long_clauses_count = long_clauses.len();
    let long_subordinate_clauses_message = if long_clauses_count == 0 {
        "No overly complex sentences found.".to_string()
    } else if long_clauses_count <= 3 {
        format!("{} complex sentence{} found (3+ commas). Consider breaking into shorter sentences.", 
            long_clauses_count, if long_clauses_count == 1 { "" } else { "s" })
    } else if long_clauses_count <= 10 {
        format!("{} complex sentences found. Break up long sentences to improve readability.", long_clauses_count)
    } else {
        format!("{} complex sentences found. High usage - extensively revise to improve flow and clarity.", long_clauses_count)
    };
    
    // Passive Index Message
    let passive_index_message = if passive_index < 10.0 {
        format!("Excellent! Passive index: {:.1}% (target: up to 25%). Your writing uses active voice effectively.", passive_index)
    } else if passive_index <= 25.0 {
        format!("Good. Passive index: {:.1}% (target: up to 25%). Within acceptable range.", passive_index)
    } else if passive_index <= 40.0 {
        format!("High passive voice: {:.1}% (target: up to 25%). Consider revising to use more active voice.", passive_index)
    } else {
        format!("Very high passive voice: {:.1}% (target: up to 25%). Extensive revision needed - convert to active voice.", passive_index)
    };
    
    // Repeated Sentence Starts Message
    let repeated_count = repeated_sentence_starts.len();
    let repeated_sentence_starts_message = if repeated_count == 0 {
        "No repetitive sentence starts found.".to_string()
    } else if repeated_count <= 3 {
        format!("{} word{} used to start multiple sentences. Vary your sentence beginnings for better flow.", 
            repeated_count, if repeated_count == 1 { "" } else { "s" })
    } else {
        format!("{} words repeatedly used to start sentences. Add variety to sentence beginnings to improve rhythm and engagement.", repeated_count)
    };
    
    // Style Guide Items Message
    let style_guide_count = style_guide_items.iter().map(|s| s.count).sum::<usize>();
    let style_guide_items_message = if style_guide_count == 0 {
        "No style guide violations found.".to_string()
    } else if style_guide_count == 1 {
        "1 common grammar/style mistake found (e.g., 'alot', 'should of'). Review and correct.".to_string()
    } else {
        format!("{} common grammar/style mistakes found. Review and correct these errors.", style_guide_count)
    };

    let response = PassiveVoiceResponse {
        passive_verbs_found: total_passive_count,
        passive_verbs_message,
        passive_verbs,
        hidden_verbs_found,
        hidden_verbs_message,
        hidden_verbs,
        adverbs_in_dialogue: 0,  // TODO: Need dialogue detection
        adverbs_outside_dialogue,
        adverbs_message,
        adverbs_list,
        readability_enhancements_found,
        readability_enhancements_message,
        readability_enhancements,
        inclusive_language_message,
        inclusive_language_improvements,
        emotion_tells_message,
        emotion_tells,
        style_improvements_message,
        style_improvements,
        business_jargon_message,
        business_jargon: business_jargon_items,
        long_subordinate_clauses_message,
        long_subordinate_clauses: long_clauses,
        passive_index,
        passive_index_message,
        passive_index_target: "up to 25".to_string(),
        repeated_sentence_starts_message,
        repeated_sentence_starts,
        style_guide_items_message,
        style_guide_items,
    };

    Ok(Json(response))
}

// Handler for /glueindex endpoint
async fn get_glue_index(
    Json(payload): Json<SentenceLengthRequest>,
) -> Result<Json<GlueIndexResponse>, ApiError> {
    // Work with whatever is given - no validation
    if payload.data.is_empty() {
        // Return empty response instead of error
        return Ok(Json(GlueIndexResponse {
            glue_index: 0.0,
            glue_index_target: "up to 40%".to_string(),
            sentences: Vec::new(),
        }));
    }

    // Combine all texts for overall glue index
    let combined_text: String = payload.data.iter()
        .map(|p| p.text.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    // Work with empty text too - just return empty results
    if combined_text.trim().is_empty() {
        return Ok(Json(GlueIndexResponse {
            glue_index: 0.0,
            glue_index_target: "up to 40%".to_string(),
            sentences: Vec::new(),
        }));
    }

    let config = Config::default();
    let analyzer = TextAnalyzer::new(combined_text.clone(), config)
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    let full_report = analyzer.generate_full_report()
        .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

    // Calculate overall glue index - now using glue_index field directly
    let glue_index = (full_report.sticky_sentences.glue_index * 10.0).round() / 10.0;

    // Process each paragraph to find sticky/semi-sticky sentences with paragraph-relative positions
    let mut all_sticky_sentences = Vec::new();
    let mut all_semi_sticky_sentences = Vec::new();

    for paragraph in &payload.data {
        let text = &paragraph.text;
        let para_key = paragraph.key.clone();  // Capture key explicitly at start of loop
        
        if text.trim().is_empty() {
            continue;
        }

        // Analyze this paragraph
        let para_config = Config::default();
        let para_analyzer = TextAnalyzer::new(text.clone(), para_config)
            .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

        let para_report = para_analyzer.generate_full_report()
            .map_err(|e| ApiError::AnalysisError(e.to_string()))?;

        // Process sticky sentences (>45% glue) - now using fields directly from struct
        for s in para_report.sticky_sentences.sticky_sentences {
            // Use the sentence text directly from the analyzer - it's already complete!
            let string = s.sentence.clone();
            
            // Convert byte positions to character positions for the API response
            let safe_start = find_char_boundary(text, s.start_index);
            let safe_end = find_char_boundary(text, s.end_index.min(text.len()));
            let char_start = text[..safe_start].chars().count();
            let char_end = text[..safe_end].chars().count();
            
            let char_length = string.chars().count();
            let excerpt = if char_length > 50 {
                string.chars().take(50).collect::<String>() + "..."
            } else {
                string.clone()
            };

            all_sticky_sentences.push(StickySentence {
                start: char_start,
                end: char_end,
                string,
                excerpt,
                glue_percentage: s.glue_percentage,
                category: "sticky".to_string(),  // >45% glue
                paragraph_key: para_key.clone(),  // Use the explicitly captured key
            });
        }

        // Process semi-sticky sentences (35-45% glue) - now using field directly from struct
        for s in para_report.sticky_sentences.semi_sticky_sentences {
            // Use the sentence text directly from the analyzer - it's already complete!
            let string = s.sentence.clone();
            
            // Convert byte positions to character positions for the API response
            let safe_start = find_char_boundary(text, s.start_index);
            let safe_end = find_char_boundary(text, s.end_index.min(text.len()));
            let char_start = text[..safe_start].chars().count();
            let char_end = text[..safe_end].chars().count();
            
            let char_length = string.chars().count();
            let excerpt = if char_length > 50 {
                string.chars().take(50).collect::<String>() + "..."
            } else {
                string.clone()
            };

            all_semi_sticky_sentences.push(StickySentence {
                start: char_start,
                end: char_end,
                string,
                excerpt,
                glue_percentage: s.glue_percentage,
                category: "semi-sticky".to_string(),  // 35-45% glue
                paragraph_key: para_key.clone(),  // Use the explicitly captured key
            });
        }
    }

    // Combine sticky and semi-sticky into single array
    let mut all_sentences = all_sticky_sentences;
    all_sentences.extend(all_semi_sticky_sentences);

    let response = GlueIndexResponse {
        glue_index,
        glue_index_target: "up to 40%".to_string(),
        sentences: all_sentences,
    };

    Ok(Json(response))
}

// Helper function to find the nearest valid character boundary
fn find_char_boundary(text: &str, byte_index: usize) -> usize {
    if byte_index >= text.len() {
        return text.len();
    }
    
    // If already at a char boundary, return it
    if text.is_char_boundary(byte_index) {
        return byte_index;
    }
    
    // Search backwards for the nearest char boundary
    for i in (0..=byte_index).rev() {
        if text.is_char_boundary(i) {
            return i;
        }
    }
    
    0
}

// Create user-friendly scores with ideal values and quality messages
fn create_user_friendly_scores(
    full_report: &Rust_Grammar::FullAnalysisReport,
    passive_voice: &[PassiveVoiceMatch],
    _grammar: &[GrammarIssue],
    stats: &Rust_Grammar::TextStatistics,
    readability: &Rust_Grammar::ReadabilityMetrics,
    text: &str,
) -> UserFriendlyScores {
    let sentence_count = stats.sentence_count.max(1) as f64;
    let word_count = stats.word_count.max(1) as f64;
    
    // Calculate key metrics
    let passive_percentage = (passive_voice.len() as f64 / sentence_count) * 100.0;
    let glue_percentage = full_report.sticky_sentences.overall_glue_index;
    let jargon_count = full_report.business_jargon.total_jargon;
    let conjunction_percentage = full_report.conjunction_starts.percentage;
    let slow_pacing_percentage = full_report.pacing.slow_paced_percentage;
    let very_long_count = full_report.sentence_length.very_long_sentences;
    let very_long_percentage = (very_long_count as f64 / sentence_count) * 100.0;
    
    // Count weak adverbs
    let weak_adverbs = count_weak_adverbs(text);
    let weak_adverbs_percentage = (weak_adverbs as f64 / word_count) * 100.0;
    
    // Count emotion tells
    let emotion_tells = count_emotion_tells(text);
    
    // Count ing starts
    let ing_starts = count_ing_starts(text);
    let ing_percentage = (ing_starts as f64 / sentence_count) * 100.0;
    
    // Dialogue analysis
    let dialogue_tags = analyze_dialogue(text);
    
    UserFriendlyScores {
        style_score: SimpleScore {
            current: (full_report.style_score as f64 * 100.0).round() / 100.0,
            ideal: "80-100".to_string(),
            status: if full_report.style_score >= 80 { "good" } else if full_report.style_score >= 60 { "fair" } else { "needs improvement" }.to_string(),
            message: get_style_score_message(full_report.style_score),
        },
        
        sentence_length: SimpleScore {
            current: (full_report.sentence_length.avg_length * 100.0).round() / 100.0,
            ideal: "15-20 words".to_string(),
            status: if full_report.sentence_length.avg_length >= 15.0 && full_report.sentence_length.avg_length <= 20.0 { "good" } else if full_report.sentence_length.avg_length >= 12.0 && full_report.sentence_length.avg_length <= 25.0 { "fair" } else { "needs improvement" }.to_string(),
            message: get_sentence_length_message(full_report.sentence_length.avg_length),
        },
        
        readability: SimpleScore {
            current: (readability.flesch_kincaid_grade * 100.0).round() / 100.0,
            ideal: "7-9 grade level".to_string(),
            status: if readability.flesch_kincaid_grade >= 7.0 && readability.flesch_kincaid_grade <= 9.0 { "good" } else if readability.flesch_kincaid_grade >= 5.0 && readability.flesch_kincaid_grade <= 12.0 { "fair" } else { "needs improvement" }.to_string(),
            message: get_readability_message(readability.flesch_kincaid_grade),
        },
        
        sentence_variety: SimpleScore {
            current: (full_report.sentence_length.std_deviation * 100.0).round() / 100.0,
            ideal: "5-10".to_string(),
            status: if full_report.sentence_length.std_deviation >= 5.0 { "good" } else if full_report.sentence_length.std_deviation >= 3.0 { "fair" } else { "needs improvement" }.to_string(),
            message: get_variety_message(full_report.sentence_length.std_deviation),
        },
        
        glue_index: SimpleScore {
            current: (glue_percentage * 100.0).round() / 100.0,
            ideal: "< 40%".to_string(),
            status: if glue_percentage < 40.0 { "good" } else if glue_percentage < 45.0 { "fair" } else { "needs improvement" }.to_string(),
            message: get_glue_message(glue_percentage),
        },
        
        passive_voice: SimpleScore {
            current: (passive_percentage * 100.0).round() / 100.0,
            ideal: "< 10%".to_string(),
            status: if passive_percentage < 10.0 { "good" } else if passive_percentage < 20.0 { "fair" } else { "needs improvement" }.to_string(),
            message: get_passive_message(passive_percentage, passive_voice.len()),
        },
        
        business_jargon: SimpleScore {
            current: ((jargon_count as f64) * 100.0).round() / 100.0,
            ideal: "0 instances".to_string(),
            status: if jargon_count == 0 { "good" } else if jargon_count <= 2 { "fair" } else { "needs improvement" }.to_string(),
            message: get_jargon_message(jargon_count),
        },
        
        complex_paragraphs: SimpleScore {
            current: (((full_report.complex_paragraphs.complex_paragraph_count as f64 / stats.paragraph_count.max(1) as f64) * 100.0) * 100.0).round() / 100.0,
            ideal: "0%".to_string(),
            status: if full_report.complex_paragraphs.complex_paragraph_count == 0 { "good" } else { "needs improvement" }.to_string(),
            message: get_complex_paragraphs_message(full_report.complex_paragraphs.complex_paragraph_count),
        },
        
        conjunction_starts: SimpleScore {
            current: (conjunction_percentage * 100.0).round() / 100.0,
            ideal: "< 10%".to_string(),
            status: if conjunction_percentage < 10.0 { "good" } else if conjunction_percentage < 15.0 { "fair" } else { "needs improvement" }.to_string(),
            message: get_conjunction_starts_message(conjunction_percentage),
        },
        
        slow_pacing: SimpleScore {
            current: (slow_pacing_percentage * 100.0).round() / 100.0,
            ideal: "< 30%".to_string(),
            status: if slow_pacing_percentage < 30.0 { "good" } else if slow_pacing_percentage < 40.0 { "fair" } else { "needs improvement" }.to_string(),
            message: get_slow_pacing_message(slow_pacing_percentage),
        },
        
        very_long_sentences: SimpleScore {
            current: (very_long_percentage * 100.0).round() / 100.0,
            ideal: "< 10%".to_string(),
            status: if very_long_percentage < 10.0 { "good" } else if very_long_percentage < 15.0 { "fair" } else { "needs improvement" }.to_string(),
            message: get_very_long_sentences_message(very_long_count, very_long_percentage),
        },
        
        emotion_tells: SimpleScore {
            // current: ((emotion_tells as f64) * 100.0).round() / 100.0,
            current: (((emotion_tells as f64 / sentence_count) * 100.0) * 100.0).round() / 100.0,
            ideal: "0 instances".to_string(),
            status: if emotion_tells == 0 { "good" } else if emotion_tells <= 3 { "fair" } else { "needs improvement" }.to_string(),
            message: get_emotion_tells_message(emotion_tells),
        },
        
        ing_starts: SimpleScore {
            current: (ing_percentage * 100.0).round() / 100.0,
            ideal: "< 10%".to_string(),
            status: if ing_percentage < 10.0 { "good" } else if ing_percentage < 15.0 { "fair" } else { "needs improvement" }.to_string(),
            message: get_ing_starts_message(ing_starts, ing_percentage),
        },
        
        weak_adverbs: SimpleScore {
            current: (weak_adverbs_percentage * 100.0).round() / 100.0,
            ideal: "< 5%".to_string(),
            status: if weak_adverbs_percentage < 5.0 { "good" } else if weak_adverbs_percentage < 10.0 { "fair" } else { "needs improvement" }.to_string(),
            message: get_weak_adverbs_message(weak_adverbs, weak_adverbs_percentage),
        },
        
        dialogue_tags: SimpleScore {
            current: (dialogue_tags.0.0 * 100.0).round() / 100.0,
            ideal: "< 50%".to_string(),
            status: if dialogue_tags.0.0 < 50.0 { "good" } else if dialogue_tags.0.0 < 70.0 { "fair" } else { "needs improvement" }.to_string(),
            message: get_dialogue_tags_message(dialogue_tags.0.1),
        },
    }
}


fn calculate_comprehensive_scores(
    full_report: &Rust_Grammar::FullAnalysisReport,
    passive_voice: &[PassiveVoiceMatch],
    _grammar: &[GrammarIssue],
    stats: &Rust_Grammar::TextStatistics,
    readability: &Rust_Grammar::ReadabilityMetrics,
    text: &str,
) -> ComprehensiveScores {
    let sentence_count = stats.sentence_count.max(1) as f64;
    let word_count = stats.word_count.max(1) as f64;
    
    // Calculate individual sentence details with positions
    let sentences = split_into_sentences(text);
    let mut individual_lengths = Vec::new();
    
    // Convert text to character array for accurate Unicode indexing
    let text_chars: Vec<char> = text.chars().collect();
    let mut char_pos = 0;
    
    for sentence in &sentences {
        let sentence_trimmed = sentence.trim();
        let sentence_chars: Vec<char> = sentence_trimmed.chars().collect();
        
        // Find sentence in character array
        let mut found = false;
        for i in char_pos..text_chars.len() {
            if i + sentence_chars.len() <= text_chars.len() {
                let slice: Vec<char> = text_chars[i..i + sentence_chars.len()].to_vec();
                if slice == sentence_chars {
                    let start = i;
                    let end = i + sentence_chars.len();
                    let char_length = sentence_chars.len();
                    let word_count_val = sentence.split_whitespace().count();
                    
                    individual_lengths.push(SentenceOccurrence {
                        start,
                        end,
                        length: char_length,
                        string: if sentence.len() > 50 {
                            sentence.chars().take(50).collect::<String>() + "..."
                        } else {
                            sentence.clone()
                        },
                        word_count: word_count_val,
                        paragraph_key: format!("{}", estimate_paragraph(text, start)),
                    });
                    
                    char_pos = end;
                    found = true;
                    break;
                }
            }
        }
        
        // If not found with exact match, skip to next sentence
        if !found {
            char_pos += sentence.chars().count();
        }
    }
    
    // Calculate distribution
    let under_10 = sentences.iter().map(|s| s.split_whitespace().count()).filter(|&l| l < 10).count();
    let range_10_to_19 = sentences.iter().map(|s| s.split_whitespace().count()).filter(|l| *l >= 10 && *l < 20).count();
    let range_20_to_29 = sentences.iter().map(|s| s.split_whitespace().count()).filter(|l| *l >= 20 && *l < 30).count();
    let range_30_to_39 = sentences.iter().map(|s| s.split_whitespace().count()).filter(|l| *l >= 30 && *l < 40).count();
    let over_40 = sentences.iter().map(|s| s.split_whitespace().count()).filter(|l| *l >= 40).count();
    
    // Calculate percentages
    let passive_percentage = (passive_voice.len() as f64 / sentence_count) * 100.0;
    let conjunction_percentage = full_report.conjunction_starts.percentage;
    let slow_pacing_percentage = full_report.pacing.slow_paced_percentage;
    
    // Get very long sentences count (already calculated)
    let very_long_count = full_report.sentence_length.very_long_sentences;
    let very_long_percentage = (very_long_count as f64 / sentence_count) * 100.0;
    
    // Count -ing starts (approximation)
    let ing_starts = count_ing_starts(text);
    let ing_percentage = (ing_starts as f64 / sentence_count) * 100.0;
    let ing_starts_positions = get_ing_starts_positions(text);
    
    // Count weak adverbs (words ending in -ly)
    let weak_adverbs = count_weak_adverbs(text);
    let weak_adverbs_positions = get_weak_adverbs_positions(text);
    
    // Dialogue analysis (approximation based on quotes)
    let (dialogue_tags, unusual_tags, tags_with_adverbs) = analyze_dialogue(text);
    
    // Emotion tells (words like "felt", "seemed", "appeared")
    let emotion_tells = count_emotion_tells(text);
    
    ComprehensiveScores {
        style_score: ScoreDetail {
            score: full_report.style_score,
            percentage: full_report.style_score as f64,
            message: Some(get_style_message(full_report.style_score)),
        },
        
        style_guide_compliance: ScoreDetail {
            score: 100, // Default to 100 if no custom style guide
            percentage: 100.0,
            message: Some("Create your own style guide".to_string()),
        },
        
        sentence_length: SentenceLengthDetail {
            score: calculate_sentence_length_score(&full_report.sentence_length),
            percentage: calculate_sentence_length_score(&full_report.sentence_length) as f64,
            message: Some("Use a few longer sentences to add more depth to your writing.".to_string()),
            num_words: stats.word_count,
            num_characters: stats.character_count,
            avg_sentence_length: full_report.sentence_length.avg_length,
            target_range: "11 to 18".to_string(),
            sentence_variety: full_report.sentence_length.std_deviation,
            variety_target: "over 3".to_string(),
            sentences_by_word_count: SentenceDistribution {
                under_10,
                range_10_to_19,
                range_20_to_29,
                range_30_to_39,
                over_40,
            },
            individual_sentence_lengths: individual_lengths,
        },
        
        readability_grade: ScoreDetail {
            score: readability.flesch_kincaid_grade as i32,
            percentage: (readability.flesch_kincaid_grade / 16.0 * 100.0).min(100.0),
            message: None,
        },
        
        sentence_variety: ScoreDetail {
            score: calculate_variety_score(&full_report.sentence_length),
            percentage: calculate_variety_score(&full_report.sentence_length) as f64,
            message: None,
        },
        
        glue_index: PercentageScore {
            percentage: full_report.sticky_sentences.overall_glue_index,
            count: full_report.sticky_sentences.sticky_sentences.len(),
            total: stats.sentence_count,
            message: Some("Reduce the number of glue words to make your writing clearer.".to_string()),
            occurrences: Some(full_report.sticky_sentences.sticky_sentences.iter().map(|s| Occurrence {
                start: s.start_index,
                end: s.end_index,
                length: s.length,
                string: s.sentence.chars().take(50).collect::<String>() + if s.sentence.len() > 50 { "..." } else { "" },
                paragraph_key: format!("{}", estimate_paragraph(text, s.start_index)),
            }).collect()),
        },
        
        passive_voice: CountScore {
            count: passive_voice.len(),
            percentage: Some(passive_percentage),
            message: None,
            occurrences: Some(passive_voice.iter().map(|pv| Occurrence {
                start: pv.start_index,
                end: pv.end_index,
                length: pv.length,
                string: pv.text.clone(),
                paragraph_key: format!("{}", estimate_paragraph(text, pv.start_index)),
            }).collect()),
        },
        
        business_jargon: CountScore {
            count: full_report.business_jargon.total_jargon,
            percentage: Some((full_report.business_jargon.total_jargon as f64 / word_count) * 100.0),
            message: None,
            occurrences: Some(full_report.business_jargon.jargon_list.iter()
                .flat_map(|j| j.occurrences.iter().map(move |occ| Occurrence {
                    start: occ.start_index,
                    end: occ.end_index,
                    length: occ.length,
                    string: j.jargon.clone(),
                    paragraph_key: format!("{}", estimate_paragraph(text, occ.start_index)),
                }))
                .collect()),
        },
        
        complex_paragraphs: PercentageScore {
            percentage: (full_report.complex_paragraphs.complex_paragraph_count as f64 / stats.paragraph_count.max(1) as f64) * 100.0,
            count: full_report.complex_paragraphs.complex_paragraph_count,
            total: stats.paragraph_count,
            message: None,
            occurrences: Some(full_report.complex_paragraphs.complex_paragraphs.iter().map(|p| Occurrence {
                start: p.start_index,
                end: p.end_index,
                length: p.length,
                string: format!("Paragraph {} (avg {:.1} words/sentence)", p.paragraph_num, p.avg_sentence_length),
                paragraph_key: format!("{}", p.paragraph_num),
            }).collect()),
        },
        
        conjunction_starts: PercentageScore {
            percentage: conjunction_percentage,
            count: full_report.conjunction_starts.count,
            total: stats.sentence_count,
            message: None,
            occurrences: None,
        },
        
        slow_pacing: PercentageScore {
            percentage: slow_pacing_percentage,
            count: full_report.pacing.pacing_distribution.slow,
            total: stats.sentence_count,
            message: None,
            occurrences: None,
        },
        
        very_long_sentences: PercentageScore {
            percentage: very_long_percentage,
            count: very_long_count,
            total: stats.sentence_count,
            message: None,
            occurrences: Some(full_report.sentence_length.very_long_details.iter().map(|(sent_num, word_count)| Occurrence {
                start: 0,  // Position not available in this report
                end: 0,
                length: 0,
                string: format!("Sentence {} ({} words)", sent_num, word_count),
                paragraph_key: format!("{}", sent_num / 3),  // Estimate paragraph
            }).collect()),
        },
        
        emotion_tells: CountScore {
            count: emotion_tells,
            percentage: Some((emotion_tells as f64 / word_count) * 100.0),
            message: None,
            occurrences: None,
        },
        
        ing_starts: PercentageScore {
            percentage: ing_percentage,
            count: ing_starts,
            total: stats.sentence_count,
            message: Some("Rewrite some of your sentences that start with words ending in \"-ing.\"".to_string()),
            occurrences: Some(ing_starts_positions),
        },
        
        dialogue_tags: PercentageScore {
            percentage: dialogue_tags.0,
            count: dialogue_tags.1,
            total: dialogue_tags.2,
            message: Some("Remove some dialogue tags to help your writing flow better.".to_string()),
            occurrences: None,
        },
        
        unusual_dialogue_tags: PercentageScore {
            percentage: unusual_tags.0,
            count: unusual_tags.1,
            total: unusual_tags.2,
            message: None,
            occurrences: None,
        },
        
        dialogue_tags_with_adverbs: PercentageScore {
            percentage: tags_with_adverbs.0,
            count: tags_with_adverbs.1,
            total: tags_with_adverbs.2,
            message: Some("Remove adverbs from dialogue tags and \"show\" emotions more.".to_string()),
            occurrences: None,
        },
        
        weak_adverbs: CountScore {
            count: weak_adverbs,
            percentage: Some((weak_adverbs as f64 / word_count) * 100.0),
            message: Some("Replace some adverbs with stronger verbs to improve engagement.".to_string()),
            occurrences: Some(weak_adverbs_positions),
        },
    }
}

fn convert_to_issues(
    full_report: &Rust_Grammar::FullAnalysisReport,
    passive_voice: &[PassiveVoiceMatch],
    grammar: &[GrammarIssue],
    text: &str,
) -> Vec<AnalysisIssue> {
    let mut issues = Vec::new();

    // Add passive voice
    for pv in passive_voice {
        issues.push(AnalysisIssue {
            id: format!("{}_{}_{}_{}_passive", "auto", pv.start_index, pv.end_index, pv.length),
            start: pv.start_index,
            length: pv.length,
            end: pv.end_index,
            paragraph_key: format!("{}", estimate_paragraph(text, pv.start_index)),
            string: pv.text.clone(),
            issue_type: "PassiveVoice".to_string(),
            suggestions: Suggestions {
                recommendation: vec!["Consider using active voice for clarity".to_string()],
            },
        });
    }

    // Add grammar issues
    for issue in grammar {
        let issue_text = extract_text(text, issue.start_index, issue.end_index);
        issues.push(AnalysisIssue {
            id: format!("{}_{}_{}_{}_grammar", "auto", issue.start_index, issue.end_index, issue.length),
            start: issue.start_index,
            length: issue.length,
            end: issue.end_index,
            paragraph_key: format!("{}", issue.sentence_num / 3),
            string: issue_text,
            issue_type: format!("Grammar_{:?}", issue.issue_type),
            suggestions: Suggestions {
                recommendation: vec![issue.message.clone()],
            },
        });
    }

    // Add other issues (clichés, jargon, etc.) - shortened for brevity
    for cliche in &full_report.cliches.cliches {
        for occ in &cliche.occurrences {
            issues.push(AnalysisIssue {
                id: format!("{}_{}_{}_{}_cliche", "auto", occ.start_index, occ.end_index, occ.length),
                start: occ.start_index,
                length: occ.length,
                end: occ.end_index,
                paragraph_key: format!("{}", estimate_paragraph(text, occ.start_index)),
                string: cliche.cliche.clone(),
                issue_type: "Cliche".to_string(),
                suggestions: Suggestions {
                    recommendation: vec!["Avoid clichés".to_string()],
                },
            });
        }
    }

    issues.sort_by_key(|i| i.start);
    issues
}

// Helper functions
fn get_style_message(score: i32) -> String {
    match score {
        90..=100 => "Excellent writing!".to_string(),
        80..=89 => "Good writing with minor improvements needed".to_string(),
        70..=79 => "Adequate writing, could be improved".to_string(),
        _ => "Needs significant improvement".to_string(),
    }
}

fn calculate_sentence_length_score(report: &Rust_Grammar::SentenceLengthReport) -> i32 {
    // Score based on variety and average
    let avg = report.avg_length;
    if avg >= 15.0 && avg <= 20.0 {
        10
    } else if avg >= 10.0 && avg <= 25.0 {
        8
    } else {
        6
    }
}

fn calculate_variety_score(report: &Rust_Grammar::SentenceLengthReport) -> i32 {
    // Score based on standard deviation (higher = more variety)
    let std_dev = report.std_deviation;
    if std_dev > 8.0 {
        10
    } else if std_dev > 5.0 {
        8
    } else {
        6
    }
}

fn count_ing_starts(text: &str) -> usize {
    use regex::Regex;
    let ing_pattern = Regex::new(r"(?m)^[A-Z]\w+ing\b").unwrap();
    ing_pattern.find_iter(text).count()
}

fn get_ing_starts_positions(text: &str) -> Vec<Occurrence> {
    use regex::Regex;
    let ing_pattern = Regex::new(r"(?m)^[A-Z]\w+ing\b").unwrap();
    ing_pattern.find_iter(text).map(|m| Occurrence {
        start: m.start(),
        end: m.end(),
        length: m.end() - m.start(),
        string: m.as_str().to_string(),
        paragraph_key: format!("{}", estimate_paragraph(text, m.start())),
    }).collect()
}

fn count_weak_adverbs(text: &str) -> usize {
    use regex::Regex;
    let adverb_pattern = Regex::new(r"\b\w+ly\b").unwrap();
    adverb_pattern.find_iter(text).count()
}

fn get_weak_adverbs_positions(text: &str) -> Vec<Occurrence> {
    use regex::Regex;
    let adverb_pattern = Regex::new(r"\b\w+ly\b").unwrap();
    adverb_pattern.find_iter(text).map(|m| Occurrence {
        start: m.start(),
        end: m.end(),
        length: m.end() - m.start(),
        string: m.as_str().to_string(),
        paragraph_key: format!("{}", estimate_paragraph(text, m.start())),
    }).collect()
}

fn count_emotion_tells(text: &str) -> usize {
    let emotion_words = ["felt", "seemed", "appeared", "looked like", "sounded like"];
    let lower = text.to_lowercase();
    emotion_words.iter()
        .map(|word| lower.matches(word).count())
        .sum()
}

fn count_complex_words(text: &str) -> usize {
    // Count words with 3 or more syllables
    text.split_whitespace()
        .filter(|word| {
            let clean_word = word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
            count_syllables(&clean_word) >= 3
        })
        .count()
}

fn count_syllables(word: &str) -> usize {
    if word.is_empty() {
        return 0;
    }
    
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    let mut count = 0;
    let mut previous_was_vowel = false;
    let chars: Vec<char> = word.chars().collect();
    
    for ch in chars.iter() {
        let is_vowel = vowels.contains(&ch.to_lowercase().next().unwrap_or(' '));
        
        if is_vowel && !previous_was_vowel {
            count += 1;
        }
        
        previous_was_vowel = is_vowel;
    }
    
    // Subtract 1 for silent 'e' at the end
    if word.ends_with('e') && count > 1 {
        count -= 1;
    }
    
    // Every word has at least one syllable
    count.max(1)
}

fn analyze_dialogue(text: &str) -> ((f64, usize, usize), (f64, usize, usize), (f64, usize, usize)) {
    use regex::Regex;
    
    // Count dialogue instances (text in quotes)
    let quote_pattern = Regex::new(r#""[^"]+""#).unwrap();
    let dialogue_count = quote_pattern.find_iter(text).count();
    
    if dialogue_count == 0 {
        return ((0.0, 0, 0), (0.0, 0, 0), (0.0, 0, 0));
    }
    
    // Count tags like "said", "asked", "replied"
    let tag_pattern = Regex::new(r#""\s+(said|asked|replied|answered|shouted|whispered)"#).unwrap();
    let tags = tag_pattern.find_iter(text).count();
    
    // Count unusual tags
    let unusual_pattern = Regex::new(r#""\s+(exclaimed|gushed|hissed|opined)"#).unwrap();
    let unusual = unusual_pattern.find_iter(text).count();
    
    // Count tags with adverbs
    let adverb_tag_pattern = Regex::new(r#""\s+\w+\s+\w+ly"#).unwrap();
    let adverb_tags = adverb_tag_pattern.find_iter(text).count();
    
    let tag_percentage = (tags as f64 / dialogue_count as f64) * 100.0;
    let unusual_percentage = (unusual as f64 / tags.max(1) as f64) * 100.0;
    let adverb_percentage = (adverb_tags as f64 / tags.max(1) as f64) * 100.0;
    
    (
        (tag_percentage, tags, dialogue_count),
        (unusual_percentage, unusual, tags),
        (adverb_percentage, adverb_tags, tags),
    )
}

fn extract_text(text: &str, start: usize, end: usize) -> String {
    text.chars().skip(start).take(end - start).collect()
}

fn estimate_paragraph(text: &str, position: usize) -> usize {
    text[..position.min(text.len())].matches("\n\n").count()
}

fn split_into_sentences(text: &str) -> Vec<String> {
    use regex::Regex;
    // Simple sentence splitter - matches ., !, ? followed by space or end
    let sentence_pattern = Regex::new(r"[.!?]+(?:\s+|$)").unwrap();
    
    let mut sentences = Vec::new();
    let mut last_end = 0;
    
    for mat in sentence_pattern.find_iter(text) {
        let sentence = text[last_end..mat.end()].trim().to_string();
        if !sentence.is_empty() {
            sentences.push(sentence);
        }
        last_end = mat.end();
    }
    
    // Add any remaining text
    if last_end < text.len() {
        let remaining = text[last_end..].trim().to_string();
        if !remaining.is_empty() {
            sentences.push(remaining);
        }
    }
    
    sentences
}

// Helper function to determine sentence kind based on word count
fn get_sentence_kind(word_count: usize) -> String {
    match word_count {
        0..=9 => "under10".to_string(),
        10..=19 => "range10to19".to_string(),
        20..=29 => "range20to29".to_string(),
        30..=39 => "range30to39".to_string(),
        _ => "over40".to_string(),
    }
}

fn split_sentences_with_positions(text: &str) -> Vec<(String, usize, usize)> {
    use regex::Regex;
    
    let mut sentences = Vec::new();
    
    // Convert entire text to character array - THIS IS CRITICAL FOR UNICODE
    let text_chars: Vec<char> = text.chars().collect();
    let total_chars = text_chars.len();
    
    // Match sentence endings: ., !, or ?
    let sentence_pattern = Regex::new(r"[.!?]").unwrap();
    
    let mut char_position = 0;
    
    // Find all punctuation marks
    let text_string: String = text_chars.iter().collect();
    
    for mat in sentence_pattern.find_iter(&text_string) {
        // Convert byte position to character position
        let punct_byte_pos = mat.start();
        let punct_char_pos = text[..punct_byte_pos].chars().count();
        
        // Sentence goes from char_position to after the punctuation
        let sentence_start = char_position;
        let sentence_end = punct_char_pos + 1; // Include the punctuation mark
        
        if sentence_end <= total_chars && sentence_start < sentence_end {
            // Extract sentence using CHARACTER positions
            let sentence_chars: Vec<char> = text_chars[sentence_start..sentence_end].to_vec();
            let sentence_text: String = sentence_chars.iter().collect();
            
            if !sentence_text.trim().is_empty() {
                // Calculate length using CHARACTER count
                let character_array = sentence_text.chars();
                let length = character_array.count();
                
                // Push with character-based positions
                sentences.push((
                    sentence_text,
                    sentence_start,
                    sentence_start + length
                ));
            }
        }
        
        // Move to next sentence - skip whitespace
        char_position = sentence_end;
        while char_position < total_chars && text_chars[char_position].is_whitespace() {
            char_position += 1;
        }
    }
    
    // Handle any remaining text (text without ending punctuation)
    if char_position < total_chars {
        let remaining_chars: Vec<char> = text_chars[char_position..].to_vec();
        let remaining_text: String = remaining_chars.iter().collect();
        
        if !remaining_text.trim().is_empty() {
            // Calculate length using CHARACTER count
            let character_array = remaining_text.chars();
            let length = character_array.count();
            
            sentences.push((
                remaining_text,
                char_position,
                char_position + length
            ));
        }
    }
    
    sentences
}

// Message helper functions for /score endpoint
fn get_style_score_message(score: i32) -> String {
    match score {
        90..=100 => "Excellent! Your writing style is clear, engaging, and professional.".to_string(),
        80..=89 => "Great work! Your writing is strong with minor areas for improvement.".to_string(),
        70..=79 => "Good foundation. Focus on reducing passive voice and improving sentence variety.".to_string(),
        60..=69 => "Fair writing. Work on clarity by reducing glue words and varying sentence length.".to_string(),
        _ => "Needs improvement. Focus on reducing passive voice, glue words, and improving readability.".to_string(),
    }
}

fn get_sentence_length_message(avg_length: f64) -> String {
    if avg_length >= 15.0 && avg_length <= 20.0 {
        "Perfect! Your average sentence length is in the ideal range for readability.".to_string()
    } else if avg_length < 15.0 {
        format!("Your sentences average {:.1} words. Add longer sentences (15-20 words) to create more depth and flow in your writing.", avg_length)
    } else if avg_length <= 25.0 {
        format!("Your sentences average {:.1} words. Consider shortening some sentences to improve readability.", avg_length)
    } else {
        format!("Your sentences average {:.1} words, which is quite long. Break complex sentences into shorter ones (15-20 words) for better clarity.", avg_length)
    }
}

fn get_readability_message(grade: f64) -> String {
    if grade >= 7.0 && grade <= 9.0 {
        format!("Excellent! Your writing is at grade level {:.1}, making it accessible to most readers.", grade)
    } else if grade < 7.0 {
        format!("Your writing is at grade level {:.1}. Consider using more complex sentence structures and vocabulary to add sophistication.", grade)
    } else if grade <= 12.0 {
        format!("Your writing is at grade level {:.1}. Simplify complex sentences to make your content more accessible.", grade)
    } else {
        format!("Your writing is at grade level {:.1}, which may be too complex. Use shorter sentences and simpler words to improve readability.", grade)
    }
}

fn get_variety_message(std_dev: f64) -> String {
    if std_dev >= 5.0 {
        "Excellent! You have great sentence variety, which keeps readers engaged.".to_string()
    } else if std_dev >= 3.0 {
        format!("Fair variety (σ={:.1}). Mix in more short and long sentences to create better rhythm and flow.", std_dev)
    } else {
        format!("Your sentences lack variety (σ={:.1}). Combine short punchy sentences with longer, flowing ones to maintain reader interest.", std_dev)
    }
}

fn get_glue_message(percentage: f64) -> String {
    if percentage < 40.0 {
        "Great! Your glue word usage is within the ideal range, keeping your writing clear and direct.".to_string()
    } else if percentage < 45.0 {
        format!("Your writing contains {:.1}% glue words. Aim for under 40% by replacing phrases like 'there is' with direct statements.", percentage)
    } else {
        format!("High glue word usage at {:.1}%. Reduce weak phrases (it is, there are, seems to be) to make your writing more concise and powerful.", percentage)
    }
}

fn get_passive_message(percentage: f64, count: usize) -> String {
    if percentage < 10.0 {
        "Excellent! Your active voice usage makes the writing direct and engaging.".to_string()
    } else if percentage < 20.0 {
        format!("You have {} passive voice instances ({:.1}%). Convert some to active voice (e.g., 'was written' → 'she wrote') for stronger impact.", count, percentage)
    } else {
        format!("High passive voice usage: {} instances ({:.1}%). Rewrite in active voice to make your writing more dynamic and clear.", count, percentage)
    }
}

fn get_jargon_message(count: usize) -> String {
    match count {
        0 => "Perfect! No business jargon detected. Your writing is clear and accessible.".to_string(),
        1 => "Found 1 instance of business jargon. Replace it with plain language for better clarity.".to_string(),
        2 => "Found 2 instances of business jargon. Eliminate corporate buzzwords to communicate more directly.".to_string(),
        _ => format!("Found {} instances of business jargon. Remove phrases like 'leverage', 'synergy', 'at the end of the day' and use plain language instead.", count),
    }
}

fn get_complex_paragraphs_message(count: usize) -> String {
    match count {
        0 => "Great! All your paragraphs are easy to read with well-balanced sentence lengths.".to_string(),
        1 => "1 paragraph has very complex sentences. Break long sentences into shorter ones for better readability.".to_string(),
        _ => format!("{} paragraphs are too complex. Simplify by breaking long sentences (30+ words) into multiple shorter sentences.", count),
    }
}

fn get_conjunction_starts_message(percentage: f64) -> String {
    if percentage < 10.0 {
        "Good! Your sentence variety creates natural flow without overusing conjunctions at the start.".to_string()
    } else if percentage < 15.0 {
        format!("{:.1}% of sentences start with conjunctions. Vary your sentence openings for better style.", percentage)
    } else {
        format!("{:.1}% of sentences start with conjunctions (And, But, Or). Rewrite some to begin with subjects or actions for stronger impact.", percentage)
    }
}

fn get_slow_pacing_message(percentage: f64) -> String {
    if percentage < 30.0 {
        "Excellent pacing! Your sentences maintain good rhythm and energy.".to_string()
    } else if percentage < 40.0 {
        format!("{:.1}% of sentences have slow pacing. Speed up by reducing glue words and using active voice.", percentage)
    } else {
        format!("{:.1}% of sentences are slow-paced. Tighten your prose by eliminating unnecessary words and weak verb constructions.", percentage)
    }
}

fn get_very_long_sentences_message(count: usize, percentage: f64) -> String {
    match count {
        0 => "Perfect! No overly long sentences. Your writing maintains good readability.".to_string(),
        1 => "1 sentence is very long (30+ words). Break it into 2-3 shorter sentences for clarity.".to_string(),
        2 => format!("{} sentences are very long ({:.1}%). Split complex ideas into multiple sentences to improve flow.", count, percentage),
        _ => format!("{} sentences exceed 30 words ({:.1}%). Long sentences reduce clarity—aim for 15-20 words per sentence.", count, percentage),
    }
}

fn get_emotion_tells_message(count: usize) -> String {
    match count {
        0 => "Excellent! You're showing emotions through action and description rather than telling.".to_string(),
        1..=3 => format!("Found {} emotion tell(s). Instead of 'he was angry', show it: 'his jaw clenched' or 'he slammed the door'.", count),
        _ => format!("Found {} emotion tells. Show don't tell—replace 'she felt sad' with 'tears welled in her eyes'.", count),
    }
}

fn get_ing_starts_message(count: usize, percentage: f64) -> String {
    match count {
        0 => "Great! Your sentence variety keeps the writing fresh and engaging.".to_string(),
        1..=2 => format!("{} sentence(s) start with -ing words ({:.1}%). Add variety by beginning with subjects or actions.", count, percentage),
        _ => format!("{} sentences start with -ing words ({:.1}%). Rewrite some to start differently: 'Running fast' → 'She ran fast'.", count, percentage),
    }
}

fn get_weak_adverbs_message(count: usize, percentage: f64) -> String {
    match count {
        0 => "Perfect! No weak adverbs detected. Your verbs are strong and precise.".to_string(),
        1..=3 => format!("Found {} adverb(s) ({:.1}%). Replace with stronger verbs: 'walked quickly' → 'rushed' or 'hurried'.", count, percentage),
        _ => format!("Found {} adverbs ({:.1}%). Strengthen your writing: 'said loudly' → 'shouted', 'moved carefully' → 'crept'.", count, percentage),
    }
}

fn get_dialogue_tags_message(count: usize) -> String {
    match count {
        0 => "No dialogue detected, or dialogue is well-balanced with minimal tags.".to_string(),
        1..=3 => format!("{} dialogue tags found. Consider removing some—let action and context show who's speaking.", count),
        _ => format!("{} dialogue tags detected. Remove unnecessary tags and use action beats instead: '\"Hello,\" she said' → '\"Hello.\" She waved.'", count),
    }
}

// Error handling
enum ApiError {
    EmptyText,
    AnalysisError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::EmptyText => (StatusCode::BAD_REQUEST, "Text cannot be empty".to_string()),
            ApiError::AnalysisError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = serde_json::json!({
            "error": message
        });

        (status, Json(body)).into_response()
    }
}
