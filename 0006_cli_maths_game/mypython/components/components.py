"""
Helper classes and methods for the game
"""
from random import randint

class MathsGame():
    def __init__(self):
        print("Hello, world")
        self._numbers = [x+1 for x in range(10)]
    
    def _select_numbers(self) -> None:
        x = randint(0, len(self._numbers))
        y = randint(0, len(self._numbers))

    def _write(self, a: int, b: int, op: str) -> None:
        print(f"{a} {op} {b}")