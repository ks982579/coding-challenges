# Advent of Code

## Day 2

You land on **Snow Island** in the sky. There's a long walk and an elf asks you to play a game. 

### Part 1

There's a small bag with red, green and blue cubes. When you play the game, your man hids a secret number of cubes of each colour in the bag. You must figure out iinformatiion about the number of cubes. Information gathered is from the elf reaching into the bag and showing you a random sample of cubes. 

Information appears like:

```
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
```

Each game has an ID, and the semicolon separates lists of subsets. 

Suppose the bag has 12 red, 13 green, and 14 blue cubes. You must determine which scenarios are possible and sum their game IDs. For example, game 3 above is not possible because of 20 red cubes. 

#### Rust

I think I want to convert games into a list (vector) of structures. Anticipating more colours, they should be fluid, maybe an enum. 

Then, we can check against a master game solution and sum the IDs.

I was using Enums until comparing a list of enums became difficult. After switching to structs, I notices that the enums weren't necessarily the problem. But after some testing, they would be a problem. You can always sort a Vector of Enums, which seems to in the order of the Enums which would be handy. But then there's the trying to compare values in the enums to be less than the maximum. It was a bad move to try that. 

My Answer: 2,716

### Part 2

What are the fewest number of cubes of each colour that could have been in the bag to make the game possible?

For an example, in game one above you would need:

```json
{
    "red": 4,
    "blue": 6,
    "green": 2
}
```

We now introduce _power_ of the set  of cubes is the product of the red, green and blue cubes. 

What is the power of the minimum set for all Games!

#### Rust

I set up my main so that it could tackle both problems. I rather like the approach of using `impl` to create methods on the structs. I also wrote sudo code and it's true that 10 minutes of sudo code can save you hours of coding. 

My answer: 72,227
