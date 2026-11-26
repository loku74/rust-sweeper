use crate::settings::Difficulty;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty(usize),
    Mine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
    Marked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub cell: Cell,
    pub state: CellState,
}

pub struct Map {
    difficulty: Difficulty,
    tiles: Vec<Vec<Tile>>,
    /// Mines are placed on the first reveal, around the clicked cell,
    /// so the player never loses on their first move.
    mines_placed: bool,
}

impl Map {
    /// Creates a board with every tile hidden and no mines yet.
    pub fn new(difficulty: Difficulty) -> Self {
        let (width, height) = difficulty.dimensions();
        let hidden_empty = Tile {
            cell: Cell::Empty(0),
            state: CellState::Hidden,
        };

        Self {
            difficulty,
            tiles: vec![vec![hidden_empty; width]; height],
            mines_placed: false,
        }
    }

    pub fn width(&self) -> usize {
        self.difficulty.dimensions().0
    }

    pub fn height(&self) -> usize {
        self.difficulty.dimensions().1
    }

    /// Places the mines anywhere except the 3x3 square around `(first_x, first_y)`,
    /// then computes every empty cell's neighbour count.
    /// Only touches `cell`, so flags placed before the first reveal are kept.
    fn place_mines(&mut self, first_x: usize, first_y: usize) {
        let (width, height) = (self.width(), self.height());

        let mut candidates: Vec<usize> = (0..width * height)
            .filter(|&i| {
                let (x, y) = (i % width, i / width);
                x.abs_diff(first_x) > 1 || y.abs_diff(first_y) > 1
            })
            .collect();
        candidates.shuffle(&mut rand::rng());

        for &i in candidates.iter().take(self.difficulty.mines()) {
            self.tiles[i / width][i % width].cell = Cell::Mine;
        }

        for y in 0..height {
            for x in 0..width {
                if self.tiles[y][x].cell != Cell::Mine {
                    self.tiles[y][x].cell = Cell::Empty(count_adjacent_mines(&self.tiles, x, y));
                }
            }
        }

        self.mines_placed = true;
    }

    pub fn tile(&self, x: usize, y: usize) -> Tile {
        self.tiles[y][x]
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        // Only hidden cells can be revealed: flagged/marked ones are protected.
        if self.tiles[y][x].state != CellState::Hidden {
            return;
        }

        if !self.mines_placed {
            self.place_mines(x, y);
        }

        let tile = &mut self.tiles[y][x];
        tile.state = CellState::Revealed;

        if tile.cell == Cell::Empty(0) {
            let around: Vec<_> = neighbors(&self.tiles, x, y).collect();
            for (nx, ny) in around {
                self.reveal(nx, ny);
            }
        }
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        let tile = &mut self.tiles[y][x];
        match tile.state {
            CellState::Hidden => tile.state = CellState::Flagged,
            CellState::Flagged => tile.state = CellState::Marked,
            CellState::Marked => tile.state = CellState::Hidden,
            CellState::Revealed => {}
        }
    }
}

fn count_adjacent_mines(tiles: &[Vec<Tile>], x: usize, y: usize) -> usize {
    neighbors(tiles, x, y)
        .filter(|&(nx, ny)| tiles[ny][nx].cell == Cell::Mine)
        .count()
}

fn neighbors(tiles: &[Vec<Tile>], x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let height = tiles.len();
    let width = tiles.first().map_or(0, Vec::len);

    (-1..=1)
        .flat_map(|dy| (-1..=1).map(move |dx| (dx, dy)))
        .filter(|&offset| offset != (0, 0))
        .filter_map(move |(dx, dy)| {
            let nx = x.checked_add_signed(dx)?;
            let ny = y.checked_add_signed(dy)?;
            (nx < width && ny < height).then_some((nx, ny))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_reveal_is_never_a_mine() {
        for _ in 0..100 {
            let mut map = Map::new(Difficulty::Expert);
            map.reveal(5, 5);

            assert_eq!(map.tile(5, 5).state, CellState::Revealed);
            assert_ne!(map.tile(5, 5).cell, Cell::Mine);
        }
    }

    #[test]
    fn flags_placed_before_first_reveal_are_kept() {
        let mut map = Map::new(Difficulty::Beginner);
        map.toggle_flag(0, 0);
        map.reveal(5, 5);

        assert_eq!(map.tile(0, 0).state, CellState::Flagged);
    }
}
