# Advent of Code 2024-12-01

## Part 1

The **Chief Historian** is not at the big Christmas sleigh launch.
He was last known to be checking historical places in the North Pole.
You need to help a group of Senior Historians find the Chief.

First place to check is the Chief's office.
You find a list with locations listed by a _location ID_.
The lists need to be reconciled.

Given the lists, pair the numbers and measure their difference.
You pair up smallest numbers from each list first and continue.
The sum the differences.

Examples:

> You don't need an example

Just notes

```bash
cmake .
cmake --build .
```

And learning new things -> Reset default editor for Git:

```bash
git config --global core.editro "nvim"
```

### Answer

1646452

## Part 2

Well, since the sum was no zero, this means the lists are different.
Or maybe it is bad handwriting.
We notice many location IDs appear in both lists.
So now we compute a similarity score.

I am thinking a cache system will speed up the counting.

### Hint

> Not Yet...

### The Answer

23609874
