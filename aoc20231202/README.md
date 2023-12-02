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