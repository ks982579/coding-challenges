"""
To find nth largest integer in array w/out sorting it!

https://docs.python.org/3/library/array.html
"""

from array import array
import unittest

class FindLargestElement:
    @staticmethod
    def find_nth_largest(nums: array, k: int) -> int:
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