//! Everything that draws the game. The game logic itself lives in `game` and `map`.

mod button;
mod cell;
pub mod theme;
mod tooltip;

use gpui_kit::base::StyledExt;
use gpui_kit::*;

use crate::game::Game;
use button::bar_button;
use cell::cell;
use theme::Theme;
use tooltip::HelpTooltip;

impl Render for Game {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::get(cx);

        let container = div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .bg(theme.background)
            .text_color(theme.text);

        let layout = div().flex().flex_col().gap_4();

        let mines_left = div().font_semibold().child(self.mines_left().to_string());

        let difficulty_btn = bar_button("difficulty_btn", theme)
            .on_click(cx.listener(|this, _, _, cx| {
                this.set_difficulty(this.difficulty().next());
                cx.notify();
            }))
            .tooltip(|_, cx| cx.new(|_| HelpTooltip("Change the difficulty")).into())
            .child(self.difficulty().to_string());

        let theme_btn = bar_button("theme_btn", theme)
            .on_click(cx.listener(|_, _, _, cx| {
                let next = Theme::get(cx).mode.toggle();
                cx.set_global(Theme::from_mode(next));
                cx.notify();
            }))
            .tooltip(|_, cx| cx.new(|_| HelpTooltip("Switch light / dark mode")).into())
            .child(theme.mode.to_string());

        let bar = div()
            .flex()
            .items_center()
            .gap_8()
            .child(difficulty_btn)
            .child(mines_left)
            .child(theme_btn);

        let mut grid = div()
            .flex()
            .flex_col()
            .border_t_1()
            .border_l_1()
            .border_color(theme.border);

        for y in 0..self.height() {
            let mut row = div().flex();

            for x in 0..self.width() {
                let id = y * self.width() + x;
                let button = cell(id, self.tile(x, y), self.is_pressed(x, y), theme)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.reveal(x, y);
                        cx.notify();
                    }))
                    .on_aux_click(cx.listener(move |this, event: &ClickEvent, _, cx| {
                        if event.is_right_click() {
                            this.toggle_flag(x, y);
                            cx.notify();
                        }
                    }))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.press(x, y);
                            cx.notify();
                        }),
                    );
                row = row.child(button);
            }

            grid = grid.child(row);
        }

        // Release is handled once on the whole grid rather than on each cell:
        // `on_mouse_up` covers releasing over the grid, `on_mouse_up_out`
        // covers dragging off it first, so the preview can never get stuck.
        let release =
            |this: &mut Game, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Game>| {
                this.release();
                cx.notify();
            };
        let grid = grid
            .on_mouse_up(MouseButton::Left, cx.listener(release))
            .on_mouse_up_out(MouseButton::Left, cx.listener(release));

        container.child(layout.child(bar).child(grid))
    }
}
