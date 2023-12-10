# Advent of Code 2023

## Day 9

You have followed your map to the spot and the sandstorm subsides. You see now an _oasis_. Lucky dog. You notice above you another floating island, this one has lots of metal sticking out of it. Must be where the parts to fix the sand machines come from. 

You find a hang glider partially buried in the sand and get an idea. When the sun rises and heats the sand, the hot air might take you up to **Metal Island**. 

### Part I

This is a big one...

While you wait, you pull out your _Oasis and Sand Instability Sensor_ (OASIS)... must have been on a camel. It analyzes your surroundings and give a report of many values and how they change over time. Each line contains the history of a single value. 

```
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
```

To protect the oasis, your report should include a prediction of the next value in each history.
+ Make a new sequence from the difference at each step.
+ if sequence is not all zeros, repeat the process with the new sequence.
+ If they are all zeros, extrapolate the next value.
    + At bottom, the zeros, add another 0
    + you add your zero the the last digit of the previous row
    + repeat until you are at the top.

```
1   3   6  10  15  | 21
  2   3   4   5   | 6
    1   1   1   | 1
      0   0   | 0
```

Analyze your OASIS report and extrapolate the next value for each history. What is the sum of extrapolated values?

My Answer: 1,904,165,718

Recursion in Rust is not bad.

### Part II

The rules of generating new values is kind of changing. Well, instead of generating just new values, what if we wanted more historic values? 

Think of it as the same as before, but we will go up the left side, which are the initial values, and calculate an earlier value.

```
5 |  10  13  16  21  30  45
  5 |   3   3   5   9  15
   -2 |   0   2   4   6
      2 |   2   2   2
        0 |   0   0
```

Above is an example of how we find our previous value. It isn't adding anymore but more like $x_0-y_{-1} = x_{-1}$. 

My Answer: 964
