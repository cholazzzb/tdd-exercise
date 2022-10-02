package no1155

func numRollsToTarget(n int, k int, target int) int {
	var md int = 1000000007

	if target > n*k {
		return 0
	}
	dp := make([][]int, n)

	dice1 := &dp[0]
	for idx := 0; idx < k; idx++ {
		*dice1 = append(*dice1, 1)
	}

	for dice := 1; dice < n; dice++ {
		diceX := &dp[dice]
		for x := 0; x < n*k; x++ {
			if x < dice {
				*diceX = append(*diceX, 0)
			} else if x == dice {
				*diceX = append(*diceX, 1)
			} else {
				nValue := 0
				for face := 1; face <= k; face++ {
					if x-face >= 0 && x-face < len(dp[dice-1]) {
						nValue += dp[dice-1][x-face]
						nValue %= md
					}
				}
				*diceX = append(*diceX, nValue)
			}
		}
		dp = append(dp, *diceX)
	}

	return dp[n-1][target-1]
}
