package no1679

func maxOperations(nums []int, k int) int {
	hashMap := make(map[int]int)
	res := 0
	for _, num := range nums {
		val := hashMap[num]
		if val > 0 {
			res += 1
			hashMap[num] -= 1
		} else {
			_, ok := hashMap[k-num]
			if ok {
				hashMap[k-num] += 1
			} else {
				hashMap[k-num] = 1
			}
		}
	}
	return res
}
