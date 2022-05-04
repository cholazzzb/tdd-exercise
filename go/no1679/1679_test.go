package no1679

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test1(t *testing.T) {
	nums := []int{1, 2, 3, 4}
	k := 5
	assert.Equal(t, 2, maxOperations(nums, k), "1")
}

func Test2(t *testing.T) {
	nums := []int{3, 1, 3, 4, 3}
	k := 6
	assert.Equal(t, 1, maxOperations(nums, k), "2")
}

func Test3(t *testing.T) {
	nums := []int{2, 2, 1, 1}
	k := 3
	assert.Equal(t, 2, maxOperations(nums, k), "3")
}

func Test4(t *testing.T) {
	nums := []int{2, 5, 4, 4, 1, 3, 4, 4, 1, 4, 4, 1, 2, 1, 2, 2, 3, 2, 4, 2}
	k := 3
	assert.Equal(t, 4, maxOperations(nums, k), "4")
}
