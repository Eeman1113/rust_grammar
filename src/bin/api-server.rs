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
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Build router
    let app = Router::new()
        .route("/analyze", post(analyze_text))
        .layer(CorsLayer::permissive());

    // Bind to 0.0.0.0:2000
    let addr = SocketAddr::from(([0, 0, 0, 0], 2000));
    println!("🚀 Text Analyzer API running on http://{}", addr);
    println!("📝 POST to http://{}/analyze with JSON body: {{\"text\": \"your text\"}}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Request body structure
#[derive(Debug, Deserialize)]
struct AnalyzeRequest {
    text: String,
}

// Response structure matching the requested format
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

// API response
#[derive(Debug, Serialize)]
struct AnalyzeResponse {
    issues: Vec<AnalysisIssue>,
    summary: AnalysisSummary,
}

#[derive(Debug, Serialize)]
struct AnalysisSummary {
    total_issues: usize,
    word_count: usize,
    sentence_count: usize,
    style_score: i32,
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

    // Convert all issues to the requested format
    let mut issues = Vec::new();
    let mut issue_counter = 0;

    // Add passive voice issues
    for pv in passive_voice {
        issue_counter += 1;
        issues.push(AnalysisIssue {
            id: format!("{}_{}_{}_{}_passive_{}", 
                "auto", pv.start_index, pv.end_index, pv.length, 
                pv.text.replace(" ", "_")),
            start: pv.start_index,
            length: pv.length,
            end: pv.end_index,
            paragraph_key: format!("{}", estimate_paragraph(&payload.text, pv.start_index)),
            string: pv.text.clone(),
            issue_type: "PassiveVoice".to_string(),
            suggestions: Suggestions {
                recommendation: vec![
                    "Consider using active voice for clarity".to_string(),
                ],
            },
        });
    }

    // Add grammar issues
    for issue in grammar {
        issue_counter += 1;
        let issue_text = extract_text(&payload.text, issue.start_index, issue.end_index);
        issues.push(AnalysisIssue {
            id: format!("{}_{}_{}_{}_grammar_{}", 
                "auto", issue.start_index, issue.end_index, issue.length,
                format!("{:?}", issue.issue_type).to_lowercase()),
            start: issue.start_index,
            length: issue.length,
            end: issue.end_index,
            paragraph_key: format!("{}", issue.sentence_num / 3),
            string: issue_text,
            issue_type: format!("Grammar_{:?}", issue.issue_type),
            suggestions: Suggestions {
                recommendation: vec![issue.message],
            },
        });
    }

    // Add sticky sentences
    for sticky in &full_report.sticky_sentences.sticky_sentences {
        issue_counter += 1;
        issues.push(AnalysisIssue {
            id: format!("{}_{}_{}_{}_sticky_{}", 
                "auto", sticky.start_index, sticky.end_index, sticky.length,
                sticky.sentence_num),
            start: sticky.start_index,
            length: sticky.length,
            end: sticky.end_index,
            paragraph_key: format!("{}", sticky.sentence_num / 3),
            string: sticky.sentence.chars().take(50).collect::<String>() + "...",
            issue_type: "StickySentence".to_string(),
            suggestions: Suggestions {
                recommendation: vec![
                    format!("Reduce glue words ({}% glue)", sticky.glue_percentage.round()),
                    "Use more concrete, meaningful words".to_string(),
                ],
            },
        });
    }

    // Add overused words (all occurrences)
    for word in &full_report.overused_words.overused_words {
        for occ in &word.occurrences {
            issue_counter += 1;
            issues.push(AnalysisIssue {
                id: format!("{}_{}_{}_{}_overused_{}", 
                    "auto", occ.start_index, occ.end_index, occ.length,
                    word.word.replace(" ", "_")),
                start: occ.start_index,
                length: occ.length,
                end: occ.end_index,
                paragraph_key: format!("{}", estimate_paragraph(&payload.text, occ.start_index)),
                string: word.word.clone(),
                issue_type: "OverusedWord".to_string(),
                suggestions: Suggestions {
                    recommendation: vec![
                        format!("Used {} times ({:.1}% frequency)", word.count, word.frequency),
                        "Consider using synonyms".to_string(),
                    ],
                },
            });
        }
    }

    // Add repeated phrases
    for phrase in &full_report.repeated_phrases.most_repeated {
        for occ in &phrase.occurrences {
            issue_counter += 1;
            issues.push(AnalysisIssue {
                id: format!("{}_{}_{}_{}_repetition_{}", 
                    "auto", occ.start_index, occ.end_index, occ.length,
                    phrase.phrase.replace(" ", "_")),
                start: occ.start_index,
                length: occ.length,
                end: occ.end_index,
                paragraph_key: format!("{}", estimate_paragraph(&payload.text, occ.start_index)),
                string: phrase.phrase.clone(),
                issue_type: "Repetition".to_string(),
                suggestions: Suggestions {
                    recommendation: vec![
                        format!("Repeated {} times", phrase.count),
                        "Consider rephrasing for variety".to_string(),
                    ],
                },
            });
        }
    }

    // Add clichés
    for cliche in &full_report.cliches.cliches {
        for occ in &cliche.occurrences {
            issue_counter += 1;
            issues.push(AnalysisIssue {
                id: format!("{}_{}_{}_{}_cliche_{}", 
                    "auto", occ.start_index, occ.end_index, occ.length,
                    cliche.cliche.replace(" ", "_")),
                start: occ.start_index,
                length: occ.length,
                end: occ.end_index,
                paragraph_key: format!("{}", estimate_paragraph(&payload.text, occ.start_index)),
                string: cliche.cliche.clone(),
                issue_type: "Cliche".to_string(),
                suggestions: Suggestions {
                    recommendation: vec![
                        "Avoid clichés".to_string(),
                        "Use original phrasing".to_string(),
                    ],
                },
            });
        }
    }

    // Add vague words
    for vague in &full_report.diction.most_common_vague {
        for occ in &vague.occurrences {
            issue_counter += 1;
            issues.push(AnalysisIssue {
                id: format!("{}_{}_{}_{}_vague_{}", 
                    "auto", occ.start_index, occ.end_index, occ.length,
                    vague.word.replace(" ", "_")),
                start: occ.start_index,
                length: occ.length,
                end: occ.end_index,
                paragraph_key: format!("{}", estimate_paragraph(&payload.text, occ.start_index)),
                string: vague.word.clone(),
                issue_type: "VagueWord".to_string(),
                suggestions: Suggestions {
                    recommendation: vec![
                        "Be more specific".to_string(),
                        "Use concrete language".to_string(),
                    ],
                },
            });
        }
    }

    // Add business jargon
    for jargon in &full_report.business_jargon.jargon_list {
        for occ in &jargon.occurrences {
            issue_counter += 1;
            issues.push(AnalysisIssue {
                id: format!("{}_{}_{}_{}_jargon_{}", 
                    "auto", occ.start_index, occ.end_index, occ.length,
                    jargon.jargon.replace(" ", "_")),
                start: occ.start_index,
                length: occ.length,
                end: occ.end_index,
                paragraph_key: format!("{}", estimate_paragraph(&payload.text, occ.start_index)),
                string: jargon.jargon.clone(),
                issue_type: "BusinessJargon".to_string(),
                suggestions: Suggestions {
                    recommendation: vec![
                        "Avoid corporate jargon".to_string(),
                        "Use plain language".to_string(),
                    ],
                },
            });
        }
    }

    // Sort by position
    issues.sort_by_key(|i| i.start);

    let response = AnalyzeResponse {
        issues,
        summary: AnalysisSummary {
            total_issues: issue_counter,
            word_count: stats.word_count,
            sentence_count: stats.sentence_count,
            style_score: full_report.style_score,
        },
    };

    Ok(Json(response))
}

// Helper functions
fn extract_text(text: &str, start: usize, end: usize) -> String {
    text.chars()
        .skip(start)
        .take(end - start)
        .collect()
}

fn estimate_paragraph(text: &str, position: usize) -> usize {
    let before = &text[..position.min(text.len())];
    before.matches("\n\n").count()
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
