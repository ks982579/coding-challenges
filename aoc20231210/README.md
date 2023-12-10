# Advent of Code

## Day 10

So, you take the glider up from Desert Island to the floating _metal island_. You wander around, but the place appears deserted. You do spot a signpost labeled "Hot Sprints", so you follow the sign.

As you walk, you see a strange creature scurry through the metalic grass and jump int a big pipe. You then realize there are many pipes in the ground.

### Part I

You make a wierd sketch of the pipes like this:
+ "|" = vertical pipe.
+ "-" = horizontal pipe.
+ "L" = $90\degree$ bend connecting _north_ to _east_.
+ "J" = $90\degree$ bend connecting _north_ to _west_.
+ "7" = $90\degree$ bend connecting _south_ to _west_.
+ "F" = $90\degree$ bend connecting _south_ to _east_.
+ "." = ground, no pipe.
+ "S" = Starting position of animal.

Easy Example:

```
.....
.S-7.
.|.|.
.L-J.
.....
```

We are told S will have exactly 2 pipes connecting to it. 

The mission is to get ahead of that creature. To do this, you want to find the pipe that is furthest from them. Not in a distance, but in number of steps to get there. Each pipe is a step.

Find the signle giant loop starting at "S". How many steps along the loop does it take to get from starting position to the point farthest from the starting position. 

My initial idea: 6,864