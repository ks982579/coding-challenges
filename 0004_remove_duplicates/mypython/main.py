"""
Solution to challenge

This solution, compared to leetcode, performs terrible at runtime in terms of speed.
However, performs at 99.62% better that others in terms of memory. 
Every cloud...
"""

from collections import OrderedDict
from typing import List
import unittest

class Solution:
    @staticmethod
    def remove_duplicates_good_memory(nums: List[int]) -> int:
        unique_nums = []
        for i in nums:
            if i in unique_nums:
                continue
            else:
                # nums[len(unique_nums)] = i
                unique_nums.append(i)
        nums[:] = unique_nums
        return len(unique_nums)

    @staticmethod
    def remove_duplicates_better_runtime(nums: List[int]) -> int:
        unique_list = list(OrderedDict.fromkeys(nums))
        nums[:] = unique_list
        return len(unique_list)

    
class Testing(unittest.TestCase):
    def setUp(self):
        self.lists = [
            [0,0,1,1,2,2],
            [-5,-3, -3, 0, 6, 10],
        ]
        self.expected = [
            (3, [0,1,2]),
            (5 ,[-5,-3, 0, 6, 10]),
        ]
    
    def test_remove_duplicates_good_memory(self):
        for i in range(len(self.lists)):
            result = Solution.remove_duplicates_good_memory(self.lists[i])
            self.assertEqual(result, self.expected[i][0])
            self.assertEqual(self.lists[i], self.expected[i][1])
    
    def test_remove_duplicates_better_runtime(self):
        for i in range(len(self.lists)):
            result = Solution.remove_duplicates_better_runtime(self.lists[i])
            self.assertEqual(result, self.expected[i][0])
            self.assertEqual(self.lists[i], self.expected[i][1])


if __name__ == '__main__':
    unittest.main()