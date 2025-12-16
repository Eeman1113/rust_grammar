#!/bin/bash
# Test script for Text Analyzer API

API_URL="http://localhost:2000/analyze"

echo "🧪 Testing Text Analyzer API"
echo "============================"
echo ""

# Test 1: Simple text with passive voice
echo "📝 Test 1: Passive Voice Detection"
curl -s -X POST "$API_URL" \
  -H "Content-Type: application/json" \
  -d '{"text": "The report was written yesterday."}' | jq '.issues[] | select(.type == "PassiveVoice")'

echo ""
echo "---"
echo ""

# Test 2: Clichés and jargon
echo "📝 Test 2: Clichés and Business Jargon"
curl -s -X POST "$API_URL" \
  -H "Content-Type: application/json" \
  -d '{"text": "At the end of the day, we need to synergize our efforts and leverage our resources."}' | jq '.issues[] | {type, string, start, end}'

echo ""
echo "---"
echo ""

# Test 3: Overused words
echo "📝 Test 3: Overused Words"
curl -s -X POST "$API_URL" \
  -H "Content-Type: application/json" \
  -d '{"text": "The analysis is very important. The analysis was very thorough. The analysis showed very clear results."}' | jq '.issues[] | select(.type == "OverusedWord" or .type == "VagueWord") | {type, string, start}'

echo ""
echo "---"
echo ""

# Test 4: Comprehensive test
echo "📝 Test 4: Comprehensive Analysis"
TEXT="The comprehensive report was written by the research team last week. The results were very interesting and very significant. At the end of the day, we need to think outside the box and leverage our synergies. This approach will help us move the needle on our key performance indicators."

curl -s -X POST "$API_URL" \
  -H "Content-Type: application/json" \
  -d "{\"text\": \"$TEXT\"}" | jq '{
    total_issues: .summary.total_issues,
    word_count: .summary.word_count,
    style_score: .summary.style_score,
    issue_types: [.issues[].type] | unique
  }'

echo ""
echo "---"
echo ""

# Test 5: Get all issues with suggestions
echo "📝 Test 5: Issues with Suggestions"
curl -s -X POST "$API_URL" \
  -H "Content-Type: application/json" \
  -d '{"text": "The document was created. It is very important."}' | jq '.issues[] | {
    issue: .string,
    type: .type,
    position: "\(.start)-\(.end)",
    suggestions: .suggestions.recommendation
  }'

echo ""
echo "✅ All tests complete!"
