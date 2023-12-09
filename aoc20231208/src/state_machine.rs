use num::integer::lcm;

fn parse_tup(tup: &str) -> [&str; 2] {
    let to_trim: &[_] = &[' ', '(', ')', '\n']; 
    let mut tmp_tup: &str = tup.trim_matches(to_trim);
    let mut return_tup: [&str;2] = ["";2];
    for node in tmp_tup.split(", ").enumerate() {
        return_tup[node.0] = node.1;
    }
    return_tup
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Instructions {
    pub directions: Vec<char>,
}

impl Instructions {
    pub fn from_str(directions: &str) -> Instructions {
        let mut key: Vec<char> = Vec::with_capacity(directions.len());
        for d in directions.chars() {
            key.push(d.clone());   
        }
        Instructions {
            directions: key,
        }
    }
}


// ------------------------------------


#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Node<'l> {
    pub id: &'l str,
    pub left: &'l str,
    pub right: &'l str,
}

impl<'a> Node<'a> {
    pub fn from_str(data: &'a str) -> Node {
        let mut split_data: Vec<&str> = Vec::with_capacity(2); 
        for datum in data.split("=").into_iter() {
            split_data.push(datum.trim());
        }
        let lr = parse_tup(split_data[1]);
        Node {
            id: split_data[0],
            left: lr[0],
            right: lr[1],
        }
    }
}

pub fn count_moves(instructions: &Instructions, directions: &Vec<Node>) -> usize {
    let mut moves: usize = 0;
    let mut iterations: usize = 0;
    let mut memory: &str = "AAA";
    while memory != "ZZZ" {
        iterations += 1;
        for e in instructions.directions.iter() {
            // probably could have created a huge hashmap
            for d in directions.iter() {
                if d.id == memory {
                    memory = if *e == 'L' {d.left} else {d.right};
                    moves += 1;
                    break;
                }
            }
            if memory == "ZZZ" {
                break;
            }
        }
        println!("Position at end of Iteration {}: {}", &iterations, &memory);
    }
    moves
} 


pub fn count_ghost_moves(inst: &Instructions, dir: &Vec<Node>) -> usize {
    let mut moves: usize = 0;
    let mut iterations: usize = 0;
    let mut memory: Vec<&str> = Vec::new();
    let mut ghost_truth: bool = true;
    
    // Load starting positions
    for en in dir.iter() {
        if en.id.ends_with('A') {
            memory.push(en.id);
        }
    }

    // check all paths at once...
    while ghost_truth {
        for einst in inst.directions.iter() {
            for emem in memory.iter_mut() {
                for d in dir.iter() {
                    if d.id == *emem {
                        *emem = if *einst == 'L' {d.left} else {d.right};
                        break
                    }
                }
            }   
            moves += 1;
            ghost_truth = false;
            for emem in memory.iter() {
                if !emem.ends_with('Z') {
                    ghost_truth = true;
                    break;
                } 
            }
            if !ghost_truth {
                break;
            }
        }
        iterations += 1;
        println!("Move: {} - Iteration {}", &moves, &iterations);
    }
    moves
}

pub fn count_ghost_moves_multiple(inst: &Instructions, dir: &Vec<Node>) -> usize {
    // let mut moves: usize = 0;
    let mut memory: Vec<&str> = Vec::new();
    let mut ghost_moves: Vec<usize> = Vec::new();
    
    // Load starting positions
    for en in dir.iter() {
        if en.id.ends_with('A') {
            memory.push(en.id.clone());
        }
    }

    dbg!(&memory);

    for mem in memory {
        // mem = "AAA"
        let mut moves: usize = 0;
        let mut remem: &str = mem;
        let mut ghost_truth: bool = true;
        dbg!(remem);

        while ghost_truth {
            // println!("HERE 1");
            for einst in inst.directions.iter() {
                for d in dir.iter() {
                    if d.id == remem {
                        remem = if *einst == 'L' {d.left} else {d.right};
                        moves += 1;
                        break;
                    }
                }
                if remem.ends_with('Z') {
                    ghost_truth = false;
                    break;
                }
            }
        }
        ghost_moves.push(moves);
        println!("{} takes {} moves", mem, &moves);
    }

    let least_thing = ghost_moves.into_iter()
        .reduce(|acc, x| lcm(acc, x)).unwrap();
    least_thing
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_tests() {
        assert!(true);
    }
    
    #[test]
    fn test_parse_tuple() {
        let data: &str = " (AAA, BBB)\n";
        let actual: [&str;2] = parse_tup(data);
        dbg!(data);
        dbg!(actual);
        assert_eq!(["AAA", "BBB"], actual);
    }
    #[test]
    fn test_create_instructions_from_str() {
        let data1: &str = "LRLRRLRL";
        let data2: &str = "LRLRRLRR";
        let actual: Instructions = Instructions::from_str(data1);
        let fake: Instructions = Instructions::from_str(data2);
        let expected: Instructions = Instructions {
            directions: vec!['L','R','L','R','R','L','R','L']
        };
        assert_eq!(actual, expected);
        assert_ne!(actual, fake);
    }
    #[test]
    fn test_create_node_from_str() {
        let data1: &str = "ABC = (BBB, CCC)";
        let data2: &str = "AAF = (BBB, CCD)";
        let actual: Node = Node::from_str(data1);
        let fake: Node = Node::from_str(data2);
        let expected: Node = Node {
            id: "ABC",
            left: "BBB",
            right: "CCC",
        };
        assert_eq!(actual, expected);
        assert_ne!(actual, fake);
    }
    #[test]
    fn test_count_moves(){
        let puz = String::from_str(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"
        ).unwrap();

        let mut instructions: Instructions = Instructions::default();
        let mut nodes: Vec::<Node> = Vec::with_capacity(puz.lines().count());

        for line_tup in puz.lines().enumerate() {
            // line_tup = (index: usize, line: &str)
            if line_tup.0 == 0 {
                instructions = Instructions::from_str(line_tup.1);
            } else if line_tup.1.trim() != "" {
                nodes.push(
                    Node::from_str(line_tup.1)
                );
            }
        }
        let actual: usize = count_moves(&instructions, &nodes);
        let expected: usize = 6;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_count_ghost_moves(){
        let puz = String::from_str(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"
        ).unwrap();

        let mut instructions: Instructions = Instructions::default();
        let mut nodes: Vec::<Node> = Vec::with_capacity(puz.lines().count());

        for line_tup in puz.lines().enumerate() {
            // line_tup = (index: usize, line: &str)
            if line_tup.0 == 0 {
                instructions = Instructions::from_str(line_tup.1);
            } else if line_tup.1.trim() != "" {
                nodes.push(
                    Node::from_str(line_tup.1)
                );
            }
        }
        let actual: usize = count_ghost_moves(&instructions, &nodes);
        let expected: usize = 6;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_count_ghost_moves_lcm(){
        let puz = String::from_str(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"
        ).unwrap();

        let mut instructions: Instructions = Instructions::default();
        let mut nodes: Vec::<Node> = Vec::with_capacity(puz.lines().count());

        for line_tup in puz.lines().enumerate() {
            // line_tup = (index: usize, line: &str)
            if line_tup.0 == 0 {
                instructions = Instructions::from_str(line_tup.1);
            } else if line_tup.1.trim() != "" {
                nodes.push(
                    Node::from_str(line_tup.1)
                );
            }
        }
        let actual: usize = count_ghost_moves_multiple(&instructions, &nodes);
        let expected: usize = 6;
        assert_eq!(actual, expected);
    }
}