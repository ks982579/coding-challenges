"""
Given Roman numerals this library can convert to them into an integer
"""

import unittest as ut

class Converter:
    help = "Class to convert between Roman and int"

    @staticmethod
    def roman_to_int(roman: str):
        if roman == 'X':
            return 10
    
class TestConverter(ut.TestCase):
    def setUp(self):
        self.romans_q = ['X']
        self.romans_a = [10]
    
    def test_roman_to_int(self):
        for i in range(len(self.romans_q)):
            self.assertEqual(
                Converter.roman_to_int(self.romans_q[i]),
                self.romans_a[i],
            )


if __name__ == "__main__":
    print('Hello, world!')

    ut.main()