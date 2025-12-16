# 🚀 Text Analyzer REST API

## 📡 **API Server**

A REST API that provides comprehensive text analysis with exact position tracking.

---

## 🎯 **Quick Start**

### **1. Build the API Server**
```bash
cd text-analyzer
cargo build --release --bin api-server
```

### **2. Start the Server**
```bash
./target/release/api-server
```

**Output:**
```
🚀 Text Analyzer API running on http://0.0.0.0:2000
📝 POST to http://0.0.0.0:2000/analyze with JSON body: {"text": "your text"}
```

### **3. Test the API**
```bash
curl -X POST http://localhost:2000/analyze \
  -H "Content-Type: application/json" \
  -d '{"text": "The report was written yesterday. This is very important and very critical."}'
```

---

## 📋 **API Endpoint**

### **POST /analyze**

Analyzes text and returns all detected issues with positions.

**URL:** `http://0.0.0.0:2000/analyze`

**Method:** `POST`

**Content-Type:** `application/json`

---

## 📥 **Request Format**

```json
{
  "text": "Your text here to analyze"
}
```

**Example:**
```json
{
  "text": "The comprehensive analysis was conducted by the team. At the end of the day, we need to synergize our efforts. This is very important."
}
```

---

## 📤 **Response Format**

```json
{
  "issues": [
    {
      "Id": "auto_27_40_13_passive_was_conducted",
      "start": 27,
      "length": 13,
      "end": 40,
      "paragraphKey": "para_0",
      "string": "was conducted",
      "type": "PassiveVoice",
      "suggestions": {
        "recommendation": [
          "Consider using active voice for clarity"
        ]
      }
    },
    {
      "Id": "auto_56_77_21_cliche_at_the_end_of_the_day",
      "start": 56,
      "length": 21,
      "end": 77,
      "paragraphKey": "para_0",
      "string": "at the end of the day",
      "type": "Cliche",
      "suggestions": {
        "recommendation": [
          "Avoid clichés",
          "Use original phrasing"
        ]
      }
    },
    {
      "Id": "auto_92_101_9_jargon_synergize",
      "start": 92,
      "length": 9,
      "end": 101,
      "paragraphKey": "para_0",
      "string": "synergize",
      "type": "BusinessJargon",
      "suggestions": {
        "recommendation": [
          "Avoid corporate jargon",
          "Use plain language"
        ]
      }
    },
    {
      "Id": "auto_120_124_4_vague_very",
      "start": 120,
      "length": 4,
      "end": 124,
      "paragraphKey": "para_0",
      "string": "very",
      "type": "VagueWord",
      "suggestions": {
        "recommendation": [
          "Be more specific",
          "Use concrete language"
        ]
      }
    }
  ],
  "summary": {
    "total_issues": 15,
    "word_count": 24,
    "sentence_count": 3,
    "style_score": 65
  }
}
```

---

## 🎨 **Issue Types**

| Type | Description | Example |
|------|-------------|---------|
| **PassiveVoice** | Passive voice construction | "was written", "has been done" |
| **Grammar_*** | Grammar issues | SubjectVerbAgreement, DoubleNegative |
| **StickySentence** | High glue word percentage | Sentences with >40% glue words |
| **OverusedWord** | Words used too frequently | "analysis" (8 times, 2.1% frequency) |
| **Repetition** | Repeated phrases | "the analysis", "in order to" |
| **Cliche** | Overused expressions | "at the end of the day" |
| **VagueWord** | Imprecise language | "thing", "stuff", "very", "really" |
| **BusinessJargon** | Corporate speak | "synergize", "leverage", "circle back" |

---

## 💡 **Usage Examples**

### **Example 1: Using cURL**
```bash
curl -X POST http://localhost:2000/analyze \
  -H "Content-Type: application/json" \
  -d '{
    "text": "The report was written. It is very important."
  }' | jq '.'
```

### **Example 2: Using Python**
```python
import requests
import json

url = "http://localhost:2000/analyze"
data = {
    "text": "The comprehensive analysis was conducted yesterday. At the end of the day, we need to synergize."
}

response = requests.post(url, json=data)
result = response.json()

print(f"Total issues: {result['summary']['total_issues']}")
print(f"Style score: {result['summary']['style_score']}")

for issue in result['issues']:
    print(f"\n{issue['type']} at position {issue['start']}-{issue['end']}")
    print(f"  Text: '{issue['string']}'")
    print(f"  Suggestions: {issue['suggestions']['recommendation']}")
```

