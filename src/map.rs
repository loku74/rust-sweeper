use crate::settings::Difficulty;
use rand::seq::SliceRandom;

/// What a cell contains.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty(usize),
    Mine,
}

/// What the player has done with a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
    /// "?" mark: the player is unsure.
    Marked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub cell: Cell,
    pub state: CellState,
}

pub struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(difficulty: Difficulty, first_x: usize, first_y: usize) -> Self {
        let (width, height) = difficulty.dimensions();
        let hidden_empty = Tile {
            cell: Cell::Empty(0),
            state: CellState::Hidden,
        };
        let mut tiles = vec![vec![hidden_empty; width]; height];

        let mut candidates: Vec<usize> = (0..width * height)
            .filter(|&i| {
                let (x, y) = (i % width, i / width);
                x.abs_diff(first_x) > 1 || y.abs_diff(first_y) > 1
            })
            .collect();
        candidates.shuffle(&mut rand::rng());

        for &i in candidates.iter().take(difficulty.mines()) {
            tiles[i / width][i % width].cell = Cell::Mine;
        }

        for y in 0..height {
            for x in 0..width {
                if tiles[y][x].cell != Cell::Mine {
                    tiles[y][x].cell = Cell::Empty(count_adjacent_mines(&tiles, x, y));
                }
            }
        }

        Self { tiles }
    }

    pub fn tiles(&self) -> &[Vec<Tile>] {
        &self.tiles
    }

    pub fn tile(&self, x: usize, y: usize) -> Tile {
        self.tiles[y][x]
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        let tile = &mut self.tiles[y][x];
        // Only hidden cells can be revealed: flagged/marked ones are protected.
        if tile.state == CellState::Hidden {
            tile.state = CellState::Revealed;
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
