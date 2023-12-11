#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    East,
    West,
    #[default]
    Above,
}
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MazeRunner {
    // ( Y, X) to make simpler when traversing vectors.
    pub steps: usize,
    pub from: Direction,
    pub position: (usize, usize),
    pub current_pipe: char,
    pub memory: Vec<(usize, usize)>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PipeMaze {
    pub start: (usize, usize),
    pub matrix: Vec<Vec<char>>,
    pub length: usize, // Max Y
    pub width: usize, // Max X
}

impl PipeMaze {
    pub fn from_str(data: &str) -> PipeMaze {
        println!("hi");
        let mut start: (usize, usize) = (0,0);
        let mut maze: Vec<Vec<char>> = Vec::new();
        for datum in data.lines().enumerate() {
            if let Some(index) = datum.1.find('S') {
                start = (datum.0, index);
                println!("Starting Position: {:?}", &start);
            }
            maze.push(
                datum.1.chars().collect()
            )
        }

        let length = maze.len();
        let width = maze[0].len();

        PipeMaze {
            start: start,
            matrix: maze,
            length: length,
            width: width,
        }
    }
    pub fn get_char_at_position(&self, pos: &(usize, usize)) -> char {
        self.matrix[pos.0][pos.1]
    }
}

impl MazeRunner {
    // fn first_move(&mut self, maze: &PipeMaze) {
    //     let look_up = (self.position.0 -1, self.position.1);
    //     let mut pipe = maze.get_char_at_position(&look_up);
    //     match pipe {
    //         '|' => self.
    //     }
    // }
    // fn can_go_north(&self, maze: &PipeMaze) -> bool {
    //     if self.from == Direction::South {
    //         return false;
    //     }

    //     let mut potential_new_pos = self.position.clone();
        
    //     // North is -1
    //     potential_new_pos.0 -= 1;
        
    //     if potential_new_pos.0 < 0 {
    //         return false;
    //     }

    //     let pipe: char = maze.get_char_at_position(&potential_new_pos);

    //     if pipe == '|' || pipe == 
    // }
    pub fn traverse_maze(&mut self, maze: &mut PipeMaze) {
        self.position = maze.start.clone();
        let loop_start = true;
        self.current_pipe = 'S';
        let mut all_locations: Vec<(usize, usize)> = Vec::new();
        loop {
            match self.current_pipe {
                '|' => {
                    all_locations.push(self.position.clone());
                    if self.from == Direction::North {
                        // new position
                        self.position = (self.position.0 + 1, self.position.1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // From is still North...
                    } else { // BOLD - but direction must be from south
                        // new position
                        self.position = (self.position.0 - 1, self.position.1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // From is still South...
                    } 
                    self.steps += 1;
                }
                '-' => {
                    if self.from == Direction::East {
                        // new position
                        self.position = (self.position.0, self.position.1 - 1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // From is still East...
                    } else { // BOLD - but direction must be from south
                        // new position
                        self.position = (self.position.0, self.position.1 + 1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // From is still South...
                    } 
                    self.steps += 1;
                }
                'L' => {
                    all_locations.push(self.position.clone());
                    if self.from == Direction::North {
                        // new position
                        self.position = (self.position.0, self.position.1 + 1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // Going East so from the west
                        self.from = Direction::West;
                    } else { // BOLD - must be from the east - going north, from south
                        // new position
                        self.position = (self.position.0 - 1, self.position.1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // From is still South...
                        self.from = Direction::South;
                    } 
                    self.steps += 1;
                }
                'J' => {
                    all_locations.push(self.position.clone());
                    if self.from == Direction::North {
                        // new position
                        self.position = (self.position.0, self.position.1 - 1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // Going East so from the west
                        self.from = Direction::East;
                    } else { // BOLD - must be from the west - going north, from south
                        // new position
                        self.position = (self.position.0 - 1, self.position.1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // From is still South...
                        self.from = Direction::South;
                    } 
                    self.steps += 1;
                }
                '7' => {
                    all_locations.push(self.position.clone());
                    if self.from == Direction::South {
                        // new position
                        self.position = (self.position.0, self.position.1 - 1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // Going East so from the west
                        self.from = Direction::East;
                    } else { // BOLD - must be from the west - going south, from north
                        // new position
                        self.position = (self.position.0 + 1, self.position.1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // From is still South...
                        self.from = Direction::North;
                    } 
                    self.steps += 1;
                }
                'F' => {
                    all_locations.push(self.position.clone());
                    if self.from == Direction::South {
                        // new position
                        self.position = (self.position.0, self.position.1 + 1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // Going East so from the west
                        self.from = Direction::West;
                    } else { // BOLD - must be from the east - going south, from north
                        // new position
                        self.position = (self.position.0 + 1, self.position.1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // From is still South...
                        self.from = Direction::North;
                    } 
                    self.steps += 1;
                }
                'S' => {
                    if self.from != Direction::Above {
                        break;
                    }
                    all_locations.push(self.position.clone());
                    // look right - I know there is something here on my puzzle
                    let mut potential_position = self.position.clone();
                    potential_position.1 += 1;
                    let potential_pipe = maze.get_char_at_position(&potential_position);
                    if potential_pipe == '7' {
                        self.position = potential_position;
                        self.current_pipe = potential_pipe;
                        self.from = Direction::West;
                        self.steps += 1;
                    } else if potential_pipe == 'J' {
                        self.position = potential_position;
                        self.current_pipe = potential_pipe;
                        self.from = Direction::West;
                        self.steps += 1;
                    }
                    
                    // Reset
                    potential_position = self.position.clone();
                    potential_position.0 += 1;
                    let potential_pipe = maze.get_char_at_position(&potential_position);
                    if potential_pipe == '|' {
                        self.position = potential_position;
                        self.current_pipe = potential_pipe;
                        self.from = Direction::North;
                        self.steps += 1;
                    } else if potential_pipe == 'J' {
                        self.position = potential_position;
                        self.current_pipe = potential_pipe;
                        self.from = Direction::North;
                        self.steps += 1;
                    }
                }


                _ => { panic!("Non-matching Character: {}", &self.current_pipe) }
            }
        }
        self.memory = all_locations;
        // --------------------------------
    }
    pub fn count_nest_spots(&mut self, maze: &PipeMaze) -> usize {
        self.memory.sort_by(|x, y| {
            if x.0 == y.0 {
                return x.1.cmp(&y.1);
            } else {
                return x.0.cmp(&y.0);
            }
        });
        // println!("{:?}", &self.memory);
        let mut ordered_chaos: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut tmp_vec: Vec<(usize, usize)> = Vec::new();
        let mut y: Option<usize> = None;
        for pair in self.memory.iter() {
            match y {
                None => {
                    y = Some(pair.0);
                    tmp_vec.push(pair.clone());
                }
                Some(prev) => {
                    if pair.0 == prev {
                        tmp_vec.push(pair.clone());
                    } else {
                        ordered_chaos.push(tmp_vec.clone());
                        tmp_vec = Vec::new();
                        tmp_vec.push(pair.clone());
                        y = Some(pair.0);
                    }
                }
            }
        }
        if tmp_vec.len() > 0 {
            ordered_chaos.push(tmp_vec);
        }
        // println!("{:?}", &ordered_chaos);
    
        let mut left: Option<(usize, usize)> = None;
        // let mut right: Option<(usize, usize)> = None;
        let mut count: usize = 0;
    
        for entropy in ordered_chaos {
            // Vec<(usize, usize)>
            for chaos in entropy {
                match left {
                    None => left = Some(chaos),
                    Some(left_pair) => {
                        if left_pair.0 != chaos.0 {
                            dbg!("{} != {}", &left_pair.0, &chaos.0);
                        }
                        dbg!(&left_pair);
                        dbg!(&chaos);
                        let mut horizontal = true;
                        for c in (left_pair.1+1..chaos.1) {
                            let check_pair = (left_pair.0, c);
                            dbg!(&check_pair);
                            let pipe = maze.get_char_at_position(&check_pair);
                            dbg!(&pipe);

                            if '-' == pipe && horizontal {
                                continue;
                            } else {
                                horizontal = false;
                                count += 1;
                                dbg!(&count);
                            }
                        }
                        left = None;
                    }
                }
            }
            // RESET
            left = None;
        }
        count
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tests() {
        assert!(true);
    }

    #[test]
    fn test_maze_runner() {
        let puzzle = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        let mut maze = PipeMaze::from_str(puzzle);
        let mut runner = MazeRunner::default();
        runner.traverse_maze(&mut maze);
        assert_eq!(runner.steps, 16);
    }

    #[test]
    fn test_chaos() {
        let puzzle = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        let mut maze = PipeMaze::from_str(puzzle);
        let mut runner = MazeRunner::default();
        runner.traverse_maze(&mut maze);
        // dbg!(&runner);
        assert_eq!(runner.steps, 140);
        let count = runner.count_nest_spots(&maze);
        assert_eq!(count, 8);
    }

    #[test]
    fn test_chaos2() {
        let puzzle = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

        let mut maze = PipeMaze::from_str(puzzle);
        let mut runner = MazeRunner::default();
        runner.traverse_maze(&mut maze);
        // dbg!(&runner);
        assert_eq!(runner.steps, 160);
        let count = runner.count_nest_spots(&maze);
        assert_eq!(count, 10);
    }
}