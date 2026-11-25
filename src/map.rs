use crate::settings::Difficulty;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty(usize),
    Mine,
}

pub struct Map {
    cells: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(difficulty: Difficulty, first_x: usize, first_y: usize) -> Self {
        let (width, height) = difficulty.dimensions();
        let mut cells = vec![vec![Cell::Empty(0); width]; height];

        let mut candidates: Vec<usize> = (0..width * height)
            .filter(|&i| {
                let (x, y) = (i % width, i / width);
                x.abs_diff(first_x) > 1 || y.abs_diff(first_y) > 1
            })
            .collect();
        candidates.shuffle(&mut rand::rng());

        for &i in candidates.iter().take(difficulty.mines()) {
            cells[i / width][i % width] = Cell::Mine;
        }

        for y in 0..height {
            for x in 0..width {
                if cells[y][x] != Cell::Mine {
                    cells[y][x] = Cell::Empty(count_adjacent_mines(&cells, x, y));
                }
            }
        }

        Self { cells }
    }

    pub fn cells(&self) -> &[Vec<Cell>] {
        &self.cells
    }
}

fn count_adjacent_mines(cells: &[Vec<Cell>], x: usize, y: usize) -> usize {
    neighbors(cells, x, y)
        .filter(|&(nx, ny)| cells[ny][nx] == Cell::Mine)
        .count()
}

fn neighbors(cells: &[Vec<Cell>], x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let height = cells.len();
    let width = cells.first().map_or(0, Vec::len);

    (-1..=1)
        .flat_map(|dy| (-1..=1).map(move |dx| (dx, dy)))
        .filter(|&offset| offset != (0, 0))
        .filter_map(move |(dx, dy)| {
            let nx = x.checked_add_signed(dx)?;
            let ny = y.checked_add_signed(dy)?;
            (nx < width && ny < height).then_some((nx, ny))
        })
}
