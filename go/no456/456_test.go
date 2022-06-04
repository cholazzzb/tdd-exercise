package no456

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test1(t *testing.T) {
	nums := []int{1, 3, 2}
	assert.Equal(t, true, find132pattern(nums), "1")
}

func Test2(t *testing.T) {
	nums := []int{1, 2, 3, 3, 2, 0}
	assert.Equal(t, true, find132pattern(nums), "2")
}

func Test3(t *testing.T) {
	nums := []int{3, 5, 0, 3, 4}
	assert.Equal(t, true, find132pattern(nums), "3")
}
