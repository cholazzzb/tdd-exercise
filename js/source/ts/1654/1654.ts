interface GNode {
  index: number;
  isBackJump: boolean;
  jumps: number;
}

function generateHash(node: GNode): number {
  return node.index * (node.isBackJump ? -1 : 1);
}

function minimumJumps(
  forbidden: number[],
  forwardSteps: number,
  backwardSteps: number,
  endIndex: number
): number {
  const forbiddenIndexes = new Set(forbidden);
  const visitedNodes = new Set<number>();
  const outOfBoundsIndex = Math.min(
    forwardSteps * backwardSteps + endIndex,
    10000
  );

  const startNode: GNode = {
    index: 0,
    isBackJump: false,
    jumps: 0,
  };

  const queue: GNode[] = new Array(startNode);

  while (queue.length > 0) {
    const { index, isBackJump, jumps } = queue.shift() as GNode;

    // Reached the end
    if (index === endIndex) {
      return jumps;
    }

    const nextNodes = [
      {
        index: index + forwardSteps,
        isBackJump: false,
        jumps: jumps + 1,
      },
      {
        index: index - backwardSteps,
        isBackJump: true,
        jumps: jumps + 1,
      },
    ];

    nextNodes
      .filter((node) => {
        return (
          !forbiddenIndexes.has(node.index) &&
          !visitedNodes.has(generateHash(node)) &&
          !(isBackJump && node.isBackJump) &&
          node.index >= 0 &&
          node.index <= outOfBoundsIndex
        );
      })
      .forEach((node) => {
        visitedNodes.add(generateHash(node));
        queue.push(node);
      });
  }

  return -1;
}

export { minimumJumps };
