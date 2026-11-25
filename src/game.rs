use crate::map::Map;
use crate::settings::Difficulty;

pub struct Game {
    map: Map,
}

impl Game {
    pub fn new(difficulty: Difficulty, first_x: usize, first_y: usize) -> Self {
        Self {
            map: Map::new(difficulty, first_x, first_y),
        }
    }

    pub fn print_map(&self) {
        self.map
            .cells()
            .iter()
            .for_each(|line| println!("{line:?}"));
    }
}
