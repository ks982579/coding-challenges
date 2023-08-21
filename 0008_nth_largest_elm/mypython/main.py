"""
To find nth largest integer in array w/out sorting it!

https://docs.python.org/3/library/array.html
"""

from array import array
import unittest

class FindLargestElement:
    @staticmethod
    def find_nth_largest(nums: array, k: int) -> int:
        # tmp = nums[0]

        if k > len(nums):
            raise IndexError("K > length of list.")
        
        for _ in range(k):
            tmp = nums[0]
            for numb in nums:
                tmp = numb if numb > tmp else tmp
            nums.remove(tmp) # Inplace operation
        return tmp


class TestFindNthLargestElement(unittest.TestCase):
    def setUp(self) -> None:
        self.trials = [
            array('i', range(0,10)),
            [9,8,7,6,5],
            [-100,100,32,64,77,9,0]
        ]
        self.expected = [9,8,64]
        self.n_value = [1,2,3]
    
    def test_finding_nth_largest(self):
        for i in range(len(self.trials)):
            actual = FindLargestElement.find_nth_largest(self.trials[i], self.n_value[i])
            self.assertEqual(actual, self.expected[i])

if __name__ == "__main__":
    print('Hello, world!')
    unittest.main()