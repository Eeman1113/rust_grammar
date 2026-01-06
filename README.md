# Rust_Grammar

## Complete Professional Text Analysis Library & API

[![Crates.io](https://img.shields.io/crates/v/Rust_Grammar.svg)](https://crates.io/crates/Rust_Grammar)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)

**Rust_Grammar** is a production-grade, comprehensive text analysis library written in Rust that provides detailed analysis of grammar, readability, style, and 19+ professional writing metrics. Built for maximum performance, reliability, and accuracy with zero-crash guarantees.

---

## Table of Contents

1. [Overview](#overview)
2. [Features](#features)
3. [Architecture](#architecture)
4. [Installation](#installation)
5. [Configuration](#configuration)
6. [Usage](#usage)
7. [REST API Reference](#rest-api-reference)
8. [Code Examples](#code-examples)
9. [Data Models](#data-models)
10. [Error Handling](#error-handling)
11. [Testing](#testing)
12. [Deployment](#deployment)
13. [Contributing](#contributing)
14. [Changelog & Roadmap](#changelog--roadmap)
15. [FAQ](#faq)
16. [License & Credits](#license--credits)

---

## Overview

### Project Description

Rust_Grammar is a comprehensive text analysis toolkit designed for writers, editors, developers, and content creators who need detailed insights into their writing. Whether you're analyzing academic papers, fiction manuscripts, business documents, or technical documentation, Rust_Grammar provides actionable feedback on grammar, style, readability, and writing quality.

### Key Value Propositions

| Value | Description |
|-------|-------------|
| **Performance** | Written in Rust for blazing-fast analysis (~500ms per 1000 words) |
| **Accuracy** | 95%+ sentence splitting, 85%+ passive voice detection, 90%+ syllable counting |
| **Reliability** | Zero crashes with comprehensive error handling using `Result<T, E>` |
| **Completeness** | 19+ professional analysis features in a single library |
| **Flexibility** | CLI, Library, and REST API interfaces |
| **Extensibility** | Modular architecture with configurable presets |

### Target Audience

- **Writers & Authors**: Improve prose quality and readability
- **Editors**: Automated first-pass analysis
- **Developers**: Integrate text analysis into applications
- **Educators**: Teaching writing improvement
- **Content Teams**: Maintain consistent writing standards

---

## Features

### Complete Feature List (19+ Professional Features)

<details open>
<summary><strong>📝 Grammar Analysis</strong></summary>

| Feature | Description |
|---------|-------------|
| Subject-Verb Agreement | Detects mismatched subjects and verbs (e.g., "He are going") |
| Double Negative Detection | Finds double negatives (e.g., "don't have nothing") |
| Run-on Sentence Detection | Identifies excessively long compound sentences |
| Comma Splice Detection | Finds improperly joined independent clauses |
| Missing Punctuation | Detects sentences lacking end punctuation |
| Severity Levels | Issues categorized as Low, Medium, or High severity |

</details>

<details open>
<summary><strong>✍️ Style Analysis</strong></summary>

| Feature | Description |
|---------|-------------|
| Passive Voice Detection | 85%+ accuracy with confidence scoring (0.0-1.0) |
| Adverb Counting | Counts `-ly` words throughout the text |
| Hidden Verbs | Finds nominalizations (e.g., "decision" → "decide") |
| Overall Style Score | 0-100% rating based on multiple factors |

</details>

<details open>
<summary><strong>📊 Readability Metrics</strong></summary>

| Metric | Description |
|--------|-------------|
| Flesch Reading Ease | 0-100 scale (higher = easier to read) |
| Flesch-Kincaid Grade Level | U.S. school grade level equivalent |
| SMOG Index | Readability formula for healthcare documents |
| Coleman-Liau Index | Character-based readability |
| Automated Readability Index | Character and word count based |
| Average Words per Sentence | Sentence complexity indicator |
| Average Syllables per Word | Vocabulary complexity indicator |

</details>

<details open>
<summary><strong>🔗 Sticky Sentences</strong></summary>

| Feature | Description |
|---------|-------------|
| Overall Glue Index | Percentage of "glue words" (the, a, is, etc.) |
| Sticky Sentence Detection | Sentences with >45% glue words |
| Semi-Sticky Detection | Sentences with 35-45% glue words |
| Position Tracking | Character-level positions for highlighting |

</details>

<details open>
<summary><strong>⚡ Pacing Analysis</strong></summary>

| Category | Description |
|----------|-------------|
| Fast-Paced | Sentences with <10 words |
| Medium-Paced | Sentences with 10-20 words |
| Slow-Paced | Sentences with >20 words |
| Distribution | Count and percentage per category |

</details>

<details open>
<summary><strong>📏 Sentence Structure</strong></summary>

| Feature | Description |
|---------|-------------|
| Average Length | Mean word count per sentence |
| Standard Deviation | Variation in sentence length |
| Variety Score | 0-10 scale measuring length diversity |
| Very Long Detection | Sentences >30 words flagged |
| Shortest/Longest | Extremes identified |

</details>

<details open>
<summary><strong>🔄 Transition Analysis</strong></summary>

| Feature | Description |
|---------|-------------|
| Transition Count | Sentences containing transitions |
| Transition Percentage | Ratio of sentences with transitions |
| Unique Transitions | Count of distinct transition words |
| Most Common | Ranked list with frequencies |
| Multi-word Support | Detects phrases like "on the other hand" |

</details>

<details open>
<summary><strong>🔁 Repetition Analysis</strong></summary>

| Feature | Description |
|---------|-------------|
| Overused Words | Words appearing >0.5% frequency |
| Repeated Phrases | 2-4 word phrase repetition |
| Echoes | Word repetition within 20 words |
| Position Tracking | Exact character positions |

</details>

<details open>
<summary><strong>👁️👂✋👃👅 Sensory Language</strong></summary>

| Sense | Example Words |
|-------|---------------|
| Sight | see, bright, vivid, sparkle, glowing |
| Sound | hear, loud, whisper, echo, buzz |
| Touch | feel, soft, rough, texture, smooth |
| Smell | scent, aroma, fragrant, stench |
| Taste | flavor, sweet, savory, bitter |

</details>

<details open>
<summary><strong>💭 Diction Analysis</strong></summary>

| Feature | Description |
|---------|-------------|
| Vague Words | thing, stuff, nice, very, really |
| Vague Phrases | "kind of", "sort of", "a bit" |
| Frequency Count | Per-word usage statistics |
| Position Tracking | Character-level locations |

</details>

<details open>
<summary><strong>🎭 Cliché Detection</strong></summary>

50+ common clichés tracked including:
- "avoid like the plague"
- "piece of cake"
- "think outside the box"
- "at the end of the day"
- "break the ice"

</details>

<details open>
<summary><strong>✅ Consistency Checking</strong></summary>

| Check Type | Examples |
|------------|----------|
| US vs UK Spelling | color/colour, analyze/analyse |
| Hyphenation | email/e-mail, online/on-line |
| Capitalization | Inconsistent title case |

</details>

<details open>
<summary><strong>🔤 Acronym Analysis</strong></summary>

| Feature | Description |
|---------|-------------|
| Total Count | All acronym occurrences |
| Unique Count | Distinct acronyms found |
| Frequency List | Ranked by usage |

</details>

<details open>
<summary><strong>💼 Business Jargon</strong></summary>

Single words and phrases detected:
- synergy, leverage, paradigm
- "circle back", "touch base"
- "low-hanging fruit", "move the needle"

</details>

<details open>
<summary><strong>🧩 Complex Paragraphs</strong></summary>

| Threshold | Description |
|-----------|-------------|
| Sentence Length | >20 words average |
| Syllables | >1.8 per word average |
| Position Tracking | Start/end character indices |

</details>

<details open>
<summary><strong>🔗 Conjunction Starts</strong></summary>

Tracks sentences beginning with:
- and, but, or, so, yet, for, nor

</details>

---

## Architecture

### System Design Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Rust_Grammar Architecture                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐  │
│  │   CLI App   │    │  REST API   │    │   Library   │    │ HTML Visual │  │
│  │  (main.rs)  │    │(api-server) │    │  (lib.rs)   │    │(visualizer) │  │
│  └──────┬──────┘    └──────┬──────┘    └──────┬──────┘    └──────┬──────┘  │
│         │                  │                  │                  │          │
│         └──────────────────┴────────┬─────────┴──────────────────┘          │
│                                     │                                        │
│                          ┌──────────▼──────────┐                            │
│                          │    TextAnalyzer     │                            │
│                          │   (Core Engine)     │                            │
│                          └──────────┬──────────┘                            │
│                                     │                                        │
│         ┌───────────────────────────┼───────────────────────────┐           │
│         │                           │                           │           │
│  ┌──────▼──────┐           ┌───────▼───────┐           ┌───────▼───────┐   │
│  │   Grammar   │           │Comprehensive  │           │ Configuration │   │
│  │   Module    │           │   Analysis    │           │    System     │   │
│  ├─────────────┤           ├───────────────┤           ├───────────────┤   │
│  │• Sentence   │           │• Sticky Sent. │           │• YAML/TOML    │   │
│  │  Splitter   │           │• Pacing       │           │• Presets      │   │
│  │• Passive    │           │• Transitions  │           │• Thresholds   │   │
│  │  Voice      │           │• Sensory      │           │• Feature      │   │
│  │• Grammar    │           │• Clichés      │           │  Toggles      │   │
│  │  Checker    │           │• Jargon       │           │               │   │
│  └──────┬──────┘           └───────┬───────┘           └───────────────┘   │
│         │                          │                                        │
│  ┌──────▼──────────────────────────▼──────┐                                │
│  │           Dictionaries Module           │                                │
│  ├─────────────────────────────────────────┤                                │
│  │  • 200+ Abbreviations                   │                                │
│  │  • 200+ Irregular Past Participles      │                                │
│  │  • 1000+ Syllable Counts                │                                │
│  │  • Glue Words, Transitions, Clichés     │                                │
│  └─────────────────────────────────────────┘                                │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────────┐│
│  │                         Error Handling Layer                            ││
│  │    Result<T, AnalysisError> throughout • Zero crashes guaranteed        ││
│  └─────────────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────────────┘
```

### Directory Structure

```
rust_grammar/
├── Cargo.toml                        # Project manifest and dependencies
├── README.md                         # This documentation
├── LICENSE                           # MIT License
├── config.example.yaml               # Example configuration file
├── sample.txt                        # Sample text for testing
│
├── src/
│   ├── main.rs                       # CLI entry point with clap
│   ├── lib.rs                        # Core library exports
│   ├── error.rs                      # Custom error types (thiserror)
│   ├── config.rs                     # Configuration system
│   ├── word_lists.rs                 # Static word dictionaries
│   ├── analysis_reports.rs           # Report data structures
│   ├── comprehensive_analysis.rs     # All 19 analysis features
│   ├── visualizer.rs                 # HTML report generator
│   │
│   ├── bin/
│   │   ├── api-server.rs             # Basic REST API server (1 endpoint)
│   │   └── api-server-enhanced.rs    # Enhanced API with 6 endpoints
│   │
│   ├── dictionaries/
│   │   ├── mod.rs                    # Module exports
│   │   ├── abbreviations.rs          # 200+ abbreviations
│   │   ├── irregular_verbs.rs        # Past participles dictionary
│   │   └── syllable_dict.rs          # 1000+ syllable counts
│   │
│   └── grammar/
│       ├── mod.rs                    # Module exports
│       ├── sentence_splitter.rs      # Advanced sentence boundary detection
│       ├── passive_voice.rs          # Confidence-scored detection
│       └── checker.rs                # Grammar rules engine
│
├── tests/
│   └── integration_tests.rs          # Comprehensive integration tests
│
├── benches/
│   └── performance.rs                # Criterion benchmarks
│
└── docs/
    ├── API_DOCUMENTATION.md          # REST API documentation
    ├── IMPLEMENTATION.md             # Technical details
    ├── QUICKSTART.md                 # Quick setup guide
    └── CHANGELOG.md                  # Version history
```

---

## Installation

### Prerequisites

| Requirement | Minimum Version | Notes |
|------------|-----------------|-------|
| Rust | 1.75+ | Install via [rustup](https://rustup.rs/) |
| Cargo | Latest | Included with Rust |
| Git | Any recent | For cloning repository |

### Method 1: Install from Crates.io

```bash
# Install as a library dependency
cargo add Rust_Grammar

# Or add to Cargo.toml manually
[dependencies]
Rust_Grammar = "2.1.1"
```

### Method 2: Install from Source

```bash
# Clone the repository
git clone https://github.com/Eeman1113/rust_grammar.git
cd rust_grammar

# Build release version (optimized)
cargo build --release

# Verify installation
cargo test

# Optional: Install binary globally
cargo install --path .
```

### Method 3: Build API Server

```bash
# Build enhanced API server (recommended)
cargo build --release --features server --bin api-server-enhanced

# Or build basic API server
cargo build --release --features server --bin api-server
```

### Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `cli` | Command-line interface | ✅ Yes |
| `server` | REST API server | ❌ No |
| `parallel` | Rayon parallel processing | ✅ Yes |
| `markdown` | Markdown preprocessing | ✅ Yes |
| `html` | HTML parsing support | ✅ Yes |
| `full` | All features enabled | ❌ No |

---

## Configuration

### Configuration File Format

The analyzer supports both YAML and TOML configuration formats.

#### YAML Configuration (`config.yaml`)

```yaml
# Text Analyzer Configuration

# Input validation settings
validation:
  max_file_size_mb: 10
  min_words: 10
  max_words: null
  timeout_seconds: 300

# Analysis behavior settings
analysis:
  parallel_processing: true
  cache_results: false
  document_type: general  # general|academic|fiction|business|technical

# Threshold settings
thresholds:
  sticky_sentence_threshold: 40.0
  overused_word_threshold: 0.5
  echo_distance: 20
  very_long_sentence: 30
  complex_paragraph_sentence_length: 20.0
  complex_paragraph_syllables: 1.8
  passive_voice_max: 10
  adverb_max: 20

# Feature toggles
features:
  grammar_check: true
  style_check: true
  readability_check: true
  consistency_check: true
  sensory_analysis: true
  cliche_detection: true
  jargon_detection: true
  echo_detection: true

# Output settings
output:
  format: text  # text|json|yaml|html
  verbosity: normal  # quiet|normal|verbose|debug
  color: true
  show_progress: true
```

### Document Type Presets

| Preset | Use Case | Key Modifications |
|--------|----------|-------------------|
| `general` | Default for most documents | Balanced settings |
| `academic` | Research papers, theses | Lenient on passive voice (max=20%) |
| `fiction` | Novels, short stories | Strict sticky sentences (35%) |
| `business` | Reports, proposals | Lenient glue words (45%) |
| `technical` | Documentation, manuals | Lenient complexity |

---

## Usage

### CLI Usage

```bash
# Basic analysis
./target/release/text-analyzer myfile.txt

# Comprehensive analysis (ALL 19 features)
./target/release/text-analyzer myfile.txt --all

# With document type preset
./target/release/text-analyzer paper.txt -a -t academic

# Save report to file
./target/release/text-analyzer myfile.txt -a -o report.txt

# JSON output
./target/release/text-analyzer myfile.txt -f json

# Visual HTML report
./target/release/text-analyzer myfile.txt -V

# Use custom configuration
./target/release/text-analyzer myfile.txt -c config.yaml
```

### Starting the API Server

```bash
# Start enhanced API server (recommended)
./target/release/api-server-enhanced

# Output:
# 🚀 Text Analyzer API running on http://0.0.0.0:2000
# 📝 POST to http://0.0.0.0:2000/analyze with JSON body: {"text": "your text"}
# 📊 POST to http://0.0.0.0:2000/score for scores only
# 📏 POST to http://0.0.0.0:2000/sentencelength for sentence length analysis
# 📖 POST to http://0.0.0.0:2000/readability for readability analysis
# 🎯 POST to http://0.0.0.0:2000/passivevoice for passive voice analysis
# 🔗 POST to http://0.0.0.0:2000/glueindex for glue index analysis
```

---

## REST API Reference

The API server runs on `http://0.0.0.0:2000` by default with CORS enabled.

### API Servers

| Server | Binary | Endpoints | Use Case |
|--------|--------|-----------|----------|
| Basic | `api-server` | 1 (`/analyze`) | Simple integration |
| Enhanced | `api-server-enhanced` | 6 | Full-featured applications |

---

### Endpoint Summary

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/analyze` | POST | Full analysis with all scores and issues |
| `/score` | POST | Scores only with ideal values and status |
| `/sentencelength` | POST | Detailed sentence length analysis |
| `/readability` | POST | Readability metrics and difficult paragraphs |
| `/passivevoice` | POST | Passive voice, adverbs, hidden verbs, and style |
| `/glueindex` | POST | Glue index and sticky sentences |

---

### 1. POST `/analyze`

**Full analysis with comprehensive scores and all detected issues.**

#### Request

```bash
curl -X POST http://localhost:2000/analyze \
  -H "Content-Type: application/json" \
  -d '{"text": "Your text to analyze here."}'
```

**Request Body:**
```json
{
  "text": "The report was written yesterday. This is very important."
}
```

#### Response

```json
{
  "scores": {
    "styleScore": {
      "score": 75,
      "percentage": 75.0,
      "message": "Good writing with minor improvements needed"
    },
    "styleGuideCompliance": {
      "score": 100,
      "percentage": 100.0,
      "message": "Create your own style guide"
    },
    "sentenceLength": {
      "score": 8,
      "percentage": 8.0,
      "message": "Use a few longer sentences to add more depth to your writing.",
      "numWords": 9,
      "numCharacters": 58,
      "avgSentenceLength": 4.5,
      "targetRange": "11 to 18",
      "sentenceVariety": 2.1,
      "varietyTarget": "over 3",
      "sentencesByWordCount": {
        "under10": 2,
        "range10to19": 0,
        "range20to29": 0,
        "range30to39": 0,
        "over40": 0
      },
      "individualSentenceLengths": [...]
    },
    "readabilityGrade": {
      "score": 5,
      "percentage": 31.25,
      "message": null
    },
    "sentenceVariety": {
      "score": 6,
      "percentage": 6.0,
      "message": null
    },
    "glueIndex": {
      "percentage": 33.3,
      "count": 0,
      "total": 2,
      "message": "Reduce the number of glue words to make your writing clearer.",
      "occurrences": []
    },
    "passiveVoice": {
      "count": 1,
      "percentage": 50.0,
      "message": null,
      "occurrences": [
        {
          "start": 11,
          "end": 22,
          "length": 11,
          "string": "was written",
          "paragraphKey": "0"
        }
      ]
    },
    "businessJargon": {
      "count": 0,
      "percentage": 0.0,
      "message": null,
      "occurrences": []
    },
    "complexParagraphs": {
      "percentage": 0.0,
      "count": 0,
      "total": 1,
      "message": null,
      "occurrences": []
    },
    "conjunctionStarts": {
      "percentage": 0.0,
      "count": 0,
      "total": 2,
      "message": null
    },
    "slowPacing": {
      "percentage": 0.0,
      "count": 0,
      "total": 2,
      "message": null
    },
    "veryLongSentences": {
      "percentage": 0.0,
      "count": 0,
      "total": 2,
      "message": null,
      "occurrences": []
    },
    "emotionTells": {
      "count": 0,
      "percentage": 0.0,
      "message": null
    },
    "ingStarts": {
      "percentage": 0.0,
      "count": 0,
      "total": 2,
      "message": "Rewrite some of your sentences that start with words ending in \"-ing.\"",
      "occurrences": []
    },
    "dialogueTags": {
      "percentage": 0.0,
      "count": 0,
      "total": 0,
      "message": "Remove some dialogue tags to help your writing flow better."
    },
    "unusualDialogueTags": {
      "percentage": 0.0,
      "count": 0,
      "total": 0,
      "message": null
    },
    "dialogueTagsWithAdverbs": {
      "percentage": 0.0,
      "count": 0,
      "total": 0,
      "message": "Remove adverbs from dialogue tags and \"show\" emotions more."
    },
    "weakAdverbs": {
      "count": 0,
      "percentage": 0.0,
      "message": "Replace some adverbs with stronger verbs to improve engagement.",
      "occurrences": []
    }
  },
  "issues": [
    {
      "Id": "auto_11_22_11_passive_was_written",
      "start": 11,
      "length": 11,
      "end": 22,
      "paragraphKey": "0",
      "string": "was written",
      "type": "PassiveVoice",
      "suggestions": {
        "recommendation": ["Consider using active voice for clarity"]
      }
    },
    {
      "Id": "auto_37_41_4_vague_very",
      "start": 37,
      "length": 4,
      "end": 41,
      "paragraphKey": "0",
      "string": "very",
      "type": "VagueWord",
      "suggestions": {
        "recommendation": ["Be more specific", "Use concrete language"]
      }
    }
  ],
  "summary": {
    "total_issues": 2,
    "word_count": 9,
    "sentence_count": 2,
    "paragraph_count": 1,
    "character_count": 58
  }
}
```

---

### 2. POST `/score`

**Returns only scoring metrics with ideal values, status, and helpful messages.**

#### Request

```bash
curl -X POST http://localhost:2000/score \
  -H "Content-Type: application/json" \
  -d '{"text": "Your text to analyze here."}'
```

**Request Body:**
```json
{
  "text": "The report was written yesterday. This is very important."
}
```

#### Response

```json
{
  "scores": {
    "styleScore": {
      "current": 75.0,
      "ideal": "80-100",
      "status": "fair",
      "message": "Good foundation. Focus on reducing passive voice and improving sentence variety."
    },
    "sentenceLength": {
      "current": 4.5,
      "ideal": "15-20 words",
      "status": "needs improvement",
      "message": "Your sentences average 4.5 words. Add longer sentences (15-20 words) to create more depth and flow in your writing."
    },
    "readability": {
      "current": 5.2,
      "ideal": "7-9 grade level",
      "status": "fair",
      "message": "Your writing is at grade level 5.2. Consider using more complex sentence structures and vocabulary to add sophistication."
    },
    "sentenceVariety": {
      "current": 2.1,
      "ideal": "5-10",
      "status": "needs improvement",
      "message": "Your sentences lack variety (σ=2.1). Combine short punchy sentences with longer, flowing ones to maintain reader interest."
    },
    "glueIndex": {
      "current": 33.3,
      "ideal": "< 40%",
      "status": "good",
      "message": "Great! Your glue word usage is within the ideal range, keeping your writing clear and direct."
    },
    "passiveVoice": {
      "current": 50.0,
      "ideal": "< 10%",
      "status": "needs improvement",
      "message": "High passive voice usage: 1 instances (50.0%). Rewrite in active voice to make your writing more dynamic and clear."
    },
    "businessJargon": {
      "current": 0.0,
      "ideal": "0 instances",
      "status": "good",
      "message": "Perfect! No business jargon detected. Your writing is clear and accessible."
    },
    "complexParagraphs": {
      "current": 0.0,
      "ideal": "0%",
      "status": "good",
      "message": "Great! All your paragraphs are easy to read with well-balanced sentence lengths."
    },
    "conjunctionStarts": {
      "current": 0.0,
      "ideal": "< 10%",
      "status": "good",
      "message": "Good! Your sentence variety creates natural flow without overusing conjunctions at the start."
    },
    "slowPacing": {
      "current": 0.0,
      "ideal": "< 30%",
      "status": "good",
      "message": "Excellent pacing! Your sentences maintain good rhythm and energy."
    },
    "veryLongSentences": {
      "current": 0.0,
      "ideal": "< 10%",
      "status": "good",
      "message": "Perfect! No overly long sentences. Your writing maintains good readability."
    },
    "emotionTells": {
      "current": 0.0,
      "ideal": "0 instances",
      "status": "good",
      "message": "Excellent! You're showing emotions through action and description rather than telling."
    },
    "ingStarts": {
      "current": 0.0,
      "ideal": "< 10%",
      "status": "good",
      "message": "Great! Your sentence variety keeps the writing fresh and engaging."
    },
    "weakAdverbs": {
      "current": 0.0,
      "ideal": "< 5%",
      "status": "good",
      "message": "Perfect! No weak adverbs detected. Your verbs are strong and precise."
    },
    "dialogueTags": {
      "current": 0.0,
      "ideal": "< 50%",
      "status": "good",
      "message": "No dialogue detected, or dialogue is well-balanced with minimal tags."
    }
  },
  "word_count": 9,
  "sentence_count": 2,
  "complex_words_count": 2
}
```

---

### 3. POST `/sentencelength`

**Detailed sentence length analysis with individual sentence positions.**

#### Request

```bash
curl -X POST http://localhost:2000/sentencelength \
  -H "Content-Type: application/json" \
  -d '{
    "data": [
      {"text": "The quick brown fox jumps over the lazy dog. This is a test.", "key": "para_0"},
      {"text": "Another paragraph with different content.", "key": "para_1"}
    ]
  }'
```

**Request Body:**
```json
{
  "data": [
    {
      "text": "Paragraph text here.",
      "key": "para_0"
    },
    {
      "text": "Another paragraph.",
      "key": "para_1"
    }
  ]
}
```

#### Response

```json
{
  "score": 70,
  "percentage": 70.0,
  "message": "Great! Your sentence length is in the ideal range.",
  "numWords": 15,
  "numCharacters": 75,
  "avgSentenceLength": 7.5,
  "longestSentenceLength": 9,
  "targetRange": "11 to 18",
  "sentenceVariety": 2.1,
  "varietyTarget": "over 3",
  "sentencesByWordCount": {
    "under10": 2,
    "range10to19": 0,
    "range20to29": 0,
    "range30to39": 0,
    "over40": 0
  },
  "individualSentenceLengths": [
    {
      "start": 0,
      "end": 44,
      "length": 44,
      "string": "The quick brown fox jumps over the lazy dog.",
      "wordCount": 9,
      "paragraphKey": "para_0",
      "kind": "under10"
    },
    {
      "start": 45,
      "end": 60,
      "length": 15,
      "string": "This is a test.",
      "wordCount": 4,
      "paragraphKey": "para_0",
      "kind": "under10"
    },
    {
      "start": 0,
      "end": 40,
      "length": 40,
      "string": "Another paragraph with different content.",
      "wordCount": 5,
      "paragraphKey": "para_1",
      "kind": "under10"
    }
  ]
}
```

---

### 4. POST `/readability`

**Readability metrics with difficult paragraph identification.**

#### Request

```bash
curl -X POST http://localhost:2000/readability \
  -H "Content-Type: application/json" \
  -d '{
    "data": [
      {"text": "Your paragraph text here.", "key": "para_0"}
    ]
  }'
```

**Request Body:**
```json
{
  "data": [
    {
      "text": "Paragraph text here.",
      "key": "para_0"
    }
  ]
}
```

#### Response

```json
{
  "estimatedReadingTime": "0 min, 15 sec",
  "message": "Excellent! Your document is very easy to read - perfect for a wide audience.",
  "fleschReadingEase": 92.5,
  "fleschKincaidGrade": 4.2,
  "colemanLiau": 6.1,
  "automatedReadabilityIndex": 5.3,
  "difficultParagraphs": [
    {
      "difficulty": "slightly difficult",
      "start": 0,
      "end": 250,
      "string": "Full paragraph text...",
      "excerpt": "First 50 characters of the paragraph...",
      "paragraphKey": "para_2"
    }
  ]
}
```

**Difficulty Levels:**
- `"very hard"` - Average word length >6.5 and sentence length >30
- `"hard"` - Average word length >5.5 and sentence length >25
- `"slightly difficult"` - Sentence length >20

---

### 5. POST `/passivevoice`

**Comprehensive style analysis including passive voice, adverbs, hidden verbs, and more.**

#### Request

```bash
curl -X POST http://localhost:2000/passivevoice \
  -H "Content-Type: application/json" \
  -d '{
    "data": [
      {"text": "The report was written by the team. It seemed very important.", "key": "para_0"}
    ]
  }'
```

**Request Body:**
```json
{
  "data": [
    {
      "text": "Paragraph text here.",
      "key": "para_0"
    }
  ]
}
```

#### Response

```json
{
  "passiveVerbsFound": 1,
  "passiveVerbsMessage": "1 passive verb found. Consider revising for stronger, more direct writing.",
  "passiveVerbs": [
    {
      "verb": "was written",
      "count": 1,
      "occurrences": [
        {
          "start": 11,
          "end": 22,
          "string": "was written",
          "paragraphKey": "para_0",
          "report": "passiveVerbs"
        }
      ]
    }
  ],
  "hiddenVerbsFound": 0,
  "hiddenVerbsMessage": "No hidden verbs found.",
  "hiddenVerbs": [],
  "adverbsInDialogue": 0,
  "adverbsOutsideDialogue": 1,
  "adverbsMessage": "1 adverb found outside dialogue. Use sparingly.",
  "adverbsList": [
    {
      "adverb": "very",
      "count": 1,
      "occurrences": [
        {
          "start": 47,
          "end": 51,
          "string": "very",
          "paragraphKey": "para_0",
          "report": "adverbs"
        }
      ]
    }
  ],
  "readabilityEnhancementsFound": 0,
  "readabilityEnhancementsMessage": "No weak constructions found.",
  "readabilityEnhancements": [],
  "inclusiveLanguageMessage": "No non-inclusive language detected.",
  "inclusiveLanguageImprovements": [],
  "emotionTellsMessage": "1 emotion tell found (e.g., 'felt', 'seemed'). Show, don't tell emotions.",
  "emotionTells": [
    {
      "phrase": "seemed",
      "count": 1,
      "occurrences": [...]
    }
  ],
  "styleImprovementsMessage": "1 filler word found (e.g., 'very', 'really'). Remove for stronger writing.",
  "styleImprovements": [...],
  "businessJargonMessage": "No business jargon found.",
  "businessJargon": [],
  "longSubordinateClausesMessage": "No overly complex sentences found.",
  "longSubordinateClauses": [],
  "passiveIndex": 50.0,
  "passiveIndexMessage": "Very high passive voice: 50.0% (target: up to 25%). Extensive revision needed - convert to active voice.",
  "passiveIndexTarget": "up to 25",
  "repeatedSentenceStartsMessage": "No repetitive sentence starts found.",
  "repeatedSentenceStarts": [],
  "styleGuideItemsMessage": "No style guide violations found.",
  "styleGuideItems": []
}
```

**Analysis Categories:**
| Category | Description |
|----------|-------------|
| `passiveVerbs` | Passive voice constructions |
| `hiddenVerbs` | Nominalizations (e.g., "make a decision" → "decide") |
| `adverbsList` | Words ending in -ly |
| `readabilityEnhancements` | Weak constructions ("there is", "it was") |
| `inclusiveLanguageImprovements` | Gendered/non-inclusive language |
| `emotionTells` | Words like "felt", "seemed", "appeared" |
| `styleImprovements` | Filler words ("very", "really", "just") |
| `businessJargon` | Corporate buzzwords |
| `longSubordinateClauses` | Sentences with 3+ commas |
| `repeatedSentenceStarts` | Words used 3+ times to start sentences |
| `styleGuideItems` | Common grammar mistakes ("alot", "could of") |

---

### 6. POST `/glueindex`

**Glue word analysis and sticky sentence detection.**

#### Request

```bash
curl -X POST http://localhost:2000/glueindex \
  -H "Content-Type: application/json" \
  -d '{
    "data": [
      {"text": "The fact that it is the case that the problem is in the system.", "key": "para_0"}
    ]
  }'
```

**Request Body:**
```json
{
  "data": [
    {
      "text": "Paragraph text here.",
      "key": "para_0"
    }
  ]
}
```

#### Response

```json
{
  "glueIndex": 45.2,
  "glueIndexTarget": "up to 40%",
  "sentences": [
    {
      "start": 0,
      "end": 63,
      "string": "The fact that it is the case that the problem is in the system.",
      "excerpt": "The fact that it is the case that the problem is in...",
      "gluePercentage": 61.5,
      "category": "sticky",
      "paragraphKey": "para_0"
    }
  ]
}
```

**Sentence Categories:**
| Category | Glue Percentage | Description |
|----------|-----------------|-------------|
| `sticky` | >45% | High glue word density, needs revision |
| `semi-sticky` | 35-45% | Moderate glue word density |

---

### Error Responses

All endpoints return consistent error responses:

```json
{
  "error": "Error message description"
}
```

| Status Code | Error | Description |
|-------------|-------|-------------|
| 400 | `Text cannot be empty` | Empty text provided |
| 500 | `Failed to analyze text: [details]` | Analysis processing error |

---

## Code Examples

### Example 1: Rust Library Usage

```rust
use Rust_Grammar::{TextAnalyzer, Config, FullAnalysisReport};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string("article.txt")?;
    let config = Config::default();
    let analyzer = TextAnalyzer::new(text, config)?;
    
    // Get basic statistics
    let stats = analyzer.statistics();
    println!("Words: {}", stats.word_count);
    
    // Get readability
    let readability = analyzer.readability_metrics()?;
    println!("Reading Ease: {:.1}", readability.flesch_reading_ease);
    
    // Check grammar
    let grammar = analyzer.check_grammar()?;
    println!("Grammar issues: {}", grammar.len());
    
    // Detect passive voice
    let passive = analyzer.detect_passive_voice()?;
    for pv in &passive {
        println!("Passive: '{}' (confidence: {:.0}%)", 
            pv.text, pv.confidence * 100.0);
    }
    
    // Full comprehensive report
    let report: FullAnalysisReport = analyzer.generate_full_report()?;
    println!("Style Score: {}%", report.style_score);
    
    Ok(())
}
```

### Example 2: Python API Client

```python
#!/usr/bin/env python3
import requests

API_BASE = "http://localhost:2000"

def analyze_text(text: str) -> dict:
    """Full analysis with scores and issues."""
    response = requests.post(
        f"{API_BASE}/analyze",
        json={"text": text}
    )
    return response.json()

def get_scores(text: str) -> dict:
    """Get scores only with ideal values."""
    response = requests.post(
        f"{API_BASE}/score",
        json={"text": text}
    )
    return response.json()

def get_sentence_length(paragraphs: list) -> dict:
    """Analyze sentence length by paragraph."""
    response = requests.post(
        f"{API_BASE}/sentencelength",
        json={"data": paragraphs}
    )
    return response.json()

def get_readability(paragraphs: list) -> dict:
    """Get readability metrics."""
    response = requests.post(
        f"{API_BASE}/readability",
        json={"data": paragraphs}
    )
    return response.json()

def get_passive_voice(paragraphs: list) -> dict:
    """Get passive voice and style analysis."""
    response = requests.post(
        f"{API_BASE}/passivevoice",
        json={"data": paragraphs}
    )
    return response.json()

def get_glue_index(paragraphs: list) -> dict:
    """Get glue index and sticky sentences."""
    response = requests.post(
        f"{API_BASE}/glueindex",
        json={"data": paragraphs}
    )
    return response.json()

# Example usage
if __name__ == "__main__":
    text = """
    The comprehensive report was written by the research team. 
    At the end of the day, we need to leverage our synergies.
    """
    
    # Full analysis
    result = analyze_text(text)
    print(f"Style Score: {result['scores']['styleScore']['score']}")
    print(f"Issues Found: {result['summary']['total_issues']}")
    
    # Scores only
    scores = get_scores(text)
    for metric, data in scores['scores'].items():
        print(f"{metric}: {data['current']} (ideal: {data['ideal']}) - {data['status']}")
    
    # Paragraph-based analysis
    paragraphs = [
        {"text": "First paragraph text.", "key": "para_0"},
        {"text": "Second paragraph text.", "key": "para_1"}
    ]
    
    readability = get_readability(paragraphs)
    print(f"Flesch Reading Ease: {readability['fleschReadingEase']}")
    
    passive = get_passive_voice(paragraphs)
    print(f"Passive Voice Found: {passive['passiveVerbsFound']}")
    
    glue = get_glue_index(paragraphs)
    print(f"Glue Index: {glue['glueIndex']}%")
```

### Example 3: JavaScript/Node.js Client

```javascript
const API_BASE = 'http://localhost:2000';

async function analyzeText(text) {
  const response = await fetch(`${API_BASE}/analyze`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ text })
  });
  return response.json();
}

async function getScores(text) {
  const response = await fetch(`${API_BASE}/score`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ text })
  });
  return response.json();
}

async function getSentenceLength(paragraphs) {
  const response = await fetch(`${API_BASE}/sentencelength`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ data: paragraphs })
  });
  return response.json();
}

async function getReadability(paragraphs) {
  const response = await fetch(`${API_BASE}/readability`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ data: paragraphs })
  });
  return response.json();
}

async function getPassiveVoice(paragraphs) {
  const response = await fetch(`${API_BASE}/passivevoice`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ data: paragraphs })
  });
  return response.json();
}

