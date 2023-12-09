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

My answer in Rust is using vectors, which involves a lot of searching. I should have used a hashmap because it is causing an $O(n^3)$ situation, which I could get down to an $O(n^2)$. Maybe next time. 

I notice that Rust is very good for dereferencing many things. But _one_ thing it didn't dereference was my `&char` in the ternary expression. I suppose that makes sense as it doesn't know if I want to compare the pointers of the `char` itself, where in methods and other operations, it's explicit what the results _should_ be. Keep that in mind. 

My Answer: 12,361

Seems lower than all other answers ever.

### Part II

The **Sandstorm** is coming in hot and following the map didn't help you move too far... just went around in circles. Examine you map... you find that the number of nodes ending with 'A' is equal to the number ending in 'Z'.

_Ghost Theory_: A ghost would travel all paths ending with an 'A' at the same time until they all end at some path that ends with a 'Z'. 

You need to travers all paths at the same time until all paths end with a 'Z'. How many steps did that take? 

Why is testing important? I let my program go on for a minute thinking it was going to work. But as time passed I wondered if perhaps it was stuck in a loop. I created and ran a quick unit test and it was caught in an infinite loop, just counting away. A quick fix is all it needed. Still, the program takes a while. 

So brute force is taking forever, but I thought of an idea. It's obviously a lowest common multiple problem. 

My Answer: 18,215,611,419,223
