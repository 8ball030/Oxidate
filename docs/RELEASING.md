# Oxidate — packaging & releases

Oxidate uses a JS Dagre layout backend executed via a Node.js subprocess (`tools/dagre-svg-demo/src/layout_json.mjs`).

At runtime the app locates the layout backend via:
- `OXIDATE_DAGRE_DIR` (optional override)
- Bundled resources (macOS `.app`)
- `resources/tools/dagre-svg-demo` next to the executable (portable builds)
- `tools/dagre-svg-demo` next to the executable (dev/portable)
- `/usr/share/oxidate/tools/dagre-svg-demo` (Linux packages)

Node.js is located via:
- `OXIDATE_NODE` (optional override)
- Bundled `Resources/node/bin/node` (macOS, if you choose to ship it)
- A few conventional "next to exe" locations (portable builds)
- Otherwise falls back to `node` on `PATH`

## macOS (.app)

Prereqs:
- Rust toolchain
- `cargo-bundle`: `cargo install cargo-bundle`
- Layout deps: `cd tools/dagre-svg-demo && npm install`

Build:
- `cargo bundle --release`

Output:
- `target/release/bundle/osx/Oxidate.app`

## Linux (.deb)

Prereqs:
- `cargo-deb`: `cargo install cargo-deb`
- Layout deps: `cd tools/dagre-svg-demo && npm install`

Build:
- `cargo build --release`
- `cargo deb`

Output:
- `target/debian/*.deb`

Notes:
- The `.deb` declares a dependency on `nodejs`.

## Linux (portable zip/AppImage-style layout)

If you want a portable folder/zip:
- Easiest: `bash tools/package/make-portable.sh`

Manual layout:
- Build: `cargo build --release`
- Create a folder like:
  - `oxidate/oxidate` (binary)
  - `oxidate/resources/tools/dagre-svg-demo` (copy from repo)
- Ensure Node is available (either system `node` on PATH, or bundle it and set `OXIDATE_NODE`).

For a true AppImage, you can use tools like `cargo-appimage` or a custom AppDir + `appimagetool` pipeline; the important part is that the AppDir should install the backend at:
- `usr/share/oxidate/tools/dagre-svg-demo`

## Windows

Recommended:
- Portable `.zip` first (simplest), MSI later.

Portable zip layout:
- `oxidate.exe`
- `resources\tools\dagre-svg-demo\...`

Node:
- Either require users to install Node.js, or ship a private copy and set `OXIDATE_NODE` via a launcher.
