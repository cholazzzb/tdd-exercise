function numSquares(n: number): number {
  const dp: Array<number> = Array(n + 1).fill(n);
  dp[1] = 1;

  for (let idx = 1; idx < n + 1; idx++) {
    const root = Math.sqrt(idx);

    if (root % 1 === 0) {
      dp[idx] = 1;
    } else {
      let j = 1;
      let power = j * j;
      while (power <= idx) {
        dp[idx] = Math.min(dp[idx], dp[power] + dp[idx - power]);
        j += 1;
        power = j * j;
      }
    }
  }

  return dp[n];
}

export { numSquares };
