#!/bin/bash

# Text Analyzer API - First Time PM2 Setup
# Run this script the first time you set up PM2

echo "=========================================="
echo "TEXT ANALYZER API - PM2 FIRST TIME SETUP"
echo "=========================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Configuration
APP_NAME="text-analyzer-api"
BINARY_PATH="./target/release/api-server-enhanced"

# Step 1: Check PM2
echo -e "${YELLOW}Step 1: Checking PM2...${NC}"
if ! command -v pm2 &> /dev/null; then
    echo -e "${RED}❌ PM2 is not installed${NC}"
    echo ""
    echo "Installing PM2 globally..."
    if npm install -g pm2; then
        echo -e "${GREEN}✓ PM2 installed${NC}"
    else
        echo -e "${RED}❌ Failed to install PM2${NC}"
        echo "Please install manually: npm install -g pm2"
        exit 1
    fi
else
    echo -e "${GREEN}✓ PM2 is installed (version: $(pm2 --version))${NC}"
fi
echo ""

# Step 2: Check directory
echo -e "${YELLOW}Step 2: Checking directory...${NC}"
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Not in text-analyzer directory!${NC}"
    exit 1
fi
echo -e "${GREEN}✓ In correct directory${NC}"
echo ""

# Step 3: Build
echo -e "${YELLOW}Step 3: Building API...${NC}"
if [ ! -f "$BINARY_PATH" ]; then
    echo "Binary not found, building..."
    if cargo build --release --bin api-server-enhanced; then
        echo -e "${GREEN}✓ Build successful${NC}"
    else
        echo -e "${RED}❌ Build failed${NC}"
        exit 1
    fi
else
    echo -e "${GREEN}✓ Binary already exists${NC}"
    echo "If you want to rebuild, run: cargo build --release --bin api-server-enhanced"
fi
echo ""

# Step 4: Create logs directory
echo -e "${YELLOW}Step 4: Creating logs directory...${NC}"
mkdir -p logs
echo -e "${GREEN}✓ Created${NC}"
echo ""

# Step 5: Update ecosystem config
echo -e "${YELLOW}Step 5: Checking ecosystem config...${NC}"
if [ -f "ecosystem.config.js" ]; then
    CURRENT_DIR=$(pwd)
    if grep -q "$CURRENT_DIR" ecosystem.config.js; then
        echo -e "${GREEN}✓ Ecosystem config looks good${NC}"
    else
        echo -e "${YELLOW}⚠ Warning: Update 'cwd' in ecosystem.config.js to:${NC}"
        echo "  $CURRENT_DIR"
    fi
else
    echo -e "${YELLOW}⚠ Warning: ecosystem.config.js not found${NC}"
    echo "Using direct PM2 start command instead"
fi
echo ""

# Step 6: Start with PM2
echo -e "${YELLOW}Step 6: Starting API with PM2...${NC}"

# Check if already running
if pm2 describe $APP_NAME > /dev/null 2>&1; then
    echo "App already exists in PM2"
    echo "Do you want to restart it? (y/n)"
    read -r response
    if [[ "$response" =~ ^[Yy]$ ]]; then
        pm2 restart $APP_NAME
    fi
else
    if [ -f "ecosystem.config.js" ]; then
        pm2 start ecosystem.config.js
    else
        pm2 start $BINARY_PATH --name $APP_NAME \
            --log ./logs/combined.log \
            --error ./logs/err.log \
            --output ./logs/out.log \
            --time
    fi
fi
echo -e "${GREEN}✓ Started${NC}"
echo ""

# Step 7: Save PM2 list
echo -e "${YELLOW}Step 7: Saving PM2 process list...${NC}"
pm2 save
echo -e "${GREEN}✓ Saved${NC}"
echo ""

# Step 8: Setup auto-start (optional)
echo -e "${YELLOW}Step 8: Auto-start on system boot${NC}"
echo "Do you want the API to auto-start on system reboot? (y/n)"
read -r response
if [[ "$response" =~ ^[Yy]$ ]]; then
    echo ""
    echo "Setting up startup script..."
    pm2 startup
    echo ""
    echo -e "${YELLOW}⚠ IMPORTANT:${NC}"
    echo "PM2 has shown you a command above. Please copy and run it."
    echo "After running that command, execute: pm2 save"
    echo ""
else
    echo "Skipped auto-start setup"
fi
echo ""

# Step 9: Show status
echo "=========================================="
echo -e "${GREEN}SETUP COMPLETE!${NC}"
echo "=========================================="
echo ""
echo "API Status:"
pm2 status
echo ""
echo "Recent logs:"
pm2 logs $APP_NAME --lines 10 --nostream
echo ""
echo -e "${GREEN}✅ API is running on http://0.0.0.0:2000${NC}"
echo ""
echo "📝 Useful commands:"
echo "  pm2 status               - Check status"
echo "  pm2 logs $APP_NAME      - View logs"
echo "  pm2 monit                - Monitor resources"
echo "  pm2 restart $APP_NAME   - Restart"
echo "  pm2 stop $APP_NAME      - Stop"
echo "  pm2 delete $APP_NAME    - Remove from PM2"
echo ""
echo "🔄 When you update code:"
echo "  ./deploy.sh              - Rebuild and restart"
echo ""
echo "🧪 Test the API:"
echo "  curl -X POST http://localhost:2000/analyze \\"
echo "    -H 'Content-Type: application/json' \\"
echo "    -d '{\"text\": \"Test\"}'"
echo ""
