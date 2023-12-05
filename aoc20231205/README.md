# Advent of Code 2023

## Day 5: If You Give a Seed a Fertilizer

We now take the boat to the gardener. He tells you the island is the water source. The water stopped because the elves rand out of _sand_ to filter it with. The elf suggests taking a ferry to check out why they stopped getting sand. 

### Part 1 

But while we wait for the ferry, solve Island Island food production problem. They have an Almanac they need help understanding. It lists seeds that need planing, the soil to use with seed, and the gertilizer to use with soil, and water to use with fertilizer, etc... They are identified with a number, but numbers are reused by each category. So, 123 soil doesn't mean 123 fertilizer are related. 

```
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
```

The "maps" describe how to convert a seed number into a soil number. We have:
+ destination range start
+ source range start
+ range length

First soil map starts at 98 and contains 98 and 99 `(x..x+y)`. 

The destination starts at 50 and has 50 and 51. 

If a source isn't mapped, it corresponds to the same number! So, 14 is 14 in first example. Here's entire list of seed-to-soil:

```
seed  soil
0     0
1     1
...   ...
48    48
49    49
50    52
51    53
...   ...
96    98
97    99
98    50
99    51
```

We want to find closest location for a seed. These are the lowest location numbers corresponding to any initial seed. 

My Answer: 389,056,265

### Part 2

Everyone will starve if you only plant such a small number of seeds. Re-reading the almanac, it looks like the seeds: line actually describes ranges of seed numbers.

The values on the initial seeds: line come in pairs. Within each pair, the first value is the start of the range and the second value is the length of the range. So, in the first line of the example above:

seeds: 79 14 55 13

This line describes two ranges of seed numbers to be planted in the garden. The first range starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92. The second range starts with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.

Now, rather than considering four seed numbers, you need to consider a total of 27 seed numbers.

In the above example, the lowest location number can be obtained from seed number 82, which corresponds to soil 84, fertilizer 84, water 84, light 77, temperature 45, humidity 46, and location 46. So, the lowest location number is 46.

Consider all of the initial seed numbers listed in the ranges on the first line of the almanac. What is the lowest location number that corresponds to any of the initial seed numbers?