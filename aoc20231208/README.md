# Advent of Code 2023

## Day 8

A sandstorm is approaching and the elf you were just playing cards with has vanished... odd. You find some maps on the camel you are riding and want to use it to cross the desert and get out of the storm.

### Part I

The top of the map has some _left/right_ instructions, and below are nodes. You start at AAA and end at ZZZ. At the first node, depending on the _left/right_ instructions, you'll pick the corresponding left or right side of the tuple. 

Example: 

```
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
```

So, above, you start at AAA. The first instruction is "R", so you take the right side of AAA tuple, which is CCC. The next instruction is "L". Find the CCC tuple, take the left side, which is ZZZ. And you win in just one iteration of the instuctions! What does that mean? well, if you reach the end of the instructions and are still not at ZZZ, you start them again from the beginning... but don't go back to AAA obviously. You restart the instuction at your current node. 

Given your puzzle, how many iterations does it take to get to the end?

My Answer: 