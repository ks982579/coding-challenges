# Roman Numeral to Integer

```yaml
url: https://leetcode.com/problems/roman-to-integer/
title: Roman to Integer
difficulty: easy
```

I particularly like this exercise. Here are the following Roman Numerals:

| Roman | Int |
| :-----: | :---: |
| I | 1 |
| V | 5 |
| X | 10 |
| L | 50 |
| C | 100 |
| D | 500 |
| M | 1000 |

According to [wikipedia](https://en.wikipedia.org/wiki/Roman_numerals), there's a special case for $N = 0$, as the Roman's didn't actually really account for the number zero.
We will keep it simple for now. Remember the pattern that $IIII \neq 4$, but $IV = 4$. It's a subtract one system to make writing a little easier. 