use crate::map::{Map, Tile};
use crate::settings::Difficulty;

pub struct Game {
    map: Map,
}

impl Game {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            map: Map::new(difficulty),
        }
    }

    pub fn width(&self) -> usize {
        self.map.width()
    }

    pub fn height(&self) -> usize {
        self.map.height()
    }

    pub fn tile(&self, x: usize, y: usize) -> Tile {
        self.map.tile(x, y)
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        self.map.reveal(x, y);
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        self.map.toggle_flag(x, y);
    }
}
