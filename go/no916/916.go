package no916

func wordSubsets(words1 []string, words2 []string) []string {
	sub := [26]int{}
	for _, word := range words2 {
		temp := countChar(word)
		for i := 0; i < 26; i++ {
			sub[i] = getMax(sub[i], temp[i])
		}
	}

	res := []string{}
	for _, word := range words1 {
		priv_sub := countChar(word)
		i := 0
		for ; i < 26; i++ {
			if priv_sub[i] < sub[i] {
				break
			}
		}
		if i == 26 {
			res = append(res, word)
		}
	}
	return res
}

func countChar(s string) [26]int {
	c := [26]int{}
	for _, ch := range s {
		c[ch-'a']++
	}
	return c
}

func getMax(a, b int) int {
	if a > b {
		return a
	}
	return b
}
