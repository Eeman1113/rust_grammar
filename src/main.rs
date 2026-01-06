use Rust_Grammar::{TextAnalyzer, Config, error::Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use tracing::{info, Level};
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "text-analyzer")]
#[command(author, version, about = "Production-ready text analysis tool", long_about = None)]
struct Cli {
    /// Input text file to analyze
    #[arg(value_name = "FILE")]
    input_file: PathBuf,

    /// Output file for report (optional)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Output format: text, json, yaml
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Configuration file (YAML or TOML)
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Document type preset: general, academic, fiction, business, technical
    #[arg(short = 't', long)]
    doc_type: Option<String>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,

    /// Show only statistics (no detailed analysis)
    #[arg(short = 'q', long)]
    quiet: bool,

    /// Show comprehensive analysis (all 19 features)
    #[arg(short = 'a', long)]
    all: bool,

    /// Generate visual HTML report with highlights
    #[arg(short = 'V', long)]
    visualize: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    init_logging(&cli);

    info!("Starting text analysis");
    info!("Input file: {}", cli.input_file.display());

    // Read input file
    let start_time = Instant::now();
    let text = read_input_file(&cli.input_file)?;
    info!("File read in {:?}", start_time.elapsed());

    // Load or create configuration
    let config = load_config(&cli)?;
    info!("Configuration loaded: {:?}", config.analysis.document_type);

    // Create analyzer
    let analyzer_start = Instant::now();
    let analyzer = TextAnalyzer::new(text, config)?;
    info!("Analyzer initialized in {:?}", analyzer_start.elapsed());

    // Run analysis
    if !cli.quiet {
        println!("🔍 Analyzing text...");
    }

    let analysis_start = Instant::now();

    // Get basic statistics
    let stats = analyzer.statistics();
    if !cli.quiet {
        println!("📊 Found {} words, {} sentences, {} paragraphs", 
            stats.word_count, stats.sentence_count, stats.paragraph_count);
    }

    // Run readability analysis
    let readability = analyzer.readability_metrics()?;
    info!("Readability analysis complete");

    // Run grammar check
    let grammar_issues = analyzer.check_grammar()?;
    info!("Found {} grammar issues", grammar_issues.len());

    // Run passive voice detection
    let passive_voice = analyzer.detect_passive_voice()?;
    info!("Found {} passive voice instances", passive_voice.len());

    let total_time = analysis_start.elapsed();
    info!("Analysis completed in {:?}", total_time);

    // Generate and output report
    if cli.quiet {
        print_statistics(&stats);
    } else if cli.visualize {
        // VISUAL HTML REPORT
        println!("🎨 Generating visual HTML report...");
        
        let full_report = if cli.all {
            Some(analyzer.generate_full_report()?)
        } else {
            None
        };
        
        let html = Rust_Grammar::HtmlVisualizer::generate(
            analyzer.text(),
            analyzer.sentences(),
            &stats,
            &readability,
            &grammar_issues,
            &passive_voice,
            full_report.as_ref(),
        );
        
        // Determine output path
        let html_path = if let Some(output) = &cli.output {
            output.with_extension("html")
        } else {
            let input_stem = cli.input_file.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("analysis");
            std::path::PathBuf::from(format!("{}-visual.html", input_stem))
        };
        
        std::fs::write(&html_path, html)?;
        println!("✅ Visual report saved to: {}", html_path.display());
        println!("   Open in your browser to see highlighted text!");
    } else if cli.all {
        // COMPREHENSIVE ANALYSIS - ALL 19 FEATURES
        println!("🔍 Running comprehensive analysis (all features)...");
        let full_report = analyzer.generate_full_report()?;
        print_comprehensive_report(&full_report);
    } else {
        match cli.format.as_str() {
            "json" => print_json_report(&stats, &readability, &grammar_issues, &passive_voice)?,
            "yaml" => print_yaml_report(&stats, &readability, &grammar_issues, &passive_voice)?,
            _ => print_text_report(&stats, &readability, &grammar_issues, &passive_voice),
        }
    }

    // Save to file if requested
    if let Some(output_path) = cli.output {
        save_report(&output_path, &cli.format, &stats, &readability)?;
        println!("\n✅ Report saved to: {}", output_path.display());
    }

