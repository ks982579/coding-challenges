"""
Helper classes and methods for the game
"""
from enum import Enum
import os
from random import randint

class Engine(Enum):
    ON = 0
    OFF = 1

class MathsGame():
    def __init__(self):
        print("Hello, world")
        self._numbers = [x+1 for x in range(10)]
    
    def _select_numbers(self) -> None:
        x = randint(0, len(self._numbers))
        y = randint(0, len(self._numbers))
        return x,y 

    def _write(self, a: int, b: int, op: str) -> None:
        print(f"{a} {op} {b}")
    
    def _read(self, answer: int) -> None:

        while True:
            try:
                guess = input("> ").strip().casefold()
                if guess == 'q':
                    exit()
                guess = int(guess)
                break
            except Exception as err:
                print('Not a valid guess. Enter a number or "q" to quit')
                continue
    
    def run_game(self) -> None:
        game_state = Engine.ON
        while game_state == Engine.ON:
            # Get numbers
            x,y = self._select_numbers()
            # Random add, sub, mult....
            expected = x + y
            self._write(x,y,'+')
            try:
                actual = input("> ").strip().casefold()
                if actual == 'q':
                    game_state = Engine.OFF
                    continue
                actual = int(actual)
                # break
            except Exception as err:
                print('Not a valid guess. Enter a number or "q" to quit')
                continue
            if actual == expected:
                print('Correct!')
            else:
                print('Wrong...')
            # os.system('cls' if os.name == 'nt' else 'clear')
        # Could be for saving score or something, instead of hard exit.