async function getGlueIndex(paragraphs) {
  const response = await fetch(`${API_BASE}/glueindex`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ data: paragraphs })
  });
  return response.json();
}

// Example usage
async function main() {
  const text = `
    The comprehensive report was written by the research team.
    At the end of the day, we need to leverage our synergies.
  `;
  
  // Full analysis
  const result = await analyzeText(text);
  console.log(`Style Score: ${result.scores.styleScore.score}`);
  console.log(`Issues: ${result.summary.total_issues}`);
  
  // Scores with guidance
  const scores = await getScores(text);
  Object.entries(scores.scores).forEach(([metric, data]) => {
    console.log(`${metric}: ${data.current} (${data.status})`);
    console.log(`  → ${data.message}`);
  });
  
  // Paragraph-based analysis
  const paragraphs = [
    { text: 'First paragraph.', key: 'para_0' },
    { text: 'Second paragraph.', key: 'para_1' }
  ];
  
  const passive = await getPassiveVoice(paragraphs);
  console.log(`Passive Index: ${passive.passiveIndex}%`);
  console.log(passive.passiveIndexMessage);
}

main().catch(console.error);
```

### Example 4: cURL Test Script

```bash
#!/bin/bash
# test-api.sh - Test all API endpoints

API_URL="http://localhost:2000"
TEXT='{"text": "The report was written yesterday. This is very important."}'
PARA='{"data": [{"text": "The report was written yesterday. This is very important.", "key": "para_0"}]}'

