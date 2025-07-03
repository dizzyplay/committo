#!/bin/bash
set -euo pipefail

echo "🔨 Building committo..."
cargo build --quiet --release

# 스크립트 절대경로와 바이너리 경로 확보 (cd 전에!)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
BIN_PATH="$SCRIPT_DIR/../target/release/committo"

if [ ! -x "$BIN_PATH" ]; then
  echo "❌ 빌드된 committo 바이너리를 찾지 못했습니다: $BIN_PATH" >&2
  exit 1
fi

echo "🧪 Running smoke tests (unit/integration)..."
cargo test --quiet --test smoke_test

echo "🎯 Running manual hierarchical convention test..."
TEMP_DIR=$(mktemp -d)
echo "Created temp dir: $TEMP_DIR"
trap 'rm -rf "$TEMP_DIR"' EXIT        # 스크립트 종료 시 자동 정리

cd "$TEMP_DIR"
mkdir -p project/frontend

# Create hierarchical convention files
echo "Please write commit messages in Korean." > .committoconvention
echo "For partial modifications use UPDATE prefix, for new features use ADD prefix, for bug fixes use FIX prefix. Follow with : and a space." > project/.committoconvention

# Initialize git repo in project directory
cd project
git init -q
git config user.name  "Test User"
git config user.email "test@example.com"

# Create and stage a file in frontend subdirectory
echo "const App = () => <div>Hello</div>;" > frontend/App.js
git add frontend/App.js

echo "📝 Testing hierarchical conventions (should show both prompts)..."
cd frontend
"$BIN_PATH" generate --dry-run

echo "✅ Smoke test completed!"
