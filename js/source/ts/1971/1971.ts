// TAGS: Graph Theory

type GNode = Record<number, number>;

function validPath(
  n: number,
  edges: number[][],
  source: number,
  destination: number
): boolean {
  if (source === destination) return true;

  const graph: Array<GNode> = Array(n)
    .fill(null)
    .map((_) => ({}));
  const visited: Array<boolean> = Array(n).fill(false);

  for (const edge of edges) {
    graph[edge[0]][edge[1]] = edge[1];
    graph[edge[1]][edge[0]] = edge[0];
  }

  let exist = false;
  const dfs = (end: number) => {
    if (end === destination) {
      exist = true;
      return;
    }
    if (visited[end]) return;

    visited[end] = true;
    for (const neighbour of Object.values(graph[end])) {
      dfs(neighbour);
    }
  };

  visited[source] = true;
  for (const neighbour of Object.values(graph[source])) {
    dfs(neighbour);
  }

  return exist;
}

export { validPath };
