#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

version="$(grep -E '^version\s*=\s*"' Cargo.toml | head -n1 | sed -E 's/.*"([^"]+)".*/\1/')"
target="$(rustc -Vv | sed -n 's/^host: //p' | head -n1)"

out_dir="$repo_root/dist/oxidate-$version-$target"

# Ensure the JS layout backend is ready (node_modules present) before copying.
# If you prefer CI-style reproducibility, run: (cd tools/dagre-svg-demo && npm ci)

cargo build --release --target "$target"

mkdir -p "$out_dir/resources/tools"

cp -f "$repo_root/target/$target/release/oxidate" "$out_dir/" || cp -f "$repo_root/target/release/oxidate" "$out_dir/"

# Bundle the Dagre backend (including node_modules if installed)
rm -rf "$out_dir/resources/tools/dagre-svg-demo"
cp -R "$repo_root/tools/dagre-svg-demo" "$out_dir/resources/tools/dagre-svg-demo"

cat > "$out_dir/README.txt" <<EOF
Oxidate $version ($target)

This portable build expects Node.js available as `node` on PATH.
If you bundle Node yourself, set OXIDATE_NODE to its full path.

Optional overrides:
- OXIDATE_DAGRE_DIR: path to tools/dagre-svg-demo
- OXIDATE_NODE: path to Node.js executable
EOF

echo "Portable folder created at: $out_dir"