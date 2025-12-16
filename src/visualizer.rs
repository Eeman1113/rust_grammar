use crate::analysis_reports::*;
use crate::grammar::{GrammarIssue, PassiveVoiceMatch};
use crate::{TextStatistics, ReadabilityMetrics};
use std::collections::HashMap;

/// HTML Visualizer - Highlights text issues with colors
pub struct HtmlVisualizer {
}

impl HtmlVisualizer {
    pub fn new() -> Self {
        Self { }
    }

    /// Generate complete HTML visualization
    pub fn generate(
        text: &str,
        sentences: &[String],
        stats: &TextStatistics,
        readability: &ReadabilityMetrics,
        grammar: &[GrammarIssue],
        passive_voice: &[PassiveVoiceMatch],
        full_report: Option<&FullAnalysisReport>,
    ) -> String {
        let mut viz = Self::new();
        
        // Build highlight map
        let mut highlights = HashMap::new();
        
        // Add passive voice highlights
        for pv in passive_voice {
            highlights
                .entry(pv.position)
                .or_insert_with(Vec::new)
                .push(HighlightType::PassiveVoice(pv.text.clone()));
        }
        
        // Add grammar issue highlights
        for issue in grammar {
            highlights
                .entry(issue.sentence_num)
                .or_insert_with(Vec::new)
                .push(HighlightType::Grammar(format!("{:?}", issue.issue_type)));
        }
        
        // Add comprehensive highlights if available
        if let Some(report) = full_report {
            // Sticky sentences
            for sticky in &report.sticky_sentences.sticky_sentences {
                highlights
                    .entry(sticky.sentence_num)
                    .or_insert_with(Vec::new)
                    .push(HighlightType::Sticky(sticky.glue_percentage));
            }
            
            // Adverbs (approximate - we'll highlight -ly words)
            // Vague words
            // Clichés
            // etc.
        }
        
        viz.build_html(text, sentences, stats, readability, highlights, full_report)
    }
    
    fn build_html(
        &mut self,
        _text: &str,
        sentences: &[String],
        stats: &TextStatistics,
        readability: &ReadabilityMetrics,
        highlights: HashMap<usize, Vec<HighlightType>>,
        full_report: Option<&FullAnalysisReport>,
    ) -> String {
        let mut html = String::new();
        
        // HTML Header
        html.push_str(&self.html_header());
        
        // Summary Panel
        html.push_str(&self.summary_panel(stats, readability, full_report));
        
        // Text with highlights
        html.push_str("<div class='text-container'>\n");
        html.push_str("<h2>📝 Analyzed Text with Highlights</h2>\n");
        
        // Render each sentence with highlights
        for (i, sentence) in sentences.iter().enumerate() {
            let sentence_num = i + 1;
            let mut sentence_html = format!("<span class='sentence' data-sentence='{}'>\n", sentence_num);
            
            if let Some(highlight_types) = highlights.get(&sentence_num) {
                let classes = highlight_types.iter()
                    .map(|ht| ht.css_class())
                    .collect::<Vec<_>>()
                    .join(" ");
                
                sentence_html = format!(
                    "<span class='sentence {}' data-sentence='{}' title='{}'>\n",
                    classes,
                    sentence_num,
                    highlight_types.iter()
                        .map(|ht| ht.tooltip())
                        .collect::<Vec<_>>()
                        .join(" | ")
                );
            }
            
            // Highlight individual words
            sentence_html.push_str(&self.highlight_words(sentence, sentence_num, full_report));
            sentence_html.push_str("</span> ");
            
            html.push_str(&sentence_html);
        }
        
        html.push_str("</div>\n");
        
        // Legend
        html.push_str(&self.legend());
        
        // HTML Footer
        html.push_str(&self.html_footer());
        
        html
    }
    
    fn highlight_words(&self, sentence: &str, _sentence_num: usize, full_report: Option<&FullAnalysisReport>) -> String {
        use regex::Regex;
        
        let mut result = sentence.to_string();
        
        if let Some(report) = full_report {
            // Highlight adverbs (-ly words)
            let adverb_regex = Regex::new(r"\b(\w+ly)\b").unwrap();
            result = adverb_regex.replace_all(&result, "<span class='adverb' title='Adverb'>$1</span>").to_string();
            
            // Highlight vague words
            for vague in &report.diction.most_common_vague {
                if vague.count > 0 {
                    let word_regex = Regex::new(&format!(r"\b({})\b", regex::escape(&vague.word))).unwrap();
                    result = word_regex.replace_all(
                        &result,
                        &format!("<span class='vague-word' title='Vague word: {}'>$1</span>", vague.word)
                    ).to_string();
                }
            }
        }
        
        result
    }
    
