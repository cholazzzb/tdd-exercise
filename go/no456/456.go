package no456

func find132pattern(nums []int) bool {
	min := nums[0]
	for idx := 1; idx < len(nums)-1; idx++ {
		mid := nums[idx]
		left := getMin(min, nums[idx-1])
		right := nums[idx+1]
		if mid > right && right > left {
			return true
		}
	}
	return false
}

func getMin(a, b int) int {
	if a < b {
		return a
	}
	return b
}
