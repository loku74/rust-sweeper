use crate::map::{CellState, Map, Tile};
use crate::settings::Difficulty;

pub struct Game {
    difficulty: Difficulty,
    /// `None` until the first reveal: mines are placed around the first click
    /// so the player never loses on their first move.
    map: Option<Map>,
}

impl Game {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            difficulty,
            map: None,
        }
    }

    pub fn width(&self) -> usize {
        self.difficulty.dimensions().0
    }

    pub fn height(&self) -> usize {
        self.difficulty.dimensions().1
    }

    pub fn tile(&self, x: usize, y: usize) -> Option<Tile> {
        self.map.as_ref().map(|map| map.tile(x, y))
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        self.map
            .get_or_insert_with(|| Map::new(self.difficulty, x, y))
            .reveal(x, y);
    }
}
