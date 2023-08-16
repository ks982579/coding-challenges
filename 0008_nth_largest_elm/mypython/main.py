"""
To find nth largest integer in array w/out sorting it!

https://docs.python.org/3/library/array.html
"""

from array import array
import unittest

class FindLargestElement:
    @staticmethod
    def find_nth_largest(nums: array, k: int) -> int:
        if k > len(nums):
            raise IndexError("K > length of list.")
        
        values = []
        values[0] = nums[0]
        ind = 0
        the_min = nums[0]
        for i in range(len(nums)):
            if nums[i] > the_min:
                the_min = 0
        return 0


class TestFindNthLargestElement(unittest.TestCase):
    def setUp(self) -> None:
        self.trials = [
            array('i', range(0,10))
        ]
        self.expected = [9]
    
    def test_finding_nth_largest(self):
        for trial in self.trials:
            actual = FindLargestElement.find_nth_largest(trial, 1)
            self.assertEqual(actual, self.expected[0])

if __name__ == "__main__":
    print('Hello, world!')
    unittest.main()