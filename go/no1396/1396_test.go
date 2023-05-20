package no1396_test

import (
	"testing"

	"github.com/cholazzzb/tdd-exercise/tree/main/go/no1396"
	"github.com/stretchr/testify/assert"
)

func Test1(t *testing.T) {
	obj := no1396.Constructor()
	obj.CheckIn(45, "Leyton", 3)
	obj.CheckIn(32, "Paradise", 8)
	obj.CheckIn(27, "Leyton", 10)
	obj.CheckOut(45, "Waterloo", 15)  // Customer 45 "Leyton" -> "Waterloo" in 15-3 = 12
	obj.CheckOut(27, "Waterloo", 20)  // Customer 27 "Leyton" -> "Waterloo" in 20-10 = 10
	obj.CheckOut(32, "Cambridge", 22) // Customer 32 "Paradise" -> "Cambridge" in 22-8 = 14
	res1 := obj.GetAverageTime("Paradise", "Cambridge")
	assert.Equal(t, 14.000, res1, "should return 14.00000. One trip Paradise -> Cambridge, (14) / 1 = 14")
	res2 := obj.GetAverageTime("Leyton", "Waterloo")
	assert.Equal(t, 11.00000, res2, "should return 11.00000. Two trips Leyton -> Waterloo, (10 + 12) / 2 = 11")
	obj.CheckIn(10, "Leyton", 24)
	res3 := obj.GetAverageTime("Leyton", "Waterloo")
	assert.Equal(t, 11.00000, res3, "should return  11.00000")
	obj.CheckOut(10, "Waterloo", 38) // Customer 10 "Leyton" -> "Waterloo" in 38-24 = 14
	res4 := obj.GetAverageTime("Leyton", "Waterloo")
	assert.Equal(t, 12.00000, res4, "return 12.00000. Three trips Leyton -> Waterloo, (10 + 12 + 14) / 3 = 12")
}

func Test2(t *testing.T) {

}
