#!/bin/bash

# Text Analyzer API - Deploy Script
# This script stops, rebuilds, and restarts the API with PM2

echo "=========================================="
echo "TEXT ANALYZER API - DEPLOYMENT"
echo "=========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
APP_NAME="text-analyzer-api"
BINARY_PATH="./target/release/api-server-enhanced"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Error: Not in text-analyzer directory!${NC}"
    echo "Please run this script from the text-analyzer directory"
    exit 1
fi

# Check if PM2 is installed
if ! command -v pm2 &> /dev/null; then
    echo -e "${RED}❌ PM2 is not installed!${NC}"
    echo "Install it with: npm install -g pm2"
    exit 1
fi

echo -e "${YELLOW}Step 1: Stopping API...${NC}"
pm2 stop $APP_NAME 2>/dev/null || echo "API was not running"
echo -e "${GREEN}✓ Stopped${NC}"
echo ""

echo -e "${YELLOW}Step 2: Cleaning previous builds...${NC}"
cargo clean
echo -e "${GREEN}✓ Cleaned${NC}"
echo ""

echo -e "${YELLOW}Step 3: Building API (this may take 1-2 minutes)...${NC}"
if cargo build --release --bin api-server-enhanced; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}❌ Build failed!${NC}"
    exit 1
fi
echo ""

echo -e "${YELLOW}Step 4: Checking binary...${NC}"
if [ -f "$BINARY_PATH" ]; then
    echo -e "${GREEN}✓ Binary exists${NC}"
    ls -lh $BINARY_PATH
else
    echo -e "${RED}❌ Binary not found!${NC}"
    exit 1
fi
echo ""

echo -e "${YELLOW}Step 5: Creating logs directory...${NC}"
mkdir -p logs
echo -e "${GREEN}✓ Logs directory ready${NC}"
echo ""

echo -e "${YELLOW}Step 6: Restarting API with PM2...${NC}"
# Check if app exists in PM2
if pm2 describe $APP_NAME > /dev/null 2>&1; then
    echo "App exists, restarting..."
    pm2 restart $APP_NAME
else
    echo "App doesn't exist, starting fresh..."
    if [ -f "ecosystem.config.js" ]; then
        pm2 start ecosystem.config.js
    else
        pm2 start $BINARY_PATH --name $APP_NAME
    fi
fi
echo -e "${GREEN}✓ API restarted${NC}"
echo ""

echo -e "${YELLOW}Step 7: Saving PM2 process list...${NC}"
pm2 save
echo -e "${GREEN}✓ Saved${NC}"
echo ""

echo "=========================================="
echo -e "${GREEN}DEPLOYMENT COMPLETE!${NC}"
echo "=========================================="
echo ""
echo "API Status:"
pm2 status $APP_NAME
echo ""
echo "Recent logs:"
pm2 logs $APP_NAME --lines 10 --nostream
echo ""
echo -e "${GREEN}✅ API is running on http://0.0.0.0:2000${NC}"
echo ""
echo "Useful commands:"
echo "  pm2 logs $APP_NAME      - View logs"
echo "  pm2 monit                - Monitor resources"
echo "  pm2 restart $APP_NAME   - Restart API"
echo "  pm2 stop $APP_NAME      - Stop API"
echo ""