echo "=== Testing /analyze ==="
curl -s -X POST "$API_URL/analyze" \
  -H "Content-Type: application/json" \
  -d "$TEXT" | jq '.summary'

echo -e "\n=== Testing /score ==="
curl -s -X POST "$API_URL/score" \
  -H "Content-Type: application/json" \
  -d "$TEXT" | jq '.scores.styleScore'

echo -e "\n=== Testing /sentencelength ==="
curl -s -X POST "$API_URL/sentencelength" \
  -H "Content-Type: application/json" \
  -d "$PARA" | jq '{score, avgSentenceLength, message}'

echo -e "\n=== Testing /readability ==="
curl -s -X POST "$API_URL/readability" \
  -H "Content-Type: application/json" \
  -d "$PARA" | jq '{fleschReadingEase, fleschKincaidGrade, message}'

echo -e "\n=== Testing /passivevoice ==="
curl -s -X POST "$API_URL/passivevoice" \
  -H "Content-Type: application/json" \
  -d "$PARA" | jq '{passiveVerbsFound, passiveIndex, passiveIndexMessage}'

echo -e "\n=== Testing /glueindex ==="
curl -s -X POST "$API_URL/glueindex" \
  -H "Content-Type: application/json" \
  -d "$PARA" | jq '{glueIndex, glueIndexTarget}'

