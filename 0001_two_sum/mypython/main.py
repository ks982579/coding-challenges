"""
Solution to Two Sum challenge. 
"""
import unittest
from typing import Iterable, Tuple


class Solutions(object):
    @staticmethod
    def two_sum_double_loop(nums: Iterable[int], target: int) -> Tuple[int]:
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
        to target value using a hash table.

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
        self.lists = [
            [125, 163, 55, 79, 32],
            [17, 19, 23, 29, 31],
            [59, 67, 73, 97]
        ]
        self.targets = [
            79+32,
            19+31,
            97+97
        ]
        self.expected = [
            (3, 4),
            (1, 4),
            None
        ]
    
    def test_two_sum_double_loop(self) -> None:
        for i in range(len(self.targets)):
            result = Solutions.two_sum_double_loop(
                nums=self.lists[i],
                target=self.targets[i]
            )
            self.assertEqual(self.expected[i], result)
    
    def test_two_sum_hash(self) -> None:
        for i in range(len(self.targets)):
            result = Solutions.two_sum_hash(
                nums=self.lists[i],
                target=self.targets[i]
            )
            self.assertEqual(self.expected[i], result)



if __name__ == '__main__':
    unittest.main()