    println!("\n✅ Analysis complete! (took {:.2}s)", total_time.as_secs_f64());

    Ok(())
}

fn init_logging(cli: &Cli) {
    let level = if cli.debug {
        Level::DEBUG
    } else if cli.verbose {
        Level::INFO
    } else {
        Level::WARN
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .with_ansi(!cli.no_color)
        .init();
}

fn read_input_file(path: &PathBuf) -> Result<String> {
    info!("Reading file: {}", path.display());
    
    Ok(fs::read_to_string(path)
        .map_err(|e| Rust_Grammar::error::AnalysisError::IoError(e))?)
}

fn load_config(cli: &Cli) -> Result<Config> {
    // Load from config file if provided
    if let Some(config_path) = &cli.config {
        info!("Loading configuration from: {}", config_path.display());
        
        if config_path.extension().and_then(|s| s.to_str()) == Some("yaml") || 
           config_path.extension().and_then(|s| s.to_str()) == Some("yml") {
            return Config::from_yaml(config_path);
        } else {
            return Config::from_toml(config_path);
        }
    }

    // Use preset if document type specified
    if let Some(doc_type) = &cli.doc_type {
        let doc_type = match doc_type.to_lowercase().as_str() {
            "academic" => Rust_Grammar::config::DocumentType::Academic,
            "fiction" => Rust_Grammar::config::DocumentType::Fiction,
            "business" => Rust_Grammar::config::DocumentType::Business,
            "technical" => Rust_Grammar::config::DocumentType::Technical,
            _ => Rust_Grammar::config::DocumentType::General,
        };
        return Ok(Config::preset(doc_type));
    }

    // Default config
    Ok(Config::default())
}

fn print_statistics(stats: &Rust_Grammar::TextStatistics) {
    println!("Words: {}", stats.word_count);
    println!("Sentences: {}", stats.sentence_count);
    println!("Paragraphs: {}", stats.paragraph_count);
    println!("Characters: {}", stats.character_count);
}

fn print_text_report(
    stats: &Rust_Grammar::TextStatistics,
    readability: &Rust_Grammar::ReadabilityMetrics,
    grammar_issues: &[Rust_Grammar::grammar::GrammarIssue],
    passive_voice: &[Rust_Grammar::grammar::PassiveVoiceMatch],
) {
    println!("\n{}", "=".repeat(80));
    println!("TEXT ANALYSIS REPORT");
    println!("{}", "=".repeat(80));

    println!("\n📊 STATISTICS");
    println!("{}", "-".repeat(80));
    print_statistics(stats);

    println!("\n📖 READABILITY");
    println!("{}", "-".repeat(80));
    println!("Flesch Reading Ease: {:.1} (0-100, higher is easier)", readability.flesch_reading_ease);
    println!("Flesch-Kincaid Grade Level: {:.1}", readability.flesch_kincaid_grade);
    if let Some(smog) = readability.smog_index {
        println!("SMOG Index: {:.1}", smog);
    }
    println!("Avg Words/Sentence: {:.1}", readability.avg_words_per_sentence);
    println!("Avg Syllables/Word: {:.2}", readability.avg_syllables_per_word);

    println!("\n📝 GRAMMAR ISSUES: {}", grammar_issues.len());
    println!("{}", "-".repeat(80));
    if grammar_issues.is_empty() {
        println!("✅ No grammar issues detected!");
    } else {
        for issue in grammar_issues.iter().take(10) {
            println!("• Sentence {}: {} ({:?})", 
                issue.sentence_num, issue.message, issue.severity);
        }
        if grammar_issues.len() > 10 {
            println!("... and {} more issues", grammar_issues.len() - 10);
        }
    }

    println!("\n✍️  PASSIVE VOICE: {}", passive_voice.len());
    println!("{}", "-".repeat(80));
    if passive_voice.is_empty() {
        println!("✅ No passive voice detected!");
    } else {
        for pv in passive_voice.iter().take(10) {
            println!("• \"{}\" (confidence: {:.0}%)", pv.text, pv.confidence * 100.0);
        }
        if passive_voice.len() > 10 {
            println!("... and {} more instances", passive_voice.len() - 10);
        }
    }

    println!("\n{}", "=".repeat(80));
}

fn print_json_report(
    stats: &Rust_Grammar::TextStatistics,
    readability: &Rust_Grammar::ReadabilityMetrics,
    grammar_issues: &[Rust_Grammar::grammar::GrammarIssue],
    passive_voice: &[Rust_Grammar::grammar::PassiveVoiceMatch],
) -> Result<()> {
    #[derive(serde::Serialize)]
    struct Report<'a> {
        statistics: &'a Rust_Grammar::TextStatistics,
        readability: &'a Rust_Grammar::ReadabilityMetrics,
        grammar_issues_count: usize,
        passive_voice_count: usize,
    }

    let report = Report {
        statistics: stats,
        readability,
        grammar_issues_count: grammar_issues.len(),
        passive_voice_count: passive_voice.len(),
    };

    let json = serde_json::to_string_pretty(&report)?;
    println!("{}", json);
    Ok(())
}

