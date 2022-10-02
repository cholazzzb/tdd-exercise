const mod = 10 ** 9 + 7;

function numRollsToTarget(n: number, k: number, target: number): number {
  if (target > n * k) {
    return 0;
  }

  const dp: Array<Array<number>> = Array(n)
    .fill(0)
    .map((_) => []);

  for (let idx = 0; idx < k; idx++) {
    dp[0][idx] = 1;
  }

  for (let dice = 1; dice < n; dice++) {
    for (let x = 0; x < n * k; x++) {
      if (x < dice) {
        dp[dice][x] = 0;
      } else if (x === dice) {
        dp[dice][x] = 1;
      } else {
        dp[dice][x] = 0;
        for (let f = 1; f <= k; f++) {
          if (dp[dice - 1][x - f]) {
            dp[dice][x] += dp[dice - 1][x - f];
            dp[dice][x] %= mod;
          }
        }
      }
    }
  }

  return dp[n - 1][target - 1];
}

export { numRollsToTarget };
