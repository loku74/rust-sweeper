// mod game;
// mod map;
// mod settings;
//
// use game::Game;
// use settings::Difficulty;
//
// fn main() {
//     let game = Game::new(Difficulty::Intermediate, 4, 4);
//     game.print_map();
// }
//
use gpui_kit::component::{
    button::{Button, ButtonVariants},
    label::Label,
};
use gpui_kit::*;

struct HelloWorld {
    count: u32,
}

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut grid = div().flex().flex_col().gap_2();

        for y in 0..5usize {
            let mut row = div().flex().gap_2();

            for x in 0..5usize {
                let button = Button::new(("cell", y * 5 + x))
                    .primary()
                    .label(format!("{x},{y}"))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.count += 1;
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
            .gap_2()
            .child(Label::new(format!("Clicked {} times", self.count)))
            .child(grid)
    }
}

fn main() {
    application().with_assets(assets::Assets).run(|cx| {
        init(cx);

        open_window(WindowOptions::default(), cx, |_, cx| {
            cx.new(|_| HelloWorld { count: 0 })
        })
        .expect("Failed to open window");
    });
}
