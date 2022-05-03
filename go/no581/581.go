package no581

import "math"

func findUnsortedSubarray(nums []int) int {
	left := int(math.Inf(1))
	right := int(math.Inf(-1))
	minPrev := nums[len(nums)-1]
	maxPrev := nums[0]

	for i, j := 1, len(nums)-2; i < len(nums); i, j = i+1, j-1 {
		iNum := nums[i]
		jNum := nums[j]

		if iNum < maxPrev {
			right = i
		}
		maxPrev = getMax(maxPrev, iNum)

		if jNum > minPrev {
			left = j
		}
		minPrev = getMin(minPrev, jNum)
	}

	if left < right {
		return right - left + 1
	} else {
		return 0
	}
}

func getMax(a, b int) int {
	if a > b {
		return a
	} else {
		return b
	}
}

func getMin(a, b int) int {
	if a < b {
		return a
	} else {
		return b
	}
}
