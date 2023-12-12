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
    pub memory: Vec<Pipe>,
}

#[derive(Debug)]
pub enum State {
    In,
    Out,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PipeMaze {
    pub start: (usize, usize),
    pub matrix: Vec<Vec<char>>,
    pub length: usize, // Max Y
    pub width: usize,  // Max X
}

impl PipeMaze {
    pub fn from_str(data: &str) -> PipeMaze {
        println!("hi");
        let mut start: (usize, usize) = (0, 0);
        let mut maze: Vec<Vec<char>> = Vec::new();
        for datum in data.lines().enumerate() {
            if let Some(index) = datum.1.find('S') {
                start = (datum.0, index);
                println!("Starting Position: {:?}", &start);
            }
            maze.push(datum.1.chars().collect())
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

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pipe {
    symbol: char,
    y: usize,
    x: usize,
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
        let mut all_locations: Vec<Pipe> = Vec::new();
        let mut first_direction: Direction = Direction::Above;
        let mut last_direction: Direction = Direction::Above;
        loop {
            all_locations.push(Pipe {
                symbol: self.current_pipe,
                y: self.position.0,
                x: self.position.1,
            });
            match self.current_pipe {
                '|' => {
                    if self.from == Direction::North {
                        // new position
                        self.position = (self.position.0 + 1, self.position.1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // From is still North...
                    } else {
                        // BOLD - but direction must be from south
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
                    } else {
                        // BOLD - but direction must be from south
                        // new position
                        self.position = (self.position.0, self.position.1 + 1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // From is still South...
                    }
                    self.steps += 1;
                }
                'L' => {
                    if self.from == Direction::North {
                        // new position
                        self.position = (self.position.0, self.position.1 + 1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // Going East so from the west
                        self.from = Direction::West;
                    } else {
                        // BOLD - must be from the east - going north, from south
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
                    if self.from == Direction::North {
                        // new position
                        self.position = (self.position.0, self.position.1 - 1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // Going East so from the west
                        self.from = Direction::East;
                    } else {
                        // BOLD - must be from the west - going north, from south
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
                    if self.from == Direction::South {
                        // new position
                        self.position = (self.position.0, self.position.1 - 1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // Going East so from the west
                        self.from = Direction::East;
                    } else {
                        // BOLD - must be from the west - going south, from north
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
                    if self.from == Direction::South {
                        // new position
                        self.position = (self.position.0, self.position.1 + 1);
                        // update current pipe
                        self.current_pipe = maze.get_char_at_position(&self.position);
                        // Going East so from the west
                        self.from = Direction::West;
                    } else {
                        // BOLD - must be from the east - going south, from north
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
                        last_direction = self.from.clone();
                        break;
                    }
                    // look right - I know there is something here on my puzzle
                    let mut potential_position = self.position.clone();
                    potential_position.1 += 1;
                    let potential_pipe = maze.get_char_at_position(&potential_position);
                    if potential_pipe == '7' || potential_pipe == 'J' || potential_pipe == '-' {
                        self.position = potential_position;
                        self.current_pipe = potential_pipe;
                        self.from = Direction::West;
                        first_direction = Direction::West;
                        self.steps += 1;
                        continue;
                    }
                    // Reset
                    potential_position = self.position.clone();
                    potential_position.0 += 1;
                    let potential_pipe = maze.get_char_at_position(&potential_position);
                    if potential_pipe == '|' || potential_pipe == 'J' || potential_pipe == 'L' {
                        self.position = potential_position;
                        self.current_pipe = potential_pipe;
                        self.from = Direction::North;
                        first_direction = Direction::North;
                        self.steps += 1;
                        continue;
                    }
                    // Reset
                    potential_position = self.position.clone();
                    potential_position.1 -= 1;
                    let potential_pipe = maze.get_char_at_position(&potential_position);
                    if potential_pipe == '-' || potential_pipe == 'F' || potential_pipe == 'L' {
                        self.position = potential_position;
                        self.current_pipe = potential_pipe;
                        self.from = Direction::East;
                        first_direction = Direction::East;
                        self.steps += 1;
                        continue;
                    }
                    // Reset
                    potential_position = self.position.clone();
                    potential_position.0 -= 1;
                    let potential_pipe = maze.get_char_at_position(&potential_position);
                    if potential_pipe == '|' || potential_pipe == 'F' || potential_pipe == '7' {
                        self.position = potential_position;
                        self.current_pipe = potential_pipe;
                        self.from = Direction::South;
                        first_direction = Direction::South;
                        self.steps += 1;
                        continue;
                    }
                }
                _ => {
                    panic!("Non-matching Character: {}", &self.current_pipe)
                }
            }
        }

        let mut what_is_s: char = 'S';
        match first_direction {
            Direction::North => match last_direction {
                Direction::North => {
                    what_is_s = '|';
                }
                Direction::South => {
                    what_is_s = '?';
                }
                Direction::East => {
                    what_is_s = 'F';
                }
                Direction::West => {
                    what_is_s = '7';
                }
                _ => panic!(),
            },
            Direction::South => match last_direction {
                Direction::North => {
                    what_is_s = '?';
                }
                Direction::South => {
                    what_is_s = '|';
                }
                Direction::East => {
                    what_is_s = 'L';
                }
                Direction::West => {
                    what_is_s = 'J';
                }
                _ => panic!(),
            },
            Direction::East => match last_direction {
                Direction::North => {
                    what_is_s = 'F';
                }
                Direction::South => {
                    what_is_s = 'L';
                }
                Direction::East => {
                    what_is_s = '-';
                }
                Direction::West => {
                    what_is_s = '?';
                }
                _ => panic!(),
            },
            Direction::West => match last_direction {
                Direction::North => {
                    what_is_s = 'L';
                }
                Direction::South => {
                    what_is_s = 'F';
                }
                Direction::East => {
                    what_is_s = '?';
                }
                Direction::West => {
                    what_is_s = '-';
                }
                _ => panic!(),
            },
            _ => panic!(),
        }
        // The first == the last
        all_locations.pop();
        if let Some(the_s) = all_locations.first_mut() {
            the_s.symbol = what_is_s;
        }
        self.memory = all_locations;
    }

    pub fn count_nest_spots(&mut self, maze: &PipeMaze) -> usize {
        self.memory.sort_by(|a, b| {
            if a.y == b.y {
                return a.x.cmp(&b.x);
            } else {
                return a.y.cmp(&b.y);
            }
        });
        let mut ordered_chaos: Vec<Vec<Pipe>> = Vec::new();
        let mut tmp_vec: Vec<Pipe> = Vec::new();
        let mut y: Option<Pipe> = None;
        for pipe in self.memory.iter() {
            match y {
                None => {
                    y = Some(pipe.clone());
                    tmp_vec.push(pipe.clone());
                }
                Some(ref prev) => {
                    if pipe.y == prev.y {
                        tmp_vec.push(pipe.clone());
                    } else {
                        ordered_chaos.push(tmp_vec.clone());
                        tmp_vec = Vec::new();
                        tmp_vec.push(pipe.clone());
                        y = Some(pipe.clone());
                    }
                }
            }
        }
        if tmp_vec.len() > 0 {
            ordered_chaos.push(tmp_vec);
        }

        let mut count: usize = 0;
        for entropy in ordered_chaos {
            let mut status: State = State::Out;
            status = State::Out;
            let mut prev_pipe: Option<Pipe> = None;
            
            let mut flag_f = false;
            let mut flag_l = false;
            let mut flag_r = false; // reverse...

            for chaos in entropy {
                // dbg!(&chaos);
                match prev_pipe {
                    // Should never start with '-'
                    None => {
                        if chaos.symbol == 'F' {
                            flag_f = true;
                            status = State::In;
                        } else if chaos.symbol == 'L' {
                            flag_l = true;
                            status = State::In;
                        } else {
                            status = State::In;
                        }
                        prev_pipe = Some(chaos.clone());
                    }
                    Some(ref pipe) => {
                        
                        match status {
                            State::In => {
                                if  chaos.symbol == '-'
                                {
                                    // Status stays IN
                                    status = State::In;
                                } else if chaos.symbol == '7' {
                                    if flag_f {
                                        flag_f = false;
                                        if flag_r {
                                            status = State::In;
                                        } else {
                                            status = State::Out;
                                        }
                                        flag_r = false;
                                    } else if flag_l {
                                        flag_l = false;
                                        if flag_r {
                                            status = State::Out;
                                        } else {
                                            status = State::In;
                                        }
                                        flag_r = false;
                                    }
                                } else if chaos.symbol == 'J' {
                                    if flag_f {
                                        flag_f = false;
                                        if flag_r {
                                            status = State::Out;
                                        } else {
                                            status = State::In;
                                        }
                                        flag_r = false;
                                    } else if flag_l {
                                        flag_l = false;
                                        if flag_r {
                                            status = State::In;
                                        } else {
                                            status = State::Out;
                                        }
                                        flag_r = false;
                                    }
                                } else if chaos.symbol == 'F' {
                                    flag_f = true;
                                    flag_r = true;
                                    status = State::In;
                                } else if chaos.symbol == 'L' {
                                    flag_l = true;
                                    flag_r = true;
                                    status = State::In;
                                } else {
                                    status = State::Out;
                                }
                                count += chaos.x - pipe.x - 1;
                                prev_pipe = Some(chaos.clone());
                            }
                            State::Out => {
                                // Out will never start with '-'   
                                if chaos.symbol == 'F'
                                {
                                    flag_f = true;
                                    status = State::In;
                                } else if chaos.symbol == 'L' {
                                    flag_l = true;
                                    status = State::In;
                                } else {
                                    status = State::In;
                                }
                                prev_pipe = Some(chaos.clone());
                            }
                        }
                    }
                }
            }
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
        // dbg!(&runner.memory);
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
