"""
Separate file for unit tests. 
"""
# From Standard Library
import unittest
import sys
from pathlib import Path

# Add parent directory to sys.path
parent = Path(f"{__file__}/../..").resolve()
sys.path.insert(0,str(parent))

# Custom libraries
from components.components import MathsGame


class TestGameUnits(unittest.TestCase):
    def setUp(self):
        ...
    def first_test(self):
        MathsGame()
        self.assertEqual(1,1)

if __name__ == "__main__":
    unittest.main()