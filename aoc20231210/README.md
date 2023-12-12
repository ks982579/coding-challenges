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

### Part II

The creature never emerges at the point you calculated and you wonder if there's a nest in there somewhere. The main loop encloses the '.' being just ground. Those are areas where a nest could be. To determine if we have time to search all the locations, determine how many tiles are enclosed by the loop.

My best idea is to change the maze so that all travelled pipes are say 'O'. Then, we can do a breadth search on the outside to... that wont work because if pipes are touching then the search can't get in.

In my puzzle, 'S' would be an 'F'. This means it would not be included in the analysis. [Point in Polygon | Wiki](https://en.wikipedia.org/wiki/Point_in_polygon) article describes a couple of techniques for determining if something lies inside a polygon. If we look left to right, the '|' character is the cross over character. 

But you can build up and down with brackets. It took some drawing and research, but including '7' and 'F' seems to be what you need. You could probably change to include 'J' and 'L', but need some combination like that. 

**Any Tile that isn't part of the main loop is enclosed in the loop!!!**

It is not [26, 53]

--- Part Two ---

You quickly reach the farthest point of the loop, but the animal never emerges. Maybe its nest is within the area enclosed by the loop?

To determine whether it's even worth taking the time to search for such a nest, you should calculate how many tiles are contained within the loop. For example:

...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........

The above loop encloses merely four tiles - the two pairs of . in the southwest and southeast (marked I below). The middle . tiles (marked O below) are not in the loop. Here is the same loop again with those regions marked:

...........
.S-------7.
.|F-----7|.
.||OOOOO||.
.||OOOOO||.
.|L-7OF-J|.
.|II|O|II|.
.L--JOL--J.
.....O.....

In fact, there doesn't even need to be a full tile path to the outside for tiles to count as outside the loop - squeezing between pipes is also allowed! Here, I is still within the loop and O is still outside the loop:

..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........

In both of the above examples, 4 tiles are enclosed by the loop.

Here's a larger example:

.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...

The above sketch has many random bits of ground, some of which are in the loop (I) and some of which are outside it (O):

OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO

In this larger example, 8 tiles are enclosed by the loop.

Any tile that isn't part of the main loop can count as being enclosed by the loop. Here's another example with many bits of junk pipe lying around that aren't connected to the main loop at all:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L

Here are just the tiles that are enclosed by the loop marked with I:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L

In this last example, 10 tiles are enclosed by the loop.

Figure out whether you have time to search for the nest by calculating the area within the loop. How many tiles are enclosed by the loop?

Must save all... I just saw my error of "|L-7F..."
+ cross in on '|'
+ I say cross out at 'L', but you don't cross out until the '7'
+ If you collect everything, maybe extend out after all '-'. 
    + it acts like a bridge. 
    + So you don't count going in the loop or out of the loop until you pass all '-'
    + Quick and dirty might be to always set status to "IN" when hit horizontal pipe.
        + No, there are times when you need it to go both ways. 

You must look out for "L7" and "FJ"