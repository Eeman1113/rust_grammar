#!/usr/bin/env node
/**
 * Example Node.js client for Text Analyzer API
 */

const API_URL = 'http://localhost:2000/analyze';

/**
 * Analyze text using the Text Analyzer API
 */
async function analyzeText(text) {
  const response = await fetch(API_URL, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ text }),
  });

  if (!response.ok) {
    throw new Error(`API error: ${response.statusText}`);
  }

  return await response.json();
}

/**
 * Print analysis results
 */
function printAnalysis(result) {
  const { summary, issues } = result;

  console.log('\n' + '='.repeat(60));
  console.log('📊 ANALYSIS SUMMARY');
  console.log('='.repeat(60));
  console.log(`Total Issues:    ${summary.total_issues}`);
  console.log(`Word Count:      ${summary.word_count}`);
  console.log(`Sentence Count:  ${summary.sentence_count}`);
  console.log(`Style Score:     ${summary.style_score}/100`);

  if (issues.length > 0) {
    console.log('\n' + '='.repeat(60));
    console.log('🔍 ISSUES FOUND');
    console.log('='.repeat(60));

    // Group by type
    const byType = {};
    issues.forEach(issue => {
      if (!byType[issue.type]) {
        byType[issue.type] = [];
      }
      byType[issue.type].push(issue);
    });

    Object.entries(byType).sort().forEach(([type, issueList]) => {
      console.log(`\n📌 ${type} (${issueList.length} found)`);
      console.log('-'.repeat(60));

      issueList.slice(0, 5).forEach(issue => {
        console.log(`  Position ${issue.start}-${issue.end}: "${issue.string}"`);
        issue.suggestions.recommendation.forEach(suggestion => {
          console.log(`    💡 ${suggestion}`);
        });
      });

      if (issueList.length > 5) {
        console.log(`  ... and ${issueList.length - 5} more`);
      }
    });
  }
}

/**
 * Highlight text in browser-compatible format
 */
function highlightIssues(text, issues) {
  // Sort issues by start position (reverse)
  const sorted = [...issues].sort((a, b) => b.start - a.start);

  let highlighted = text;

  const colors = {
    PassiveVoice: '#ffcccc',
    Cliche: '#ffffcc',
    VagueWord: '#ffddaa',
    BusinessJargon: '#ddccff',
    OverusedWord: '#ffeecc',
    Repetition: '#ffccff',
    Grammar: '#ffaaaa',
  };

  sorted.forEach(issue => {
    const { start, end, type, suggestions } = issue;
    const baseType = type.split('_')[0];
    const color = colors[baseType] || '#dddddd';

    const before = highlighted.substring(0, start);
    const span = highlighted.substring(start, end);
    const after = highlighted.substring(end);

    const suggestionText = suggestions.recommendation.join(' | ');
    highlighted = `${before}<mark style="background-color: ${color};" title="${type}: ${suggestionText}">${span}</mark>${after}`;
  });

  return highlighted;
}

// Example usage
const testText = `
The comprehensive report was written by the research team last week. 
The results were very interesting and very significant. At the end of 
the day, we need to think outside the box and leverage our synergies. 
This approach will help us move the needle on our key performance indicators.
The analysis is very important. The analysis was very thorough.
`;

// Run analysis
(async () => {
  try {
    console.log('🔍 Analyzing text...');
    const result = await analyzeText(testText);

    printAnalysis(result);

    // Generate HTML output
    const highlighted = highlightIssues(testText, result.issues);
    const html = `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Text Analysis</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; line-height: 1.8; }
        .text { background: white; padding: 20px; border: 1px solid #ddd; white-space: pre-wrap; }
        mark { cursor: help; border-radius: 2px; padding: 2px 4px; }
    </style>
</head>
<body>
    <h1>📝 Text Analysis Results</h1>
    <p><strong>Style Score:</strong> ${result.summary.style_score}/100 | 
       <strong>Issues:</strong> ${result.summary.total_issues}</p>
    <div class="text">${highlighted}</div>
</body>
</html>
    `;

    // Save files (using Node.js fs module)
    const fs = require('fs');
    fs.writeFileSync('analysis_result.html', html);
    console.log('\n✅ HTML report saved to analysis_result.html');

    fs.writeFileSync('analysis_result.json', JSON.stringify(result, null, 2));
    console.log('✅ JSON report saved to analysis_result.json');

  } catch (error) {
    console.error('❌ Error:', error.message);
    process.exit(1);
  }
})();