fn print_yaml_report(
    stats: &Rust_Grammar::TextStatistics,
    readability: &Rust_Grammar::ReadabilityMetrics,
    grammar_issues: &[Rust_Grammar::grammar::GrammarIssue],
    passive_voice: &[Rust_Grammar::grammar::PassiveVoiceMatch],
) -> Result<()> {
    #[derive(serde::Serialize)]
    struct Report<'a> {
        statistics: &'a Rust_Grammar::TextStatistics,
        readability: &'a Rust_Grammar::ReadabilityMetrics,
        grammar_issues_count: usize,
        passive_voice_count: usize,
    }

    let report = Report {
        statistics: stats,
        readability,
        grammar_issues_count: grammar_issues.len(),
        passive_voice_count: passive_voice.len(),
    };

    let yaml = serde_yaml::to_string(&report)?;
    println!("{}", yaml);
    Ok(())
}

fn print_comprehensive_report(report: &Rust_Grammar::FullAnalysisReport) {
    println!("\n{}", "=".repeat(80));
    println!("COMPREHENSIVE TEXT ANALYSIS REPORT - ALL FEATURES");
    println!("{}", "=".repeat(80));

    // Overall Metrics
    println!("\n📊 OVERALL METRICS");
    println!("{}", "-".repeat(80));
    println!("Total Words: {}", report.word_count);
    println!("Total Sentences: {}", report.sentence_count);
    println!("Total Paragraphs: {}", report.paragraph_count);
    println!("Overall Style Score: {}% / 100%", report.style_score);

    // Style Report
    println!("\n✍️  STYLE REPORT");
    println!("{}", "-".repeat(80));
    println!("Passive Voice Count: {}", report.style.passive_voice_count);
    println!("Adverb Count (-ly words): {}", report.style.adverb_count);
    println!("Hidden Verbs Found: {}", report.style.hidden_verbs.len());
    if !report.style.hidden_verbs.is_empty() {
        println!("\nHidden Verbs:");
        for hv in report.style.hidden_verbs.iter().take(10) {
            println!("  • {}", hv);
        }
        if report.style.hidden_verbs.len() > 10 {
            println!("  ... and {} more", report.style.hidden_verbs.len() - 10);
        }
    }

    // Sticky Sentences
    println!("\n🔗 STICKY SENTENCES REPORT");
    println!("{}", "-".repeat(80));
    println!("Overall Glue Index: {}%", report.sticky_sentences.overall_glue_index);
    println!("Sticky Sentences: {}", report.sticky_sentences.sticky_sentence_count);
    if !report.sticky_sentences.sticky_sentences.is_empty() {
        println!("\nStickiest Sentences:");
        for ss in report.sticky_sentences.sticky_sentences.iter().take(5) {
            println!("  • Sentence {}: {}% glue words", ss.sentence_num, ss.glue_percentage);
            println!("    \"{}\"", ss.sentence);
        }
        if report.sticky_sentences.sticky_sentences.len() > 5 {
            println!("  ... and {} more", report.sticky_sentences.sticky_sentences.len() - 5);
        }
    }

    // Pacing
    println!("\n⚡ PACING REPORT");
    println!("{}", "-".repeat(80));
    println!("Fast-Paced (<10 words): {}%", report.pacing.fast_paced_percentage);
    println!("Medium-Paced (10-20 words): {}%", report.pacing.medium_paced_percentage);
    println!("Slow-Paced (>20 words): {}%", report.pacing.slow_paced_percentage);
    println!("Distribution: {} fast, {} medium, {} slow",
        report.pacing.pacing_distribution.fast,
        report.pacing.pacing_distribution.medium,
        report.pacing.pacing_distribution.slow
    );

    // Sentence Length
    println!("\n📏 SENTENCE LENGTH REPORT");
    println!("{}", "-".repeat(80));
    println!("Average Length: {} words", report.sentence_length.avg_length);
    println!("Variety Score: {}/10", report.sentence_length.variety_score);
    println!("Shortest: {} words | Longest: {} words",
        report.sentence_length.shortest, report.sentence_length.longest);
    println!("Very Long Sentences (>30 words): {}", report.sentence_length.very_long_sentences);

    // Transitions
    println!("\n🔄 TRANSITION REPORT");
    println!("{}", "-".repeat(80));
    println!("Sentences with Transitions: {}", report.transitions.sentences_with_transitions);
    println!("Transition Percentage: {}%", report.transitions.transition_percentage);
    println!("Unique Transitions Used: {}", report.transitions.unique_transitions);
    if !report.transitions.most_common_transitions.is_empty() {
        println!("\nMost Common Transitions:");
        for (word, count) in report.transitions.most_common_transitions.iter().take(10) {
            println!("  • {}: {} times", word, count);
        }
    }

    // Overused Words
    println!("\n🔁 OVERUSED WORDS REPORT");
    println!("{}", "-".repeat(80));
    println!("Total Unique Words: {}", report.overused_words.total_unique_words);
    if !report.overused_words.overused_words.is_empty() {
        println!("\nOverused Words (>0.5% frequency):");
        for ow in report.overused_words.overused_words.iter().take(10) {
            println!("  • '{}': {} times ({}%)", ow.word, ow.count, ow.frequency);
        }
        if report.overused_words.overused_words.len() > 10 {
            println!("  ... and {} more", report.overused_words.overused_words.len() - 10);
        }
    }

    // Repeated Phrases
    println!("\n🔁 REPEATED PHRASES REPORT");
    println!("{}", "-".repeat(80));
    println!("Total Repeated Phrases: {}", report.repeated_phrases.total_repeated_phrases);
    if !report.repeated_phrases.most_repeated.is_empty() {
        println!("\nMost Repeated Phrases:");
        for phrase in report.repeated_phrases.most_repeated.iter().take(15) {
            println!("  • \"{}\": {} times", phrase.phrase, phrase.count);
        }
        if report.repeated_phrases.most_repeated.len() > 15 {
            println!("  ... and {} more", report.repeated_phrases.most_repeated.len() - 15);
        }
    }

    // Echoes
    println!("\n🔊 ECHOES REPORT");
    println!("{}", "-".repeat(80));
    println!("Total Echoes Found: {}", report.echoes.total_echoes);
    if !report.echoes.echoes.is_empty() {
        println!("\nClosest Echoes:");
        for echo in report.echoes.echoes.iter().take(10) {
            println!("  • '{}' in paragraph {}: {} times, {} words apart",
                echo.word, echo.paragraph, echo.occurrences, echo.distance);
        }
        if report.echoes.echoes.len() > 10 {
            println!("  ... and {} more", report.echoes.echoes.len() - 10);
        }
    }

    // Sensory
    println!("\n👁️ 👂 ✋ 👃 👅 SENSORY REPORT");
    println!("{}", "-".repeat(80));
    println!("Total Sensory Words: {} ({}%)",
        report.sensory.sensory_word_count, report.sensory.sensory_percentage);
    println!("\nBy Sense:");
    let mut senses: Vec<_> = report.sensory.by_sense.iter().collect();
    senses.sort_by(|a, b| b.1.count.cmp(&a.1.count));
    for (sense, data) in senses {
        println!("  • {}: {} words ({}% of sensory), {} unique",
            sense, data.count, data.percentage, data.unique_words);
    }

    // Diction
    println!("\n💭 DICTION REPORT (Vague Words)");
    println!("{}", "-".repeat(80));
    println!("Total Vague Words: {}", report.diction.total_vague_words);
    println!("Unique Vague Words: {}", report.diction.unique_vague_words);
    if !report.diction.most_common_vague.is_empty() {
        println!("\nMost Common Vague Words:");
        for vague in report.diction.most_common_vague.iter().take(10) {
            println!("  • '{}': {} times", vague.word, vague.count);
        }
    }

    // Clichés
    println!("\n🎭 CLICHÉS REPORT");
    println!("{}", "-".repeat(80));
    println!("Total Clichés Found: {}", report.cliches.total_cliches);
    if !report.cliches.cliches.is_empty() {
        println!("\nClichés:");
        for cliche in &report.cliches.cliches {
            println!("  • \"{}\": {} time(s)", cliche.cliche, cliche.count);
        }
    } else {
        println!("✅ No clichés detected!");
    }

    // Consistency
    println!("\n✅ CONSISTENCY REPORT");
    println!("{}", "-".repeat(80));
    println!("Total Issues: {}", report.consistency.total_issues);
    if !report.consistency.issues.is_empty() {
        println!("\nInconsistencies Found:");
        for issue in report.consistency.issues.iter().take(10) {
            println!("  • {}", issue);
        }
        if report.consistency.issues.len() > 10 {
            println!("  ... and {} more", report.consistency.issues.len() - 10);
        }
    } else {
        println!("✅ No consistency issues detected!");
    }

    // Acronyms
    println!("\n🔤 ACRONYM REPORT");
    println!("{}", "-".repeat(80));
    println!("Total Acronyms: {}", report.acronyms.total_acronyms);
    println!("Unique Acronyms: {}", report.acronyms.unique_acronyms);
    if !report.acronyms.acronym_list.is_empty() {
        println!("\nAcronyms Found:");
        for (acronym, count) in report.acronyms.acronym_list.iter().take(10) {
            println!("  • {}: {} times", acronym, count);
        }
    }

    // Conjunction Starts
    println!("\n🔗 CONJUNCTION STARTS REPORT");
    println!("{}", "-".repeat(80));
    println!("Sentences Starting with Conjunctions: {} ({}%)",
        report.conjunction_starts.count, report.conjunction_starts.percentage);

    // Business Jargon
    println!("\n💼 BUSINESS JARGON REPORT");
    println!("{}", "-".repeat(80));
    println!("Total Jargon Instances: {}", report.business_jargon.total_jargon);
    println!("Unique Jargon Phrases: {}", report.business_jargon.unique_jargon_phrases);
    if !report.business_jargon.jargon_list.is_empty() {
        println!("\nJargon Found:");
        for j in &report.business_jargon.jargon_list {
            println!("  • \"{}\": {} time(s)", j.jargon, j.count);
        }
    } else {
        println!("✅ No business jargon detected!");
    }

    // Complex Paragraphs
    println!("\n🧩 COMPLEX PARAGRAPHS REPORT");
    println!("{}", "-".repeat(80));
    println!("Complex Paragraphs: {} ({}%)",
        report.complex_paragraphs.complex_paragraph_count, report.complex_paragraphs.percentage);
    if !report.complex_paragraphs.complex_paragraphs.is_empty() {
        println!("\nComplex Paragraphs:");
        for cp in &report.complex_paragraphs.complex_paragraphs {
            println!("  • Paragraph {}: Avg {} words/sentence, {} syllables/word",
                cp.paragraph_num, cp.avg_sentence_length, cp.avg_syllables);
        }
    }

    println!("\n{}", "=".repeat(80));
    println!("END OF COMPREHENSIVE REPORT");
    println!("{}\n", "=".repeat(80));
}

fn save_report(
    path: &PathBuf,
    format: &str,
    stats: &Rust_Grammar::TextStatistics,
    readability: &Rust_Grammar::ReadabilityMetrics,
) -> Result<()> {
    #[derive(serde::Serialize, Debug)]
    struct Report<'a> {
        statistics: &'a Rust_Grammar::TextStatistics,
        readability: &'a Rust_Grammar::ReadabilityMetrics,
    }

    let report = Report {
        statistics: stats,
        readability,
    };

    let content = match format {
        "json" => serde_json::to_string_pretty(&report)?,
        "yaml" => serde_yaml::to_string(&report)?,
        _ => format!("{:?}", report),
    };

    fs::write(path, content)?;
    Ok(())
}
