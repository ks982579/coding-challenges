pub enum Cubes {
    RED(i32),
    Blue(i32),
    Green(i32),
}

struct Round {
    cubes: Vec<Cubes>,
}
struct Game {
    id: i32,
    rounds: Vec<Round>,
}