echo -e "\n✅ All tests complete!"
```

---

## Data Models

### Request Models

#### Simple Text Request (for `/analyze`, `/score`)

```typescript
interface AnalyzeRequest {
  text: string;
}
```

#### Paragraph Data Request (for `/sentencelength`, `/readability`, `/passivevoice`, `/glueindex`)

```typescript
interface ParagraphRequest {
  data: ParagraphData[];
}

interface ParagraphData {
  text: string;
  key: string;  // Unique identifier for the paragraph
}
```

### Response Models

<details>
<summary><strong>Occurrence Model (used across all endpoints)</strong></summary>

```typescript
interface Occurrence {
  start: number;       // Character position (start)
  end: number;         // Character position (end)
  length: number;      // Character length
  string: string;      // Matched text
  paragraphKey: string; // Reference to source paragraph
}
```

</details>

<details>
<summary><strong>Score Models</strong></summary>

```typescript
interface SimpleScore {
  current: number;    // Current value
  ideal: string;      // Target range (e.g., "< 10%")
  status: string;     // "good" | "fair" | "needs improvement"
  message: string;    // Actionable guidance
}

interface ScoreDetail {
  score: number;
  percentage: number;
  message?: string;
}

interface PercentageScore {
  percentage: number;
  count: number;
  total: number;
  message?: string;
  occurrences?: Occurrence[];
}

