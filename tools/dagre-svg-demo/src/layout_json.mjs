import fs from 'node:fs';
import dagre from 'dagre';

// Reads a JSON graph from stdin and prints a JSON layout to stdout.
//
// Input format:
// {
//   "graph": { "rankdir": "tb"|"lr", "nodesep": number, "ranksep": number, "edgesep": number, "marginx": number, "marginy": number },
//   "nodes": [ {"id": string, "width": number, "height": number} ],
//   "edges": [ {"v": string, "w": string, "name"?: string|null, "labelWidth"?: number, "labelHeight"?: number} ]
// }
//
// Output format:
// {
//   "graph": {"width": number, "height": number},
//   "nodes": { [id]: {"x": number, "y": number, "width": number, "height": number} },
//   "edges": [ {"v": string, "w": string, "name": string|null, "points": [ {"x": number, "y": number} ], "x": number|null, "y": number|null } ]
// }

const inputText = fs.readFileSync(0, 'utf8');
if (!inputText.trim()) {
  console.error('No input received on stdin');
  process.exit(2);
}

const input = JSON.parse(inputText);
const graphCfg = input.graph ?? {};

const g = new dagre.graphlib.Graph({ multigraph: true, compound: false });

g.setGraph({
  rankdir: graphCfg.rankdir ?? 'tb',
  nodesep: graphCfg.nodesep ?? 50,
  ranksep: graphCfg.ranksep ?? 50,
  edgesep: graphCfg.edgesep ?? 20,
  marginx: graphCfg.marginx ?? 0,
  marginy: graphCfg.marginy ?? 0,
});

g.setDefaultEdgeLabel(() => ({}));

for (const n of input.nodes ?? []) {
  g.setNode(n.id, { width: n.width ?? 10, height: n.height ?? 10 });
}

for (const e of input.edges ?? []) {
  const edgeLabel = {
    width: e.labelWidth ?? 0,
    height: e.labelHeight ?? 0,
    labelpos: 'c',
    labeloffset: 0,
  };
  // For multigraph, dagre.graphlib expects the 4th argument as the edge name.
  if (e.name != null) {
    g.setEdge(e.v, e.w, edgeLabel, e.name);
  } else {
    g.setEdge(e.v, e.w, edgeLabel);
  }
}

dagre.layout(g);

const outNodes = {};
for (const id of g.nodes()) {
  const n = g.node(id);
  outNodes[id] = { x: n.x, y: n.y, width: n.width, height: n.height };
}

const outEdges = [];
for (const e of g.edges()) {
  const ed = g.edge(e);
  outEdges.push({
    v: e.v,
    w: e.w,
    name: e.name ?? null,
    points: ed.points ?? [],
    x: ed.x ?? null,
    y: ed.y ?? null,
  });
}

const gg = g.graph();
const outGraph = {
  width: gg.width ?? 0,
  height: gg.height ?? 0,
};

process.stdout.write(JSON.stringify({ graph: outGraph, nodes: outNodes, edges: outEdges }));
