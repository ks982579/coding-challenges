"""
Given Roman numerals this library can convert to them into an integer
"""

import unittest as ut

class Converter:
    help = "Class to convert between Roman and int"
    creed = {
        'i': 1,
        'v': 5,
        'x': 10,
        'l': 50,
        'c': 100,
        'd': 500,
        'm': 1000,
    }

    @classmethod
    def _letters_to_list_of_int(cls, romans: str) -> list[int]:
        int_list = []
        folded_romans = romans.casefold()
        for letter in folded_romans:
            # @TODO: Has chance to raise error that must be handled. 
            int_list.append(cls.creed[letter])
        
        return int_list

    @classmethod
    def roman_to_int(cls, roman: str):
        # Should we attempt a 'forward recusrion'?
        numbers = cls._letters_to_list_of_int(roman)
        return sum(numbers)
    
class TestConverter(ut.TestCase):
    def setUp(self):
        self.romans_q = ['X', 'V', 'II']
        self.romans_a = [10, 5, 2]
    
    def test_roman_to_int(self):
        for i in range(len(self.romans_q)):
            self.assertEqual(
                Converter.roman_to_int(self.romans_q[i]),
                self.romans_a[i],
            )


if __name__ == "__main__":
    print('Hello, world!')

    ut.main()

## There is a little sense of position in roman numeral

## xiv -> x(iv) = 10 + (5-1)
## xlix -> (xl)(ix) = (50 - 10)(10 - 1)
### Essentially, you should check if value preceeding next value is larger. 