interface CountScore {
  count: number;
  percentage?: number;
  message?: string;
  occurrences?: Occurrence[];
}
```

</details>

<details>
<summary><strong>Issue Model</strong></summary>

```typescript
interface AnalysisIssue {
  Id: string;           // Unique identifier
  start: number;        // Character position (start)
  end: number;          // Character position (end)
  length: number;       // Character length
  paragraphKey: string; // Source paragraph
  string: string;       // Matched text
  type: string;         // Issue category
  suggestions: {
    recommendation: string[];
  };
}
```

**Issue Types:**
- `PassiveVoice`
- `Grammar_SubjectVerbAgreement`
- `Grammar_DoubleNegative`
- `Grammar_RunOnSentence`
- `StickySentence`
- `OverusedWord`
- `Repetition`
- `Cliche`
- `VagueWord`
- `BusinessJargon`

</details>

---

## Error Handling

### Error Types

```rust
pub enum AnalysisError {
    IoError(std::io::Error),
    ValidationError(String),
    EncodingError(std::string::FromUtf8Error),
    JsonError(serde_json::Error),
    YamlError(serde_yaml::Error),
    ConfigError(String),
    TimeoutError(u64),
    RegexError(regex::Error),
    FileTooLarge { size: u64, max: u64 },
    DocumentTooShort { words: usize, min: usize },
    EmptyInput,
    InvalidPath(PathBuf),
    ProcessingError(String),
}
```

### API Error Handling

```rust
enum ApiError {
    EmptyText,
    AnalysisError(String),
}

