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

echo "🎯 Running quick manual test..."
TEMP_DIR=$(mktemp -d)
echo "Created temp dir: $TEMP_DIR"
trap 'rm -rf "$TEMP_DIR"' EXIT        # 스크립트 종료 시 자동 정리

cd "$TEMP_DIR"
echo "Please suggest an appropriate git commit message as instructed below." >> .committoconvention
git init -q
git config user.name  "Test User"
git config user.email "test@example.com"

echo "feat: add new feature" > test.txt
git add test.txt

echo "📝 Testing dry-run..."
"$BIN_PATH" generate --dry-run

echo "✅ Smoke test completed!"
