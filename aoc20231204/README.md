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