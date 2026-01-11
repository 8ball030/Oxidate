import fs from 'node:fs/promises';
import dagre from 'dagre';

function escapeXml(text) {
  return String(text)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&apos;');
}

function svgHeader({ width, height, viewBox }) {
  return `<?xml version="1.0" encoding="UTF-8"?>\n` +
    `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="${viewBox}">\n` +
    `  <defs>\n` +
    `    <marker id="arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse">\n` +
    `      <path d="M 0 0 L 10 5 L 0 10 z" fill="#586174"/>\n` +
    `    </marker>\n` +
    `  </defs>\n`;
}

function svgFooter() {
  return `</svg>\n`;
}

function computeBounds(nodes, edges) {
  let minX = Number.POSITIVE_INFINITY;
  let minY = Number.POSITIVE_INFINITY;
  let maxX = Number.NEGATIVE_INFINITY;
  let maxY = Number.NEGATIVE_INFINITY;

  for (const n of nodes) {
    const x0 = n.x - n.width / 2;
    const y0 = n.y - n.height / 2;
    const x1 = n.x + n.width / 2;
    const y1 = n.y + n.height / 2;
    minX = Math.min(minX, x0);
    minY = Math.min(minY, y0);
    maxX = Math.max(maxX, x1);
    maxY = Math.max(maxY, y1);
  }

  for (const e of edges) {
    for (const p of e.points ?? []) {
      minX = Math.min(minX, p.x);
      minY = Math.min(minY, p.y);
      maxX = Math.max(maxX, p.x);
      maxY = Math.max(maxY, p.y);
    }
    if (Number.isFinite(e.x) && Number.isFinite(e.y)) {
      minX = Math.min(minX, e.x);
      minY = Math.min(minY, e.y);
      maxX = Math.max(maxX, e.x);
      maxY = Math.max(maxY, e.y);
    }
  }

  if (!Number.isFinite(minX)) {
    minX = 0; minY = 0; maxX = 100; maxY = 100;
  }

  const pad = 40;
  return {
    minX: minX - pad,
    minY: minY - pad,
    maxX: maxX + pad,
    maxY: maxY + pad,
  };
}

function pointsToPath(points) {
  if (!points || points.length === 0) return '';
  const [first, ...rest] = points;
  return `M ${first.x} ${first.y} ` + rest.map(p => `L ${p.x} ${p.y}`).join(' ');
}

async function main() {
  const [inputPath, outputPath] = process.argv.slice(2);
  if (!inputPath || !outputPath) {
    console.error('Usage: node src/fsm_to_svg.mjs <fsm.json> <out.svg>');
    process.exit(2);
  }

  const raw = await fs.readFile(inputPath, 'utf8');
  const fsm = JSON.parse(raw);

  const g = new dagre.graphlib.Graph({ multigraph: false, compound: false });
  g.setDefaultEdgeLabel(() => ({}));

  g.setGraph({
    rankdir: fsm.direction ?? 'TB',
    nodesep: 50,
    ranksep: 70,
    edgesep: 10,
    marginx: 20,
    marginy: 20,
  });

  for (const node of fsm.nodes ?? []) {
    if (!node?.id) throw new Error('Node missing id');
    g.setNode(node.id, {
      label: node.label ?? node.id,
      width: node.width ?? 120,
      height: node.height ?? 60,
    });
  }

  for (const edge of fsm.edges ?? []) {
    if (!edge?.from || !edge?.to) throw new Error('Edge missing from/to');
    g.setEdge(edge.from, edge.to, {
      label: edge.label ?? '',
      width: 0,
      height: 0,
    });
  }

  dagre.layout(g);

  const laidOutNodes = g.nodes().map((id) => {
    const n = g.node(id);
    return {
      id,
      label: n.label ?? id,
      x: n.x,
      y: n.y,
      width: n.width,
      height: n.height,
    };
  });

  const laidOutEdges = g.edges().map((e) => {
    const ed = g.edge(e);
    return {
      from: e.v,
      to: e.w,
      label: ed.label ?? '',
      points: ed.points ?? [],
      x: ed.x,
      y: ed.y,
    };
  });

  const bounds = computeBounds(laidOutNodes, laidOutEdges);
  const width = Math.max(300, Math.ceil(bounds.maxX - bounds.minX));
  const height = Math.max(200, Math.ceil(bounds.maxY - bounds.minY));
  const viewBox = `${bounds.minX} ${bounds.minY} ${bounds.maxX - bounds.minX} ${bounds.maxY - bounds.minY}`;

  let svg = '';
  svg += svgHeader({ width, height, viewBox });

  // Edges first (behind nodes)
  for (const e of laidOutEdges) {
    const d = pointsToPath(e.points);
    if (d) {
      svg += `  <path d="${d}" fill="none" stroke="#586174" stroke-width="2" marker-end="url(#arrow)"/>\n`;
    }
    if (e.label && Number.isFinite(e.x) && Number.isFinite(e.y)) {
      svg += `  <text x="${e.x}" y="${e.y}" text-anchor="middle" dominant-baseline="central" font-family="ui-sans-serif, system-ui" font-size="12" fill="#2b2f38">${escapeXml(e.label)}</text>\n`;
    }
  }

  // Nodes on top
  for (const n of laidOutNodes) {
    const x = n.x - n.width / 2;
    const y = n.y - n.height / 2;
    svg += `  <rect x="${x}" y="${y}" width="${n.width}" height="${n.height}" rx="10" ry="10" fill="#111827" stroke="#9ca3af" stroke-width="2"/>\n`;
    svg += `  <text x="${n.x}" y="${n.y}" text-anchor="middle" dominant-baseline="central" font-family="ui-sans-serif, system-ui" font-size="14" fill="#f9fafb">${escapeXml(n.label)}</text>\n`;
  }

  svg += svgFooter();

  await fs.writeFile(outputPath, svg, 'utf8');
  console.error(`Wrote ${outputPath}`);
}

main().catch((err) => {
  console.error(err?.stack ?? String(err));
  process.exit(1);
});
