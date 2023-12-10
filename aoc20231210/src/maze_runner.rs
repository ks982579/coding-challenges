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
    fn get_char_at_position(&self, pos: &(usize, usize)) -> char {
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
    pub fn traverse_maze(&mut self, maze: &PipeMaze) {
        self.position = maze.start.clone();
        let loop_start = true;
        self.current_pipe = 'S';
        loop {
            match self.current_pipe {
                '|' => {
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
                }


                _ => { panic!("Non-matching Character: {}", &self.current_pipe) }
            }
        }
        // --------------------------------
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
        let maze = PipeMaze::from_str(puzzle);
        let mut runner = MazeRunner::default();
        runner.traverse_maze(&maze);
        dbg!(&runner);
        assert_eq!(runner.steps, 16);
    }
}