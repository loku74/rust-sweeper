use gpui_kit::*;

use super::theme::Theme;

/// A small text tooltip.
///
/// Use it with `.tooltip(|_, cx| cx.new(|_| HelpTooltip("...")).into())`.
pub struct HelpTooltip(pub &'static str);

impl Render for HelpTooltip {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::get(cx);

        div()
            .px_2()
            .py_1()
            .border_1()
            .border_color(theme.border)
            .bg(theme.surface)
            .text_color(theme.text)
            .text_xs()
            .shadow_md()
            .mt_4()
            .child(self.0)
    }
}
