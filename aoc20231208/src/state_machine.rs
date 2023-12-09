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

#[cfg(test)]
mod tests {
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
}