# Advent of Code 2023

## Day 7: Camel Cards

The all-expenses-paid trip to desert island was a five-minute one-way trip on an airship. An elf askes you if you brought "the parts". She takes you to the big rocks and explains that this part of the island is directly above Island Island. They usually use large machines to move the rocks and filter the sand, but the machines are broken and they haven't been receiving the parts to fix the machines. The journey to get parts will take time. How about a game of **Camel Cards**?

### Part I

A hand is 5 cards, standard deck of cards with A, K, Q, ..., 3, 2. The game is like poker. Strength of a hand is based on cards and A is the highest, 2 is the lowest. 

+ 5 of a kind
+ 4 of a kind
+ full house
+ three of a kind
+ two pair
+ one pair
+ high card

Hands are ordered based on _type_. Every full house is stronger than any 3 of a kind. If 2 hands are the same type, we compare strengths of individual cards, starting with the first. If the first cards are different, the stronger one wins. Even if the hands are 323Q3 and 2KAAA, the left wins because both are 3 of a kind, but the left has a higher _first_ card. 

To play, you are given a list of hands and their corresponding bid, example: 

```
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
```

Each hand wins an amount equal to its bid multiplied by its rank. The weakest hand is rank 1 and the strongest is rank 5. Given your puzzle input, you must determine the total winnings.

For context, there are 1,000 hands. 

Had an issue with the sorting thinking equal return none and continues to sort...

My Answer: 246,163,188
