"""
To play a maths adding and multiplication game
"""

# Standard Library Imports
from sys import exit

from components.components import MathsGame

if __name__ == "__main__":
    print('5 + 6')
    while True:
        try:
            guess = input().strip().casefold()
            if guess == 'q':
                exit()
            guess = int(guess)
            break
        except Exception as err:
            print('Not a valid guess. Enter a number or "q" to quit')
            continue

"""
+ Get Numbers
+ Display to user
+ Get user's guess
+ Check Answer
+ Adjust game
+ Repeat
"""