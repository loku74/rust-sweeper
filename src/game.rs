use crate::map::{CellState, Map, Tile};
use crate::settings::Difficulty;

pub struct Game {
    map: Map,
    /// The tile the left mouse button is held down on, if any.
    pressed: Option<(usize, usize)>,
}

impl Game {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            map: Map::new(difficulty),
            pressed: None,
        }
    }

    pub fn press(&mut self, x: usize, y: usize) {
        self.pressed = Some((x, y));
    }

    pub fn release(&mut self) {
        self.pressed = None;
    }

    /// Whether `(x, y)` should be drawn as "pushed in" while the button is held.
    /// Pressing a revealed number previews its whole 3x3 square (a chord);
    /// pressing a hidden tile only pushes in that one tile.
    pub fn is_pressed(&self, x: usize, y: usize) -> bool {
        let Some((px, py)) = self.pressed else {
            return false;
        };

        if self.map.tile(px, py).state == CellState::Revealed {
            x.abs_diff(px) <= 1 && y.abs_diff(py) <= 1
        } else {
            (x, y) == (px, py)
        }
    }

    pub fn width(&self) -> usize {
        self.map.width()
    }

    pub fn height(&self) -> usize {
        self.map.height()
    }

    /// Signed: if the player places more flags than there are mines.
    pub fn mines_left(&self) -> isize {
        self.map.mines() as isize - self.map.flags_placed() as isize
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
