mod game;
mod map;
mod settings;
mod ui;

use gpui_kit::*;

use game::Game;
use settings::Difficulty;
use ui::theme::Theme;

fn main() {
    application().with_assets(assets::Assets).run(|cx| {
        init(cx);
        cx.set_global(Theme::dark());

        open_window(WindowOptions::default(), cx, |_, cx| {
            let game = Game::new(Difficulty::Beginner);

            cx.new(|_| game)
        })
        .expect("Failed to open window");
    });
}
