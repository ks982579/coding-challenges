use std::io::{Error, ErrorKind};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Coordinates {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Space {
    Galaxy(Coordinates),
    Void(Coordinates),
}

pub struct Universe {
    pub space: Vec<Vec<Space>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CosmicError;

pub fn expand_str(data: &str) -> String {
    let mut expanded_universe: String = String::new();
    let mut col_expansion: Vec<bool> = Vec::new();
    for (row_index, line) in data.lines().enumerate() {
        let mut expand_col = true;
        for (col_index, symbol) in line.char_indices() {
            // let expand: &mut bool = &mut col_expansion[index];
            // If galaxy, then don't expand column, but don't check if unnecessary.
            if expand_col && symbol == '#' {
                expand_col = false;
            }
            let truth: Option<&mut bool> = col_expansion.get_mut(col_index);
            match truth {
                None => {
                    col_expansion.push(symbol == '.');
                },
                Some(that) => {
                    *that &= symbol == '.';
                }
            }
        }
        if expand_col {
            expanded_universe.push_str(line);
        }
        expanded_universe.push_str(line);
        expanded_universe.push('\n');
    }
    // When Do we add column?
    String::new()
}

impl std::str::FromStr for Universe {
    type Err = Error;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let mut col: Vec<Space> = Vec::new();
        let mut grid: Vec<Vec<Space>> = Vec::new();
        for (y, line) in data.lines().enumerate() {
            println!("{}", line);
            for (x, symbol) in line.char_indices() {
                let current_pos = Coordinates {x: x as isize, y: y as isize};
                let point = match symbol {
                    '.' => Space::Void(current_pos),
                    '#' => Space::Galaxy(current_pos),
                    _ => return Err(Error::new(ErrorKind::InvalidData, "Unexpected Symbol.")),
                };
                col.push(point);
            }
            grid.push(col.clone());
        }
        Ok(Universe { space: grid })
    }
}
