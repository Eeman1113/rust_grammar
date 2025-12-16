use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use text_analyzer::{Config, TextAnalyzer};
use text_analyzer::grammar::{PassiveVoiceMatch, GrammarIssue};
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
        .layer(CorsLayer::permissive());

    // Bind to 0.0.0.0:2000
    let addr = SocketAddr::from(([0, 0, 0, 0], 2000));
    println!("🚀 Text Analyzer API running on http://{}", addr);
    println!("📝 POST to http://{}/analyze with JSON body: {{\"text\": \"your text\"}}", addr);
    println!("📊 POST to http://{}/score for scores only", addr);
    println!("📏 POST to http://{}/sentencelength for sentence length analysis", addr);

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
    let num_characters = combined_text.len();

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
        let paragraph_sentences = split_into_sentences(&paragraph.text);
        let mut current_pos = 0;

        for sentence_text in paragraph_sentences {
            // Find the sentence in the paragraph text
            if let Some(start) = paragraph.text[current_pos..].find(sentence_text.trim()) {
                let actual_start = current_pos + start;
                let end = actual_start + sentence_text.len();
                let word_count = sentence_text.split_whitespace().count();

                // Truncate string to 50 characters with "..."
                let display_string = if sentence_text.len() > 50 {
                    format!("{}...", &sentence_text[..50])
                } else {
                    sentence_text.clone()
                };

                individual_sentence_lengths.push(IndividualSentence {
                    start: actual_start,
                    end,
                    length: end - actual_start,
                    string: display_string,
                    word_count,
                    paragraph_key: paragraph.key.clone(),
                });

                current_pos = end;
            }
        }
    }

    let response = SentenceLengthResponse {
        score,
        percentage,
        message,
        num_words,
        num_characters,
        avg_sentence_length,
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

// Create user-friendly scores with ideal values and quality messages
fn create_user_friendly_scores(
    full_report: &text_analyzer::FullAnalysisReport,
    passive_voice: &[PassiveVoiceMatch],
    _grammar: &[GrammarIssue],
    stats: &text_analyzer::TextStatistics,
    readability: &text_analyzer::ReadabilityMetrics,
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
            current: ((emotion_tells as f64) * 100.0).round() / 100.0,
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
    full_report: &text_analyzer::FullAnalysisReport,
    passive_voice: &[PassiveVoiceMatch],
    _grammar: &[GrammarIssue],
    stats: &text_analyzer::TextStatistics,
    readability: &text_analyzer::ReadabilityMetrics,
    text: &str,
) -> ComprehensiveScores {
    let sentence_count = stats.sentence_count.max(1) as f64;
    let word_count = stats.word_count.max(1) as f64;
    
    // Calculate individual sentence details with positions
    let sentences = split_into_sentences(text);
    let mut individual_lengths = Vec::new();
    let mut current_pos = 0;
    
    for sentence in &sentences {
        // Find sentence in text
        if let Some(pos) = text[current_pos..].find(sentence.trim()) {
            let start = current_pos + pos;
            let end = start + sentence.trim().len();
            let word_count_val = sentence.split_whitespace().count();
            
            individual_lengths.push(SentenceOccurrence {
                start,
                end,
                length: end - start,
                string: if sentence.len() > 50 {
                    sentence.chars().take(50).collect::<String>() + "..."
                } else {
                    sentence.clone()
                },
                word_count: word_count_val,
                paragraph_key: format!("{}", estimate_paragraph(text, start)),
            });
            
            current_pos = end;
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
    full_report: &text_analyzer::FullAnalysisReport,
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

fn calculate_sentence_length_score(report: &text_analyzer::SentenceLengthReport) -> i32 {
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

fn calculate_variety_score(report: &text_analyzer::SentenceLengthReport) -> i32 {
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
    
    for (i, ch) in chars.iter().enumerate() {
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
