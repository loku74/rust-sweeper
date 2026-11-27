use gpui_kit::*;

use super::theme::Theme;

pub fn bar_button(id: impl Into<ElementId>, theme: &Theme) -> base::Button {
    base::Button::new(id)
        .p_2()
        .border_1()
        .border_color(theme.border)
        .bg(theme.button_bg)
        .text_color(theme.text)
        .hover(|style| style.bg(theme.button_hover))
        .cursor_pointer()
}