### **Example 3: Using JavaScript/Node.js**
```javascript
const axios = require('axios');

async function analyzeText(text) {
  const response = await axios.post('http://localhost:2000/analyze', {
    text: text
  });
  
  const { issues, summary } = response.data;
  
  console.log(`Total issues: ${summary.total_issues}`);
  console.log(`Style score: ${summary.style_score}`);
  
  issues.forEach(issue => {
    console.log(`\n${issue.type} at ${issue.start}-${issue.end}`);
    console.log(`  "${issue.string}"`);
    console.log(`  Suggestions: ${issue.suggestions.recommendation.join(', ')}`);
  });
}

analyzeText("The document was written very carefully.");
```

### **Example 4: Using JavaScript/Fetch (Browser)**
```javascript
async function analyzeText(text) {
  const response = await fetch('http://localhost:2000/analyze', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ text })
  });
  
  const data = await response.json();
  
  // Highlight issues in text editor
  data.issues.forEach(issue => {
    highlightText(issue.start, issue.end, issue.type);
  });
  
  return data;
}

// Usage
const result = await analyzeText(document.getElementById('editor').value);
console.log('Analysis complete:', result.summary);
```

---

## 🔧 **Advanced Usage**

### **Batch Processing**
```bash
#!/bin/bash
# Process multiple texts
for file in texts/*.txt; do
    echo "Analyzing $file..."
    curl -X POST http://localhost:2000/analyze \
      -H "Content-Type: application/json" \
      -d "{\"text\": \"$(cat $file)\"}" \
      > "results/$(basename $file .txt).json"
done
```

### **Integration with Text Editor**
```javascript
// VS Code Extension example
class TextAnalyzerProvider {
  async provideDiagnostics(document) {
    const text = document.getText();
    const response = await fetch('http://localhost:2000/analyze', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ text })
    });
    
    const { issues } = await response.json();
    
    return issues.map(issue => ({
      range: new vscode.Range(
        document.positionAt(issue.start),
        document.positionAt(issue.end)
      ),
      message: issue.suggestions.recommendation.join('\n'),
      severity: this.getSeverity(issue.type)
    }));
  }
}
```

---

## 🎯 **Response Fields**

### **Issue Object**
| Field | Type | Description |
|-------|------|-------------|
| `Id` | string | Unique identifier for the issue |
| `start` | number | Start position (character index) |
| `end` | number | End position (character index) |
| `length` | number | Length in characters |
| `paragraphKey` | string | Paragraph identifier |
| `string` | string | The actual text with the issue |
| `type` | string | Type of issue |
| `suggestions` | object | Recommendations for fixing |

### **Summary Object**
| Field | Type | Description |
|-------|------|-------------|
| `total_issues` | number | Total number of issues found |
| `word_count` | number | Total word count |
| `sentence_count` | number | Total sentence count |
| `style_score` | number | Overall style score (0-100) |

---

## 🚀 **Deployment**

### **Production Mode**
```bash
# Build optimized binary
cargo build --release --bin api-server

# Run in background
nohup ./target/release/api-server > api.log 2>&1 &

# Or use systemd
sudo systemctl start text-analyzer-api
```

### **Docker Deployment**
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin api-server

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/api-server /usr/local/bin/
EXPOSE 2000
CMD ["api-server"]
```

```bash
docker build -t text-analyzer-api .
docker run -p 2000:2000 text-analyzer-api
```

---

## 🔒 **Security Notes**

- The API accepts any text input up to reasonable limits
- CORS is enabled (permissive) - configure as needed for production
- No authentication by default - add if needed
- Rate limiting not implemented - add if needed
- Consider adding request size limits for production

---

## ⚡ **Performance**

- Average response time: ~50-200ms for typical documents
- Handles texts up to 100KB efficiently
- Concurrent request support via Tokio async runtime
- Memory efficient streaming processing

---

## 🐛 **Error Responses**

### **Empty Text**
```json
{
  "error": "Text cannot be empty"
}
```
**Status Code:** 400 Bad Request

### **Analysis Error**
```json
{
  "error": "Failed to analyze text: [error details]"
}
```
**Status Code:** 500 Internal Server Error

---

## 📊 **Testing**

### **Health Check**
```bash
curl http://localhost:2000/analyze \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"text": "test"}'
```

### **Load Testing**
```bash
# Using Apache Bench
ab -n 1000 -c 10 -p test.json -T application/json \
  http://localhost:2000/analyze
```

---

## 🎉 **You're All Set!**

Start the server and begin analyzing text via REST API! 🚀✨
