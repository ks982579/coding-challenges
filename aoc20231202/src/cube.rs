#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Round {
    pub red: i32,
    pub blue: i32,
    pub green: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Game {
    pub id: i32,
    pub possible: bool,
    pub rounds: Vec<Round>,
}

impl Round {
    pub fn from_str(text: &str) -> Self {
        let mut red_cubes: i32 = 0;
        let mut blue_cubes: i32 = 0;
        let mut green_cubes: i32 = 0;

        let list_of_cubes: Vec<&str> = text.trim().split(',').collect();
        for each_cube in list_of_cubes {
            if each_cube.contains("red") {
                red_cubes = each_cube.trim().split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
            } else if each_cube.contains("blue") {
                blue_cubes = each_cube.trim().split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
            } else {
                green_cubes = each_cube.trim().split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(); 
            }
        }
        Round {
            red: red_cubes,
            blue: blue_cubes,
            green: green_cubes,
        }
    }
    pub fn get_set_power(&self) -> i32 {
        self.red * self.blue * self.green
    }
}

impl Game {
    fn extract_game_id(game_str: &str) -> i32 {
        /* This is to ... */
        let game_str_split: Vec<&str> = game_str.trim().split(' ').collect();
        game_str_split[1].parse::<i32>().unwrap()
    }

    pub fn from_str(text: &str, max_round: Option<&Round>) -> Self {
        let game_split: Vec<&str> = text.split(":").collect();
        // extract ID
        let game_id: i32 = Game::extract_game_id(game_split[0]); 
        let mut list_of_rounds: Vec<Round> = Vec::new();

        // The initial value was apparently consumed when extracting ID
        for round in game_split[1].split(';').collect::<Vec<&str>>() {
            let current_round = Round::from_str(round);
            list_of_rounds.push(current_round);
        }
        let possible: bool = match max_round {
            None => true,
            Some(the_max) => {
                list_of_rounds.iter().all(|x| {
                    let redok: bool = x.red <= the_max.red;
                    let blueok: bool = x.blue <= the_max.blue;
                    let greenok: bool = x.green <= the_max.green;
                    redok && blueok && greenok
                })
            }
        };
        Game {
            id: game_id,
            possible: possible,
            rounds: list_of_rounds,
        }
    }
    pub fn find_min_set(&self) -> Round {
        let mut reds: Vec<i32> = Vec::new();
        let mut blues: Vec<i32> = Vec::new();
        let mut greens: Vec<i32> = Vec::new();
        for round in self.rounds.iter() {
            reds.push(round.red);
            blues.push(round.blue);
            greens.push(round.green);
        }
        Round {
            red: *reds.iter().max().unwrap(),
            blue: *blues.iter().max().unwrap(),
            green: *greens.iter().max().unwrap(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Game, Round};

    #[test]
    fn test_setup() {
        assert_eq!(2,2);
    }
    #[test]
    fn test_game_split() {
        let expected: i32 = 42;
        let actual: i32 = Game::extract_game_id("Game 42");
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_create_round() {
        let expected: Round = Round {
            red: 3,
            blue: 4,
            green: 5
        };
        let actual: Round = Round::from_str(
            "3 red, 4 blue, 5 green"
        );
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_check_round() {
        let lesser: Round = Round {
            red: 3,
            blue: 4,
            green: 4
        };
        let greater: Round = Round::from_str(
            "3 red, 4 blue, 5 green"
        );
        assert!(lesser < greater);
    }
    #[test]
    fn test_create_possible_game() {
        let max_round: Round = Round {
            red: 100,
            blue: 100,
            green: 100,
        };
        let expected: Game = Game {
            id: 1,
            possible: true,
            rounds: vec![
                Round {
                    red: 4,
                    blue: 3,
                    green: 0,
                },
                Round {
                    red: 0,
                    blue: 6,
                    green: 2
                }
            ]
        };
        let actual: Game = Game::from_str(
            "Game 1: 3 blue, 4 red; 6 blue, 2 green",
            Some(&max_round),
        );
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_create_impossible_game() {
        let max_round: Round = Round {
            red: 10,
            blue: 10,
            green: 10,
        };
        let expected: Game = Game {
            id: 1,
            possible: false,
            rounds: vec![
                Round {
                    red: 14,
                    blue: 3,
                    green: 0,
                },
                Round {
                    red: 0,
                    blue: 6,
                    green: 2
                }
            ]
        };
        let actual: Game = Game::from_str(
            "Game 1: 3 blue, 14 red; 6 blue, 2 green",
            Some(&max_round),
        );
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_get_min_set() {
        let test_game: Game = Game {
            id: 1,
            possible: false,
            rounds: vec![
                Round {
                    red: 42,
                    blue: 3,
                    green: 0,
                },
                Round {
                    red: 0,
                    blue: 42,
                    green: 12,
                },
                 Round {
                    red: 0,
                    blue: 6,
                    green: 42,
                }
           ]
        };
        let actual: Round = test_game.find_min_set();
        let expected: Round = Round {
            red: 42,
            blue: 42,
            green: 42,
        };
        assert_eq!(actual, expected);
    }
}