#!/usr/bin/env python3
"""
Example Python client for Text Analyzer API
"""

import requests
import json
from typing import Dict, List

API_URL = "http://localhost:2000/analyze"

def analyze_text(text: str) -> Dict:
    """
    Analyze text using the Text Analyzer API
    
    Args:
        text: The text to analyze
        
    Returns:
        Dictionary containing issues and summary
    """
    response = requests.post(
        API_URL,
        json={"text": text},
        headers={"Content-Type": "application/json"}
    )
    response.raise_for_status()
    return response.json()


def print_analysis(result: Dict):
    """Pretty print the analysis results"""
    summary = result['summary']
    issues = result['issues']
    
    print("\n" + "="*60)
    print("📊 ANALYSIS SUMMARY")
    print("="*60)
    print(f"Total Issues:    {summary['total_issues']}")
    print(f"Word Count:      {summary['word_count']}")
    print(f"Sentence Count:  {summary['sentence_count']}")
    print(f"Style Score:     {summary['style_score']}/100")
    
    if issues:
        print("\n" + "="*60)
        print("🔍 ISSUES FOUND")
        print("="*60)
        
        # Group by type
        by_type = {}
        for issue in issues:
            issue_type = issue['type']
            if issue_type not in by_type:
                by_type[issue_type] = []
            by_type[issue_type].append(issue)
        
        for issue_type, issue_list in sorted(by_type.items()):
            print(f"\n📌 {issue_type} ({len(issue_list)} found)")
            print("-" * 60)
            
            for issue in issue_list[:5]:  # Show first 5 of each type
                print(f"  Position {issue['start']}-{issue['end']}: \"{issue['string']}\"")
                for suggestion in issue['suggestions']['recommendation']:
                    print(f"    💡 {suggestion}")
            
            if len(issue_list) > 5:
                print(f"  ... and {len(issue_list) - 5} more")


def highlight_in_html(text: str, issues: List[Dict]) -> str:
    """
    Generate HTML with highlighted issues
    
    Args:
        text: Original text
        issues: List of issues from API
        
    Returns:
        HTML string with highlighted text
    """
    # Sort issues by start position (reverse to process from end)
    sorted_issues = sorted(issues, key=lambda x: x['start'], reverse=True)
    
    highlighted = text
    
    # Color map for different issue types
    colors = {
        'PassiveVoice': '#ffcccc',
        'Cliche': '#ffffcc',
        'VagueWord': '#ffddaa',
        'BusinessJargon': '#ddccff',
        'OverusedWord': '#ffeecc',
        'Repetition': '#ffccff',
        'Grammar': '#ffaaaa',
    }
    
    for issue in sorted_issues:
        start = issue['start']
        end = issue['end']
        issue_type = issue['type'].split('_')[0]  # Get base type
        color = colors.get(issue_type, '#dddddd')
        
        # Extract the text
        before = highlighted[:start]
        text_span = highlighted[start:end]
        after = highlighted[end:]
        
        # Wrap in span with title
        suggestions = ' | '.join(issue['suggestions']['recommendation'])
        highlighted = (
            f"{before}"
            f"<span style='background-color: {color}; cursor: help;' "
            f"title='{issue_type}: {suggestions}'>"
            f"{text_span}"
            f"</span>"
            f"{after}"
        )
    
    return f"""
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="utf-8">
        <title>Text Analysis</title>
        <style>
            body {{ font-family: Arial, sans-serif; margin: 40px; line-height: 1.6; }}
            .text {{ background: white; padding: 20px; border: 1px solid #ddd; }}
            span {{ border-radius: 2px; padding: 2px; }}
        </style>
    </head>
    <body>
        <h1>📝 Text Analysis Results</h1>
        <div class="text">{highlighted}</div>
    </body>
    </html>
    """


# Example usage
if __name__ == "__main__":
    # Test text
    test_text = """
    The comprehensive report was written by the research team last week. 
    The results were very interesting and very significant. At the end of 
    the day, we need to think outside the box and leverage our synergies. 
    This approach will help us move the needle on our key performance indicators.
    The analysis is very important. The analysis was very thorough.
    """
    
    print("🔍 Analyzing text...")
    result = analyze_text(test_text)
    
    print_analysis(result)
    
    # Generate HTML
    html = highlight_in_html(test_text, result['issues'])
    with open('analysis_result.html', 'w') as f:
        f.write(html)
    print("\n✅ HTML report saved to analysis_result.html")
    
    # Save JSON
    with open('analysis_result.json', 'w') as f:
        json.dump(result, f, indent=2)
    print("✅ JSON report saved to analysis_result.json")
