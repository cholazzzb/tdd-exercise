md = 1000000007


class Solution:
    def numRollsToTarget(self, n: int, k: int, target: int) -> int:
        if target > n * k:
            return 0

        dp = [[]] * n
        dp[0] = [1] * k

        for idx in range(1, n):
            dp[idx] = [0] * (idx + 1) * k

        for dice in range(1, n):
            for x in range(n * k):
                if x == dice:
                    dp[dice][x] = 1
                elif x > dice:
                    for face in range(1, k + 1):
                        if x - face >= 0 and x - face < len(dp[dice - 1]):
                            dp[dice][x] += dp[dice - 1][x - face]
                            dp[dice][x] %= md

        return dp[n - 1][target - 1]
