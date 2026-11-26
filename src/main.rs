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

fn cell(id: usize, state: CellState) -> base::Button {
    let revealed = state == CellState::Revealed;

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
                    cell(id, self.state(x, y)).on_click(cx.listener(move |this, _, _, cx| {
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
