use crate::map::{CellState, Map};
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

    /// Before the first click there is no map yet, so every cell is hidden.
    pub fn state(&self, x: usize, y: usize) -> CellState {
        self.map
            .as_ref()
            .map_or(CellState::Hidden, |map| map.tile(x, y).state)
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        // Create the map on the first reveal, then reveal on it.
        self.map
            .get_or_insert_with(|| Map::new(self.difficulty, x, y))
            .reveal(x, y);
    }

    pub fn print_map(&self) {
        if let Some(map) = &self.map {
            for row in map.tiles() {
                let cells: Vec<_> = row.iter().map(|tile| tile.cell).collect();
                println!("{cells:?}");
            }
        }
    }
}
