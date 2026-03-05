set -e
OUT_DIR="site/api"
TEMP_DIR="temp-doc"
mkdir -p "$OUT_DIR"
cargo doc --target-dir "$TEMP_DIR"
cp -r "$TEMP_DIR/doc/"* "$OUT_DIR"
rm -rf "$TEMP_DIR"