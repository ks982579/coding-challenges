"""
Solution to Two Sum challenge. 
"""
import unittest
from typing import Iterable, Self, Tuple


class Solutions(object):
    @staticmethod
    def two_sum_double_loop(nums: Iterable[int], target: int) -> Tuple[int]:
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[int]
        """
        for i in range(len(nums)):
            for j in range(len(nums)):
                if i == j:
                    continue
                elif (nums[i] + nums[j]) == target:
                    return i, j
    
    @staticmethod
    def two_sum_hash(nums: Iterable[int], target: int) -> Tuple[int]:
        """
        Given an iterable of integers, this method returns the index of the 2 that sum up
        to target value. 

        If multiple combinations of different numbers add to the sum, only the first found combination is returned. 

        Args:
            nums (Iterable[int]): Iterable of numbers to sum to target
            target (int): target value for sum.

        Returns:
            Tuple[int]: Index values of the elements whose sum 
        """
        ...


class TestSumMethods(unittest.TestCase):
    """
    Running tests for the challenge
    """
    def setUp(self) -> None:
        return super().setUp()
    
    def test_double_loop(self) -> None:

if __name__ == '__main__':
    unittest.main()
    test_one = [20, 6,10,13,4]
    target = 13 + 4

    result = Solutions.two_sum_double_loop(test_one, target)
    print(result)

