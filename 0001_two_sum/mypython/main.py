"""
Solution to Two Sum challenge. 
"""
import math
import unittest
from typing import Iterable, Tuple

HASH_MAP = {
    '0': 'ze',
    '1': 'on',
    '2': 'tw',
    '3': 'th',
    '4': 'fo',
    '5': 'fi',
    '6': 'si',
    '7': 'se',
    '8': 'ei',
    '9': 'ni'
}

def generate_hash(num):
    divisor = 1
    hash = ""

    # positive or negative
    if num > 0:
        hash += 'po'
    elif num < 0:
        num *= -1
        hash += 'ne'
    else:
        return 'ze'
    
    return hash+''.join(HASH_MAP[j] for j in str(num))

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
        # create the hash map
        hash_map = {generate_hash(nums[x]): x for x in range(len(nums))}
        for i in range(len(nums)):
            # we assume that x + y = target
            x = nums[i]
            y = target - x
            y_hash = generate_hash(y)
            if y_hash in hash_map and hash_map[y_hash] != i:
                return (i, hash_map[y_hash])


class TestSumMethods(unittest.TestCase):
    """
    Running tests for the challenge
    """
    def setUp(self) -> None:
        self.lists = [
            [125, 163, 55, 79, 32],
            [17, 19, 23, 29, 31],
            [59, 67, 73, 97],
            [17, 19, 23, 29, -31],
        ]
        self.targets = [
            55+79,
            19+31,
            97+97,
            29-31,
        ]
        self.expected = [
            (2, 3),
            (1, 4),
            None,
            (3, 4),
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
    # print(generate_hash(1234))
    # print(generate_hash(0))
    # print(generate_hash(-1234))
