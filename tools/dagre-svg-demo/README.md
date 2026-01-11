# Dagre → SVG minimal demo

Goal: demonstrate the required architecture **FSM → Graph → Layout Engine → Renderer**, where the **layout engine computes both node positions and edge routes** and the renderer only draws what the engine returns.

## Install

Prerequisite: Node.js (recommended: Node 18+).

```bash
cd tools/dagre-svg-demo
npm install
```

## Run

```bash
node src/fsm_to_svg.mjs example_fsm.json out.svg
```

Open `out.svg` in your browser.

## Notes

- Layout engine: `dagre.layout(graph)` produces:
  - node positions: `node.x`, `node.y`
  - edge routes: `edge.points[]` (polyline bend points)
  - edge-label position: `edge.x`, `edge.y`
- Renderer: this script draws rectangles and paths from those points. No post-layout collision hacks, no lane routing, no manual label offsets.
- Switch direction TB/LR by changing `direction` in the JSON to `"TB"` or `"LR"`.
