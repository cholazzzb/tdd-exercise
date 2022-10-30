package no2424

import (
	"github.com/emirpasic/gods/sets/treeset"
)

type LUPrefix struct {
	size int
	set  *treeset.Set
}

func Constructor(n int) LUPrefix {
	set := treeset.NewWithIntComparator() // empty (keys are of type int)
	for i := 1; i <= n; i++ {
		set.Add(i)
	}
	return LUPrefix{n, set}
}

func (lup *LUPrefix) Upload(video int) {
	lup.set.Remove(video)
}

func (lup *LUPrefix) Longest() int {
	iter := lup.set.Iterator()
	iter.Next()
	if lup.set.Size() > 0 {
		return iter.Value().(int) - 1
	}
	return lup.size
}

/**
 * Your LUPrefix object will be instantiated and called as such:
 * obj := Constructor(n);
 * obj.Upload(video);
 * param_2 := obj.Longest();
 */
