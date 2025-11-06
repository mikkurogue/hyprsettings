use gpui::*;
use gpui_component::{StyledExt, ActiveTheme};

pub fn item_pill<T>(cx: &mut Context<T>) -> Div {
    div()
        .h_flex()
        .gap_1()
        .px_2()
        .py_1()
        .border_1()
        .border_color(cx.theme().colors.border)
        .rounded_md()
        .bg(cx.theme().colors.background)
        .text_color(cx.theme().colors.foreground)
        .items_center()
}
