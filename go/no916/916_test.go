package no916

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCount(t *testing.T) {
	g := "google"
	ans := make([]int, 26)
	ans[6] = 2
	ans[14] = 2
	ans[11] = 1
	ans[4] = 1

	res := countChar(g)
	assert.Equal(t, ans, res, "Test: Count google character")
}

func Test1(t *testing.T) {
	words1 := []string{
		"amazon", "apple", "facebook", "google", "leetcode",
	}
	words2 := []string{
		"e", "o",
	}
	assert.Equal(t,
		[]string{"facebook", "google", "leetcode"},
		wordSubsets(words1, words2),
		"Test1: Single Character",
	)
}

func Test2(t *testing.T) {
	words1 := []string{
		"amazon", "apple", "facebook", "google", "leetcode",
	}
	words2 := []string{
		"l", "e",
	}
	assert.Equal(t,
		[]string{"apple", "google", "leetcode"},
		wordSubsets(words1, words2),
		"Test2: Single Character",
	)
}
