use std::marker::Sized;
use std::ops::{Add, Sub};

pub trait Number: Add + Sub + Sized {}

pub trait GreatMind<T> {
    fn remember(&mut self, info: T);
    fn find_engine_parts(&self) -> Vec<u32>;
    fn find_gear_parts(&self) -> Vec<GearPart>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryCell<'l> {
    pub before: Option<&'l str>,
    pub current: Option<&'l str>,
    pub after: Option<&'l str>,
    pub vert_line: u32,
}

impl<'l> Default for MemoryCell<'l> {
    fn default() -> Self {
        MemoryCell {
            before: None,
            current: None,
            after: None,
            vert_line: 0,
        }
    }
}

impl<'l> GreatMind<&'l str> for MemoryCell<'l> {
    fn remember(&mut self, info: &'l str) {
        match self.after {
            None => self.after = Some(info),
            Some(s_mem) => {
                self.after = Some(info);
                match self.current {
                    None => self.current = Some(s_mem),
                    Some(l_mem) => {
                        self.current = Some(s_mem);
                        self.before = Some(l_mem);
                    }
                }
            }
        }
    }

    fn find_engine_parts(&self) -> Vec<u32> {
        let mut engine_parts: Vec<u32> = Vec::new();
        match self.before {
            Some(b_mem) => {
                let mut c_mem: &str = "";
                let mut a_mem: &str = "";
                // pull out values
                if let Some(line) = self.current {
                    c_mem = &line;
                }
                if let Some(line) = self.after {
                    a_mem = &line;
                }
                let potential_parts: Vec<Part> = find_digits(&c_mem);
                // Here
                for potential_part in potential_parts {
                    let start: usize = (potential_part.start - 1).max(0) as usize;
                    let end: usize =
                        (potential_part.end + 1).min((c_mem.len() as i32)) as usize;
                    if found_symbols(&c_mem[start..end])
                        || found_symbols(&b_mem[start..end])
                        || found_symbols(&a_mem[start..end])
                    {
                        // dbg!(&b_mem[start..end]);
                        // dbg!(&c_mem[start..end]);
                        // dbg!(&a_mem[start..end]);
                        // dbg!("-- YES --");
                        engine_parts.push(potential_part.value);
                    } else {
                        // dbg!(&b_mem[start..end]);
                        // dbg!(&c_mem[start..end]);
                        // dbg!(&a_mem[start..end]);
                        // dbg!("-- NO --");
                    }
                }
                return engine_parts;
            }
            None => match self.current {
                Some(c_mem) => {
                    let mut a_mem: &str = "";
                    // pull out values
                    if let Some(line) = self.after {
                        a_mem = &line;
                    }
                    let potential_parts: Vec<Part> = find_digits(&c_mem);
                    // Here
                    for potential_part in potential_parts {
                        let start: usize = (potential_part.start - 1).max(0) as usize;
                        let end: usize =
                            (potential_part.end + 1).min((c_mem.len() as i32) - 1) as usize;
                        if found_symbols(&c_mem[start..end]) || found_symbols(&a_mem[start..end]) {
                            engine_parts.push(potential_part.value);
                            // dbg!("-- YES --");
                        } else {
                            dbg!(&c_mem[start..end]);
                            dbg!(&a_mem[start..end]);
                            dbg!("-- NO --");
                        }
                    }
                    return engine_parts;
                }
                None => return engine_parts,
            },
        }
    }

    fn find_gear_parts(&self) -> Vec<GearPart> {
        let mut engine_parts: Vec<u32> = Vec::new();
        let mut potential_gears: Vec<GearPart> = Vec::new();
        match self.before {
            Some(b_mem) => {
                let mut c_mem: &str = "";
                let mut a_mem: &str = "";
                // pull out values
                if let Some(line) = self.current {
                    c_mem = &line;
                }
                if let Some(line) = self.after {
                    a_mem = &line;
                }
                let potential_parts: Vec<Part> = find_digits(&c_mem);
                // Here
                for potential_part in potential_parts {
                    let start: usize = (potential_part.start - 1).max(0) as usize;
                    let end: usize =
                        (potential_part.end + 1).min((c_mem.len() as i32)) as usize;
                    let b_astr: Vec<usize> = find_astr(&b_mem[start..end]);
                    let c_astr: Vec<usize> = find_astr(&c_mem[start..end]);
                    let a_astr: Vec<usize> = find_astr(&a_mem[start..end]);
                    for top in b_astr {
                        let tmp = GearPart {
                            value: potential_part.value.clone(),
                            coordinate: ((top + start) as u32, self.vert_line.clone() - 1),
                        };
                        potential_gears.push(tmp);
                    }
                    for mid in c_astr {
                        let tmp = GearPart {
                            value: potential_part.value.clone(),
                            coordinate: ((mid + start) as u32, self.vert_line.clone()),
                        };
                        potential_gears.push(tmp);
                    }
                    for bot in a_astr {
                        let tmp = GearPart {
                            value: potential_part.value.clone(),
                            coordinate: ((bot + start) as u32, self.vert_line.clone() + 1),
                        };
                        potential_gears.push(tmp);
                    }
                }
            }
            None => match self.current {
                Some(c_mem) => {
                    let mut a_mem: &str = "";
                    // pull out values
                    if let Some(line) = self.after {
                        a_mem = &line;
                    }
                    let potential_parts: Vec<Part> = find_digits(&c_mem);
                    // Here
                    for potential_part in potential_parts {
                        let start: usize = (potential_part.start - 1).max(0) as usize;
                        let end: usize =
                            (potential_part.end + 1).min((c_mem.len() as i32)) as usize;
                        let c_astr: Vec<usize> = find_astr(&c_mem[start..end]);
                        let a_astr: Vec<usize> = find_astr(&a_mem[start..end]);
                        
                        for mid in c_astr {
                            let tmp = GearPart {
                                value: potential_part.value.clone(),
                                coordinate: ((mid + start) as u32, self.vert_line.clone()),
                            };
                            potential_gears.push(tmp);
                        }
                        for bot in a_astr {
                            let tmp = GearPart {
                                value: potential_part.value.clone(),
                                coordinate: ((bot + start) as u32, self.vert_line.clone() + 1),
                            };
                            potential_gears.push(tmp);
                        }
                    }
                }
                None => return potential_gears,
            },
        }
        potential_gears
        // let mut gear_ratios: Vec<u32> = Vec::new();
        // let mut tmp_gears: Vec<GearPart> = Vec::new();
        // for potential_gear in potential_gears {
        //     for tgear in tmp_gears.iter() {
        //         if potential_gear.coordinate == tgear.coordinate {
        //             gear_ratios.push(tgear.value.clone() * potential_gear.value.clone());
        //         }
        //     }
        //     tmp_gears.push(potential_gear);
        // }
        // gear_ratios
    }
}