// Returns:
// 400 Bad Request - for empty text
// 500 Internal Server Error - for analysis failures
```

---

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific tests
cargo test grammar
cargo test integration
```

### API Testing

```bash
# Start the server
./target/release/api-server-enhanced &

# Run test script
./test-api.sh

# Or test individual endpoints
curl -X POST http://localhost:2000/analyze \
  -H "Content-Type: application/json" \
  -d '{"text": "Test sentence."}' | jq '.'
```

---

## Deployment

### Running the API Server

```bash
# Build
cargo build --release --features server --bin api-server-enhanced

# Run directly
./target/release/api-server-enhanced
# Server starts on http://0.0.0.0:2000

# Run with PM2 (production)
pm2 start ./target/release/api-server-enhanced --name text-analyzer-api
pm2 save
```

### Docker Deployment

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --features server --bin api-server-enhanced

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/api-server-enhanced /usr/local/bin/
EXPOSE 2000
CMD ["api-server-enhanced"]
```

```bash
docker build -t rust-grammar-api .
docker run -p 2000:2000 rust-grammar-api
```

### PM2 Configuration

```javascript
// ecosystem.config.js
module.exports = {
  apps: [{
    name: 'text-analyzer-api',
    script: './target/release/api-server-enhanced',
    instances: 1,
    autorestart: true,
    max_memory_restart: '1G',
    env: {
      RUST_LOG: 'info'
    }
  }]
};
```

---

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/new-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Create pull request

---

## Changelog & Roadmap

### v2.1.1 (Current)
- ✅ Enhanced API with 6 endpoints
- ✅ Paragraph-relative position tracking
- ✅ Unicode-safe character indexing
- ✅ Comprehensive style analysis
- ✅ Intelligent messaging system

### Roadmap
- [ ] WebSocket real-time analysis
- [ ] Multi-language support
- [ ] VS Code extension
- [ ] WebAssembly build

---

## FAQ

**Q: What port does the API run on?**
A: `0.0.0.0:2000` by default.

**Q: Is CORS enabled?**
A: Yes, permissive CORS is enabled for all endpoints.

**Q: What's the difference between the two API servers?**
A: `api-server` has 1 endpoint (`/analyze`), while `api-server-enhanced` has 6 specialized endpoints.

**Q: How are positions calculated?**
A: All positions are character-based (not byte-based) for proper Unicode support.

---

## License & Credits

**MIT License** - Copyright (c) 2025 Eeman Majumder

**Built with:**
- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Tokio](https://tokio.rs/) - Async runtime
- [Serde](https://serde.rs/) - Serialization
- [Tower-HTTP](https://github.com/tower-rs/tower-http) - HTTP middleware

---

<div align="center">

**Built with ❤️ using Rust 🦀**

[GitHub](https://github.com/Eeman1113/rust_grammar) · [Crates.io](https://crates.io/crates/Rust_Grammar)

</div>
