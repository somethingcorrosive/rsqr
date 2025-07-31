#!/usr/bin/env bash
set -euo pipefail

BIN="./target/debug/rsqr"
TEST_PNG="test_output.png"

echo "🧪 Running QA for rsqr..."

echo "1. Basic run"
$BIN "hello world"

echo "2. Inverted"
$BIN "hello world" --invert

echo "3. No quiet zone"
$BIN "tight" --no-quiet

echo "4. Charsets (hash and dot)"
$BIN "hashed" --charset hash
$BIN "dotted" --charset dot

echo "5. PNG export"
rm -f "$TEST_PNG"
$BIN "https://example.com" --png "$TEST_PNG"
test -f "$TEST_PNG" && echo "✅ PNG created: $TEST_PNG"

echo "6. PNG export with scale"
$BIN "scaled" --png scaled_output.png --scale 20
test -f "scaled_output.png" && echo "✅ PNG created with scaling"

echo "7. All features together"
$BIN "full stack test" --invert --no-quiet --charset block --png full_combo.png --scale 16

echo "8. Easter egg"
$BIN --me

echo "9. Error case (missing input)"
ERR_OUT=$($BIN 2>&1 || true)
echo "$ERR_OUT"

if echo "$ERR_OUT" | grep -q "Please provide input"; then
    echo "✅ Correct error for missing input"
else
    echo "❌ Error handling failed"
    exit 1
fi

echo "10. Version and help"
$BIN --version
$BIN --help

echo "🎉 All QA tests completed!"