fn find_astr(line: &str) -> Vec<usize> {
    let mut x_coords: Vec<usize> = Vec::new();
    for c in line.char_indices() { //(usize, char)
        if c.1 == '*' {
            x_coords.push(c.0);
        }
    }
    x_coords
}

fn found_symbols(line: &str) -> bool {
    for c in line.chars() {
        if c == '.' || c.is_ascii_digit() {
            continue;
        } else {
            return true;
        }
    }
    false
}

fn find_digits(line: &str) -> Vec<Part> {
    let mut coordinates: Vec<Part> = Vec::new();
    let mut digits: String = String::new();
    let mut begin: i32 = -1;
    let mut end: i32 = -1;

    for c in line.char_indices() {
        // (Index, Char)
        // dbg!(c);
        if c.1.is_ascii_digit() {
            if begin == -1 {
                begin = c.0 as i32;
            } else {
                end = c.0 as i32;
            }
            digits.push(c.1);
            continue;
        } else {
            if begin == -1 {
                continue;
            } else {
                // single digit check...
                if end == -1 {
                    end = begin.clone();
                }
                // reset
                // dbg!(&digits);
                coordinates.push(Part {
                    value: digits.parse::<u32>().unwrap(),
                    start: begin.clone(),
                    end: end.clone() + 1,
                });
                begin = -1;
                end = -1;
                digits.clear();
            }
        }
    }
    // Edge case... literally if number on edge:
    if begin != -1 {
        if end == -1 {
            end = begin.clone();
        }
        coordinates.push(
            Part {
                value: digits.parse::<u32>().unwrap(),
                start: begin.clone(),
                end: end.clone() + 1,
            }
        )
    }
    coordinates
}
// pub fn
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Part {
    value: u32,
    start: i32,
    end: i32,
}
fn find_gear(line: &str) -> Vec<Part> {
    let mut coordinates: Vec<Part> = Vec::new();
    let mut digits: String = String::new();
    let mut begin: i32 = -1;
    let mut end: i32 = -1;

    for c in line.char_indices() {
        // (Index, Char)
        dbg!(c);
        if c.1.is_ascii_digit() {
            if begin == -1 {
                begin = c.0 as i32;
            } else {
                end = c.0 as i32;
            }
            digits.push(c.1);
            continue;
        } else {
            if begin == -1 {
                continue;
            } else {
                // single digit check...
                if end == -1 {
                    end = begin.clone();
                }
                // reset
                // dbg!(&digits);
                coordinates.push(Part {
                    value: digits.parse::<u32>().unwrap(),
                    start: begin.clone(),
                    end: end.clone() + 1,
                });
                begin = -1;
                end = -1;
                digits.clear();
            }
        }
    }
    // Edge case... literally if number on edge:
    if begin != -1 {
        if end == -1 {
            end = begin.clone();
        }
        coordinates.push(
            Part {
                value: digits.parse::<u32>().unwrap(),
                start: begin.clone(),
                end: end.clone() + 1,
            }
        )
    }
    coordinates
}
pub struct GearPart {
    pub value: u32,
    pub coordinate: (u32, u32)
}


#[cfg(test)]
mod test {
    use super::{find_digits, GreatMind, MemoryCell, Part};

    #[test]
    fn test_tests() {
        assert!(true);
    }
    #[test]
    fn make_memory_cell() {
        let expected: MemoryCell = MemoryCell {
            before: Some("hello"),
            current: Some("world"),
            after: Some("AoC2023"),
            vert_line: 0,
        };
        let mut actual: MemoryCell = MemoryCell::default();
        actual.remember("rust");
        actual.remember("hello");
        actual.remember("world");
        actual.remember("AoC2023");
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_find_digits() {
        let line: &str = ".12.#567..";
        let expected = vec![
            Part {
                value: 12,
                start: 1,
                end: 2,
            },
            Part {
                value: 567,
                start: 5,
                end: 7,
            },
        ];
        let actual = find_digits(line);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_check_digits() {
        let line: &str = "0123456789";
        let start = 0;
        let end = 4;
        let line_slice: &str = &line[start..end];
        dbg!(line_slice);
        assert!(false);
    }
}
