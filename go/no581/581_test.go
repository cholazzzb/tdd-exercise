package no581

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test1(t *testing.T) {
	q1 := []int{2, 6, 4, 8, 10, 9, 15}
	assert.Equal(t, 5, findUnsortedSubarray(q1), "1")
}
func Test2(t *testing.T) {
	q2 := []int{1, 2, 3, 4}
	assert.Equal(t, 0, findUnsortedSubarray(q2), "2")
}
func Test3(t *testing.T) {
	q3 := []int{1, 3, 2, 2, 2}
	assert.Equal(t, 4, findUnsortedSubarray(q3), "3")
}
func Test4(t *testing.T) {
	q4 := []int{1}
	assert.Equal(t, 0, findUnsortedSubarray(q4), "4")
}
func Test5(t *testing.T) {
	q5 := []int{5, 4, 3, 2, 1}
	assert.Equal(t, 5, findUnsortedSubarray(q5), "5")
}
func Test6(t *testing.T) {
	q6 := []int{1, 3, 2, 3, 3}
	assert.Equal(t, 2, findUnsortedSubarray(q6), "6")
}
func Test7(t *testing.T) {
	q7 := []int{1, 2, 4, 5, 3}
	assert.Equal(t, 3, findUnsortedSubarray(q7), "7")
}
