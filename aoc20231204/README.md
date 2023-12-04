# Advent of Code

## Day 4: Scratchcards

We have gone up in the _gondola_ lift and arrived at a new seemingly floating island. It is warming and more humid than Snow Island. An Elf nearby explains this is Island Island and suggests speaking to the gardener. The Elf will let you borrow their boat if you help them determine their winnings of their scratchcards. 

### Part 1

These scrathers have been scratched and look like: 

```
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
```

Numbers to the left of `|` are winning numbers, and to the right are numbers you have. The first match is worth 1 point, and subsequent matches doubles the points' values. 

For example, their are 4 winning numbers, so our points starts at 1 and doubles to 2, 4, 8 points total. 

How many points in total?

#### Rust

I found this handy `iter().filter_map(|s| s.parse().ok()).collect()` pattern that allows for a pretty quick extraction of string number into values without crashing. Good for empty strings ;). 

My Answer: 32,001

### Part 2

Apparently the elf didn't know the actual rules. There are no points. Winners cause you to win more scratch cards, but not at random. If card 10 has 3 winning numbers, you win the next three scratch cards again, so 11, 12, and 13. 

You now must determine the total count of cards you would end up with. Note that, in the above example, since we won another card 11, if that card had a winner, we would technically have 2 of those winners... making more copies of 12...

The initial code from part 1 plays nicely into this. 

Ok, so it was going very well until I had a fight with the borrow checker and lost. It was not pretty, trying to update structs in the vector you are iterating through is very hard because you cannot reference the vector inside the iteration easily. You can only pull out one instance at a time because it's a reference to the vector, not just the instance. So, you can clone what you need from one and then update the other accordingly. 

My Answer: 5,037,841
