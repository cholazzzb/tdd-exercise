package no1155

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test1(t *testing.T) {
	n := 1
	k := 6
	target := 3
	assert.Equal(t, 1, numRollsToTarget(n, k, target), "test1")
}
func Test2(t *testing.T) {
	n := 8
	k := 6
	target := 3
	assert.Equal(t, 0, numRollsToTarget(n, k, target), "test1")
}

func Test3(t *testing.T) {
	n := 2
	k := 6
	target := 7
	assert.Equal(t, 6, numRollsToTarget(n, k, target), "test1")
}

func Test4(t *testing.T) {
	n := 5
	k := 6
	target := 15
	assert.Equal(t, 651, numRollsToTarget(n, k, target), "test1")
}

func Test5(t *testing.T) {
	n := 30
	k := 30
	target := 500
	assert.Equal(t, 222616187, numRollsToTarget(n, k, target), "test1")
}
