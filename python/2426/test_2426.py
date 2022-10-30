import unittest
from sol_2426 import Solution

sol = Solution()


class TestNumberOfPairs(unittest.TestCase):
    def test_1(self):
        nums1 = [3, 2, 5]
        nums2 = [2, 2, 1]
        diff = 1
        self.assertEqual(sol.numberOfPairs(nums1, nums2, diff), 3)

    def test_2(self):
        nums1 = [3, -1]
        nums2 = [-2, 2]
        diff = -1
        self.assertEqual(sol.numberOfPairs(nums1, nums2, diff), 0)
