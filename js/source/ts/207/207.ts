class TopologicalSort {
  public size: number;
  public orders: Array<number> = [];
  public isTopologicalSorted = false;

  constructor(graph: Array<Array<number>>, indegree: Array<number>) {
    this.size = graph.length;

    let count = 0;
    const queue: Array<number> = [];
    for (let idx = 0; idx < this.size; idx++) {
      if (indegree[idx] === 0) queue.push(idx);
    }

    while (queue.length > 0) {
      const el = queue.shift() as number;
      this.orders.push(el);
      for (const vertice of graph[el]) {
        if (indegree[vertice] - 1 == 0) {
          indegree[vertice]--;
          queue.push(vertice);
        }
      }

      count++;
    }

    this.isTopologicalSorted = count === this.size;
  }
}

function canFinish(numCourses: number, prerequisites: number[][]): boolean {
  // define the graph outdegree
  const g: Array<Array<number>> = Array(numCourses)
    .fill(null)
    .map((_) => []);
  // define count of indegree
  const indegree = Array(numCourses).fill(0);
  // build the graph
  for (const p of prerequisites) {
    // we have to take p[1] in order to take p[0]
    g[p[1]].push(p[0]);
    // increase indegree by 1 for p[0]
    indegree[p[0]]++;
  }
  // init topological sort
  const ts = new TopologicalSort(g, indegree);
  // we can finish all courses only if we can topologically sort
  return ts.isTopologicalSorted;
}

export { canFinish };
