"""
question constraints:
1. nums1[i] - nums1[j] <= nums2[i] - nums2[j] + diff
2. i < j

become
1. nums1[i] - nums2[i] <= nums1[j] - nums2[j] + diff
2. i < j

solution:
create array a where
a[i] = nums1[i] - nums2[i]
find pair (i, j) such that i < j and a[i] <= a[j] + diff
"""

from sortedcontainers import SortedList


class Solution:
    def numberOfPairs(self, nums1, nums2, diff: int) -> int:
        l = SortedList()
        ans = 0
        for j in range(len(nums1)):
            cur = nums1[j] - nums2[j]  # a[i]
            ans += l.bisect_left(
                cur + diff + 1
            )  # how many elements in l are <= cur+diff
            l.add(cur)
        return ans
