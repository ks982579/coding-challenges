"""
Program to create check if a palindrome has been passed in. 

run tests with `python3 -m unittest main.py
"""

import argparse
from math import trunc
import unittest


class Solution:
    @staticmethod
    def isPalindrome(x: int) -> bool:
        y = str(x)
        y_length = len(y)
        for i in range(trunc(y_length/2)):
            if y[i] != y[y_length-(i+1)]:
                return False
        
        return True


class TestPalindrome(unittest.TestCase):
    def setUp(self):
        self.numbers = [1234, 12321, 123321, 753159, -99999]
        self.expectations = [False, True, True, False, False]
    
    def test_palindrome(self):
        for i in range(len(self.numbers)):
            self.assertEqual(
                Solution.isPalindrome(self.numbers[i]),
                self.expectations[i],
            )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog="Palindrome Checker",
        description='Checks if input reads the same forwards as it does backwards',
        epilog='This text appears at bottom of help.'
    )
    parser.add_argument('word', nargs='?', default='', help='the word to check')
    
    args = parser.parse_args()

    print(Solution.isPalindrome(args.word))
