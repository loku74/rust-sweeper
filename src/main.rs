mod game;
mod map;
mod settings;

use game::Game;
use map::CellState;
use settings::Difficulty;

const MAP_BORDER_COLOR: u32 = 0x909090;
const CELL_BORDER_COLOR: u32 = MAP_BORDER_COLOR;
const CELL_HIDDEN_BG_COLOR: u32 = 0xe7e7e7;
const CELL_REVEALED_BG_COLOR: u32 = 0xc5c5c5;

use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

use crate::map::{Cell, Tile};

fn number_color(n: usize) -> Rgba {
    match n {
        1 => rgb(0x2563eb), // blue
        2 => rgb(0x16a34a), // green
        3 => rgb(0xdc2626), // red
        4 => rgb(0x7c3aed), // purple
        5 => rgb(0xb45309), // brown
        6 => rgb(0x0891b2), // teal
        7 => rgb(0x111111), // near-black
        _ => rgb(0x6b7280), // 8: grey
    }
}

fn cell(id: usize, tile: Option<Tile>) -> base::Button {
    let revealed = tile.is_some_and(|tile| tile.state == CellState::Revealed);

    let (label, color) = match tile {
        None => (String::new(), None),
        Some(tile) => match (tile.state, tile.cell) {
            (CellState::Revealed, Cell::Empty(0)) => (String::new(), None),
            (CellState::Revealed, Cell::Empty(n)) => (n.to_string(), Some(number_color(n))),
            (CellState::Revealed, Cell::Mine) => ("*".into(), None),
            (CellState::Flagged, _) => ("F".into(), None),
            (CellState::Marked, _) => ("?".into(), None),
            (CellState::Hidden, _) => (String::new(), None),
        },
    };

    base::Button::new(("cell", id))
        .group("cell")
        .relative()
        .size_8()
        .border_b_1()
        .border_r_1()
        .border_color(rgb(CELL_BORDER_COLOR))
        .disabled(revealed)
        .when(revealed, |this| this.bg(rgb(CELL_REVEALED_BG_COLOR)))
        .when(!revealed, |this| {
            this.bg(rgb(CELL_HIDDEN_BG_COLOR)).cursor_pointer().child(
                div()
                    .absolute()
                    .inset_0()
                    .group_hover("cell", |style| style.bg(white().opacity(0.2))),
            )
        })
        .when_some(color, |this, color| this.text_color(color))
        .child(label)
}

impl Render for Game {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut grid = div()
            .flex()
            .flex_col()
            .border_t_1()
            .border_l_1()
            .border_color(rgb(MAP_BORDER_COLOR));

        for y in 0..self.height() {
            let mut row = div().flex();

            for x in 0..self.width() {
                let id = y * self.width() + x;
                let button =
                    cell(id, self.tile(x, y)).on_click(cx.listener(move |this, _, _, cx| {
                        this.reveal(x, y);
                        cx.notify();
                    }));
                row = row.child(button);
            }

            grid = grid.child(row);
        }

        div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .child(grid)
    }
}

fn main() {
    application().with_assets(assets::Assets).run(|cx| {
        init(cx);

        open_window(WindowOptions::default(), cx, |_, cx| {
            let game = Game::new(Difficulty::Beginner);

            cx.new(|_| game)
        })
        .expect("Failed to open window");
    });
}
