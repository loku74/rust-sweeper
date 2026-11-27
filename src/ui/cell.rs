use gpui_kit::*;
use gpui_kit::{base::StyledExt, prelude::FluentBuilder};

use super::theme::Theme;
use crate::map::{Cell, CellState, Tile};

pub fn cell(id: usize, tile: Tile, pressed: bool, theme: &Theme) -> base::Button {
    // A hidden tile under a held mouse button is drawn like an opened, empty one.
    let revealed =
        tile.state == CellState::Revealed || (pressed && tile.state == CellState::Hidden);

    let (label, color) = match (tile.state, tile.cell) {
        (CellState::Revealed, Cell::Empty(0)) => (String::new(), None),
        (CellState::Revealed, Cell::Empty(n)) => (n.to_string(), Some(theme.number(n))),
        (CellState::Revealed, Cell::Mine) => ("*".into(), None),
        (CellState::Flagged, _) => ("F".into(), None),
        (CellState::Marked, _) => ("?".into(), None),
        (CellState::Hidden, _) => (String::new(), None),
    };

    base::Button::new(("cell", id))
        .group("cell")
        .relative()
        .size_8()
        .border_b_1()
        .border_r_1()
        .border_color(theme.border)
        .when(revealed, |this| this.bg(theme.cell_revealed))
        .when(!revealed, |this| this.bg(theme.cell_hidden))
        .when_some(color, |this, color| this.text_color(color))
        .font_bold()
        .child(label)
        .child(
            div()
                .absolute()
                .inset_0()
                .group_hover("cell", |style| style.bg(theme.cell_hover)),
        )
}
