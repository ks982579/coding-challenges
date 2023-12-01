# Advant of Code 2023-12-01

## Part 1

So, there's an elf who made a little mess of some calculations. There are answers that are embedded in strings. To extract the correct value, pull the first and last digit, in that order, to form a single two-digit number. 

Examples:

```json
{
  1abc2 = 12,
  pqr3stu8vwx = 38,
  a1b2c3d4e5f = 15,
  treb7uchet = 77
}
```

You'll be given your strings and need to submit the sum of all calibration values.

## Part 2

The mess is a bit more complicated than you'd originally thought. Seems that some of the numbers were also spelt out, and should be included in your calibrations. For example, "twoa1bc" before would become "11". Now we know it should acutally be "21". Suppose the same for just "abctwodef" would be "22". 

### Hint

The _gotcha_ is something like "abctwone" is actually 21. A simple find and replace method I used first made "abctwone" into "abctw1", giving "11". The way I handled it initially in Rust probably isn't super graceful, but works.

## The Answer

Overall answer to this puzzle is 53,340.