    fn html_header(&self) -> String {
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Text Analysis Visualization</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            line-height: 1.6;
            color: #333;
            background: #f5f5f5;
            padding: 20px;
        }
        
        .container {
            max-width: 1400px;
            margin: 0 auto;
            background: white;
            padding: 30px;
            border-radius: 10px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        
        h1 {
            color: #2c3e50;
            margin-bottom: 20px;
            font-size: 2.5em;
            text-align: center;
        }
        
        .summary {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
            padding: 20px;
            background: #f8f9fa;
            border-radius: 8px;
        }
        
        .summary-card {
            background: white;
            padding: 15px;
            border-radius: 8px;
            border-left: 4px solid #3498db;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        
        .summary-card h3 {
            font-size: 0.9em;
            color: #7f8c8d;
            text-transform: uppercase;
            margin-bottom: 5px;
        }
        
        .summary-card .value {
            font-size: 2em;
            font-weight: bold;
            color: #2c3e50;
        }
        
        .text-container {
            background: white;
            padding: 30px;
            border-radius: 8px;
            line-height: 2;
            font-size: 1.1em;
            margin-bottom: 30px;
            border: 1px solid #e0e0e0;
        }
        
        .text-container h2 {
            margin-bottom: 20px;
            color: #2c3e50;
        }
        
        .sentence {
            transition: all 0.2s ease;
            cursor: pointer;
            padding: 2px 0;
        }
        
        .sentence:hover {
            background: rgba(52, 152, 219, 0.1);
            border-radius: 3px;
        }
        
        /* Highlight Colors */
        .passive-voice {
            background: rgba(231, 76, 60, 0.2);
            border-bottom: 2px solid #e74c3c;
            padding: 2px 4px;
            border-radius: 3px;
        }
        
        .grammar-issue {
            background: rgba(241, 196, 15, 0.2);
            border-bottom: 2px wavy #f39c12;
            padding: 2px 4px;
            border-radius: 3px;
        }
        
        .sticky-sentence {
            background: rgba(155, 89, 182, 0.2);
            border-bottom: 2px solid #9b59b6;
            padding: 2px 4px;
            border-radius: 3px;
        }
        
        .adverb {
            background: rgba(46, 204, 113, 0.2);
            padding: 1px 3px;
            border-radius: 2px;
            font-weight: 600;
        }
        
        .vague-word {
            background: rgba(230, 126, 34, 0.3);
            padding: 1px 3px;
            border-radius: 2px;
            text-decoration: underline wavy rgba(230, 126, 34, 0.6);
        }
        
        .long-sentence {
            background: rgba(52, 152, 219, 0.15);
            border-left: 4px solid #3498db;
            padding-left: 8px;
        }
        
        .legend {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 15px;
            padding: 20px;
            background: #f8f9fa;
            border-radius: 8px;
            margin-top: 30px;
        }
        
        .legend h3 {
            grid-column: 1 / -1;
            color: #2c3e50;
            margin-bottom: 10px;
        }
        
        .legend-item {
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 8px;
            background: white;
            border-radius: 5px;
        }
        
        .legend-color {
            width: 30px;
            height: 20px;
            border-radius: 3px;
            flex-shrink: 0;
        }
        
        .legend-text {
            font-size: 0.9em;
            color: #555;
        }
        
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
            gap: 10px;
            margin-top: 10px;
        }
        
        .stat-item {
            background: #ecf0f1;
            padding: 10px;
            border-radius: 5px;
            text-align: center;
        }
        
        .stat-label {
            font-size: 0.8em;
            color: #7f8c8d;
            text-transform: uppercase;
        }
        
        .stat-value {
            font-size: 1.3em;
            font-weight: bold;
            color: #2c3e50;
            margin-top: 5px;
        }
        
        @media print {
            body {
                background: white;
            }
            .container {
                box-shadow: none;
            }
        }
        
        .tooltip {
            position: relative;
            display: inline-block;
        }
        
        .tooltip .tooltiptext {
            visibility: hidden;
            width: 200px;
            background-color: #555;
            color: #fff;
            text-align: center;
            border-radius: 6px;
            padding: 5px;
            position: absolute;
            z-index: 1;
            bottom: 125%;
            left: 50%;
            margin-left: -100px;
            opacity: 0;
            transition: opacity 0.3s;
        }
        
        .tooltip:hover .tooltiptext {
            visibility: visible;
            opacity: 1;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>📊 Text Analysis Visualization</h1>
"#.to_string()
    }
    
    fn summary_panel(&self, stats: &TextStatistics, readability: &ReadabilityMetrics, full_report: Option<&FullAnalysisReport>) -> String {
        let mut html = String::from("<div class='summary'>\n");
        
        // Basic stats
        html.push_str(&format!(r#"
            <div class="summary-card">
                <h3>Words</h3>
                <div class="value">{}</div>
            </div>
            <div class="summary-card">
                <h3>Sentences</h3>
                <div class="value">{}</div>
            </div>
            <div class="summary-card">
                <h3>Paragraphs</h3>
                <div class="value">{}</div>
            </div>
            <div class="summary-card">
                <h3>Readability</h3>
                <div class="value">{:.1}</div>
            </div>
        "#, stats.word_count, stats.sentence_count, stats.paragraph_count, readability.flesch_reading_ease));
        
        if let Some(report) = full_report {
            html.push_str(&format!(r#"
                <div class="summary-card" style="border-left-color: #e74c3c;">
                    <h3>Passive Voice</h3>
                    <div class="value">{}</div>
                </div>
                <div class="summary-card" style="border-left-color: #9b59b6;">
                    <h3>Sticky Sentences</h3>
                    <div class="value">{}</div>
                </div>
                <div class="summary-card" style="border-left-color: #2ecc71;">
                    <h3>Adverbs</h3>
                    <div class="value">{}</div>
                </div>
                <div class="summary-card" style="border-left-color: #f39c12;">
                    <h3>Style Score</h3>
                    <div class="value">{}%</div>
                </div>
            "#, report.style.passive_voice_count, report.sticky_sentences.sticky_sentence_count, 
                report.style.adverb_count, report.style_score));
        }
        
        html.push_str("</div>\n");
        html
    }
    
    fn legend(&self) -> String {
        r#"
        <div class="legend">
            <h3>🎨 Highlight Legend</h3>
            <div class="legend-item">
                <div class="legend-color" style="background: rgba(231, 76, 60, 0.4);"></div>
                <div class="legend-text"><strong>Passive Voice</strong> - Consider active voice</div>
            </div>
            <div class="legend-item">
                <div class="legend-color" style="background: rgba(241, 196, 15, 0.4);"></div>
                <div class="legend-text"><strong>Grammar Issue</strong> - Check grammar</div>
            </div>
            <div class="legend-item">
                <div class="legend-color" style="background: rgba(155, 89, 182, 0.4);"></div>
                <div class="legend-text"><strong>Sticky Sentence</strong> - Too many glue words</div>
            </div>
            <div class="legend-item">
                <div class="legend-color" style="background: rgba(46, 204, 113, 0.4);"></div>
                <div class="legend-text"><strong>Adverb</strong> - -ly word (use sparingly)</div>
            </div>
            <div class="legend-item">
                <div class="legend-color" style="background: rgba(230, 126, 34, 0.4);"></div>
                <div class="legend-text"><strong>Vague Word</strong> - Be more specific</div>
            </div>
            <div class="legend-item">
                <div class="legend-color" style="background: rgba(52, 152, 219, 0.3);"></div>
                <div class="legend-text"><strong>Long Sentence</strong> - Consider breaking up</div>
            </div>
        </div>
        "#.to_string()
    }
    
    fn html_footer(&self) -> String {
        r#"
    </div>
    <script>
        // Add interactivity
        document.querySelectorAll('.sentence').forEach(sentence => {
            sentence.addEventListener('click', function() {
                const sentenceNum = this.getAttribute('data-sentence');
                alert('Sentence #' + sentenceNum + '\n\n' + this.textContent.trim());
            });
        });
    </script>
</body>
</html>
"#.to_string()
    }
}

#[derive(Debug, Clone)]
enum HighlightType {
    PassiveVoice(String),
    Grammar(String),
    Sticky(f64),
}

impl HighlightType {
    fn css_class(&self) -> &str {
        match self {
            HighlightType::PassiveVoice(_) => "passive-voice",
            HighlightType::Grammar(_) => "grammar-issue",
            HighlightType::Sticky(_) => "sticky-sentence",
        }
    }
    
    fn tooltip(&self) -> String {
        match self {
            HighlightType::PassiveVoice(text) => format!("Passive voice: {}", text),
            HighlightType::Grammar(issue) => format!("Grammar: {}", issue),
            HighlightType::Sticky(pct) => format!("Sticky sentence: {:.1}% glue words", pct),
        }
    }
}
