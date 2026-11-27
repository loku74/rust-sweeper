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

    pub fn mines(&self) -> usize {
        self.difficulty.mines()
    }

    pub fn flags_placed(&self) -> usize {
        self.tiles
            .iter()
            .flatten()
            .filter(|tile| tile.state == CellState::Flagged)
            .count()
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

    /// What the player's left-click does, depending on the tile's state.
    pub fn reveal(&mut self, x: usize, y: usize) {
        match self.tiles[y][x].state {
            CellState::Hidden => self.open(x, y),
            CellState::Revealed => self.chord(x, y),
            // Flagged/marked tiles are protected.
            CellState::Flagged | CellState::Marked => {}
        }
    }

    /// Reveals a hidden tile; an `Empty(0)` spreads to its neighbours.
    /// Only ever touches hidden tiles, which is what stops the recursion.
    fn open(&mut self, x: usize, y: usize) {
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
                self.open(nx, ny);
            }
        }
    }

    /// Clicking a revealed number whose neighbouring flags match that number
    /// opens all its other hidden neighbours.
    fn chord(&mut self, x: usize, y: usize) {
        let Cell::Empty(n) = self.tiles[y][x].cell else {
            return;
        };

        let around: Vec<_> = neighbors(&self.tiles, x, y).collect();
        let flags = around
            .iter()
            .filter(|&&(nx, ny)| self.tiles[ny][nx].state == CellState::Flagged)
            .count();

        if flags == n {
            for (nx, ny) in around {
                self.open(nx, ny);
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

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
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

    #[test]
    fn chord_opens_neighbours_when_flags_match() {
        let mut map = Map::new(Difficulty::Expert);
        map.reveal(5, 5);

        // Find a revealed number that still has hidden neighbours.
        let (x, y) = (0..map.height())
            .flat_map(|y| (0..map.width()).map(move |x| (x, y)))
            .find(|&(x, y)| {
                let tile = map.tile(x, y);
                tile.state == CellState::Revealed
                    && tile.cell != Cell::Empty(0)
                    && neighbors(&map.tiles, x, y)
                        .any(|(nx, ny)| map.tile(nx, ny).state == CellState::Hidden)
            })
            .expect("the first reveal always opens at least one number");

        // Flag exactly the neighbouring mines, then chord.
        let around: Vec<_> = neighbors(&map.tiles, x, y).collect();
        for &(nx, ny) in &around {
            if map.tile(nx, ny).cell == Cell::Mine {
                map.toggle_flag(nx, ny);
            }
        }
        map.reveal(x, y);

        for (nx, ny) in around {
            let tile = map.tile(nx, ny);
            let expected = if tile.cell == Cell::Mine {
                CellState::Flagged
            } else {
                CellState::Revealed
            };
            assert_eq!(tile.state, expected);
        }
    }

    #[test]
    fn flags_placed_counts_only_flags() {
        let mut map = Map::new(Difficulty::Beginner);
        map.toggle_flag(0, 0); // Hidden -> Flagged
        map.toggle_flag(1, 0); // Hidden -> Flagged
        map.toggle_flag(1, 0); // Flagged -> Marked: no longer a flag

        assert_eq!(map.flags_placed(), 1);
    }
}
