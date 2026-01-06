use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use Rust_Grammar::TextAnalyzer;

fn benchmark_sentence_splitting(c: &mut Criterion) {
    let text = "This is a test. Dr. Smith works at MIT. The U.S.A. is great. ".repeat(100);
    
    c.bench_function("sentence_splitting", |b| {
        b.iter(|| {
            TextAnalyzer::with_default_config(black_box(text.clone())).unwrap()
        });
    });
}

fn benchmark_readability_analysis(c: &mut Criterion) {
    let text = "This is a simple test sentence for benchmarking purposes. ".repeat(100);
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    c.bench_function("readability_metrics", |b| {
        b.iter(|| {
            analyzer.readability_metrics().unwrap()
        });
    });
}

fn benchmark_grammar_check(c: &mut Criterion) {
    let text = "He are going to the store. They is coming later. ".repeat(50);
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    c.bench_function("grammar_check", |b| {
        b.iter(|| {
            analyzer.check_grammar().unwrap()
        });
    });
}

fn benchmark_passive_voice(c: &mut Criterion) {
    let text = "The ball was thrown by John. The book was written by the author. ".repeat(50);
    let analyzer = TextAnalyzer::with_default_config(text).unwrap();
    
    c.bench_function("passive_voice_detection", |b| {
        b.iter(|| {
            analyzer.detect_passive_voice().unwrap()
        });
    });
}

fn benchmark_full_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_analysis");
    
    for size in [100, 500, 1000].iter() {
        let text = "This is a test sentence. Here is another one. Dr. Smith wrote this. The book was written by the author. ".repeat(*size);
        
        group.bench_with_input(BenchmarkId::from_parameter(size), &text, |b, text| {
            b.iter(|| {
                let analyzer = TextAnalyzer::with_default_config(black_box(text.clone())).unwrap();
                analyzer.readability_metrics().unwrap();
                analyzer.check_grammar().unwrap();
                analyzer.detect_passive_voice().unwrap();
            });
        });
    }
    
    group.finish();
}

fn benchmark_word_extraction(c: &mut Criterion) {
    let text = "well-known mother-in-law can't won't François naïve ".repeat(100);
    
    c.bench_function("word_extraction", |b| {
        b.iter(|| {
            TextAnalyzer::with_default_config(black_box(text.clone())).unwrap()
        });
    });
}

criterion_group!(
    benches,
    benchmark_sentence_splitting,
    benchmark_readability_analysis,
    benchmark_grammar_check,
    benchmark_passive_voice,
    benchmark_full_analysis,
    benchmark_word_extraction
);

criterion_main!(benches);
