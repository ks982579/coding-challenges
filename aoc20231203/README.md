# Advent of Code

## Day 3

You are taken to the gondola lift station, but they aren't moving. There's a part missing from the engine. 

### Part 1

The puzzle input is the engine schematic. Any number adjacent to a symbol, even diagonally, is a "part number". The "." is not a symbol. 

Your goal is to find the sum of all part numbers. 

#### Rust

Today is a bit harder than the other days. I am creating a `MemoryCell` that will hold the string slices so it can remember and check for symbols in lines all around. But for a struct to hold a slice, that will require... LifeTimes! So, in your progression through your Rust journey, we now are moving steadily towards more advanced topics. They aren't that bad though, just letting the compiler know we intend for the value to exist. 

Also, with the `MemoryCell`, I want to practice traits, so might do that as well... we'll see about implementing the default trait. I am very glad to go through lifetimes and traits at the same time. When you implement a trait with generics on a struct with lifetime, you must specify that the generic will explicitly live as long as the lifetime. 

All of this has lead me to the spooky [PhantomData](https://doc.rust-lang.org/core/marker/struct.PhantomData.html) for reference types stucts may be "tied" to. For what it is worth, the refactor for my idea of a generic number type would be a lot. So, that'll just be a constraint.

In Rust, sometimes it is easier to create structures to reference as types than to build something adhoc. 

The end value of chopping the string ends up being doubly decreased, so you nearly add +2 to see the next character instead of +1. 

I also didn't notice that the numbers on the edge were being overlooked. That's poor testing on my part, should have tested the _edge_ cases, literally. 

Answer: 540,212

### Part 2

Now, if two numbers are combined with an `*`, they are gears. The gear ratio is the product of the part numbers. You must give the sum of gear ratios. 

Wow...

I actually got this on my first submit, unlike Part 1. Basically, I got each number and then the position of any astricks around that number. Then I summed products of numbers with same astricks coordinates, being careful to nott double count any sums.
