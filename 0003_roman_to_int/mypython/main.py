"""
Given Roman numerals this library can convert to them into an integer
"""

import unittest as ut

class Converter:
    help = "Class to convert between Roman and int"

    @staticmethod
    def roman_to_int(roman: str):
        return 0
    
class TestConverter(ut.TestCase):
    def setUp(self):
        super()
    
    def test_roman_to_int(self):
        self.assertEqual(Converter.roman_to_int('test'), 0)


if __name__ == "__main__":
    print('Hello, world!')

    ut.main()