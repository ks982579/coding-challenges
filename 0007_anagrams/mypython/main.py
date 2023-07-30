"""
Creating a method that solves if 2 words are anagrams.
"""

import unittest
from sys import stdout

class Anagram(object):
    @staticmethod
    def check(word_one, word_two):
        word_one = word_one.strip().casefold()
        word_two = word_two.strip().casefold()
        if word_one == word_two or len(word_one) != len(word_two):
            return False
        else:
            letters_of_word_one = [x for x in word_one]
            for letter in word_two:
                if letter in letters_of_word_one:
                    letters_of_word_one.remove(letter)
                    continue
                else:
                    return False
            if len(letters_of_word_one) == 0:
                return True
            else:
                return False


class TestAnagram(unittest.TestCase):
    def setUp(self):
        self.word_one = [
            "race", "part ", "Heart", "knee", "Anger", "AAron", "chocolate",
        ]
        self.word_two = [
            "  care", " trap ", "Earth", "keen", "Hunger", "Aron", "ChoColate  ",
        ]
        self.answers = [
            True, True, True, True, False, False, False,
        ]
    
    def test_check_anagrams(self):
        for i in range(len(self.answers)):
            value = Anagram.check(
                self.word_one[i],
                self.word_two[i]
            )
            self.assertEqual(
                value,
                self.answers[i]
            )

if __name__ == "__main__":
    stdout.write("Running Tests...\n")
    unittest.main()
