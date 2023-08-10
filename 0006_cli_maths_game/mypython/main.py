"""
To play a maths adding and multiplication game
"""

# Standard Library Imports
from sys import exit

from components.components import MathsGame

if __name__ == "__main__":
    game = MathsGame()
    game.run_game()

"""
+ Get Numbers
+ Display to user
+ Get user's guess
+ Check Answer
+ Adjust game
+ Repeat

dynamic method calling w/getattr(obj, attr)
"""