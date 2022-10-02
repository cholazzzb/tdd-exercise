import unittest
from sol_1155 import Solution

sol = Solution()


class TestStringMethods(unittest.TestCase):
    def test_1(self):
        self.assertEqual(sol.numRollsToTarget(1, 6, 3), 1)

    def test_2(self):
        self.assertEqual(sol.numRollsToTarget(8, 6, 3), 0)

    def test_3(self):
        self.assertEqual(sol.numRollsToTarget(2, 6, 7), 6)

    def test_4(self):
        self.assertEqual(sol.numRollsToTarget(5, 6, 15), 651)

    def test_5(self):
        self.assertEqual(sol.numRollsToTarget(30, 30, 500), 222616187)


if __name__ == "__main__":
    unittest.main()
