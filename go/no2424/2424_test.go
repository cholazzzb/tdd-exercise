package no2424

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test1(t *testing.T) {
	var longest int
	obj := Constructor(4)
	obj.Upload(3)
	longest = obj.Longest()
	assert.Equal(t, 0, longest, "1")
	obj.Upload(1)
	longest = obj.Longest()
	assert.Equal(t, 1, longest, "2")
	obj.Upload(2)
	longest = obj.Longest()
	assert.Equal(t, 3, longest, "3")
}
