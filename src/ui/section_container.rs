use gpui::*;
use gpui_component::StyledExt;

use gpui_component::ActiveTheme as _;

/// Common title style for sections
pub fn section_title<T>(title_text: impl IntoElement, cx: &mut Context<T>) -> Div {
    div()
        .text_xl()
        .font_weight(FontWeight::BOLD)
        .text_color(cx.theme().foreground)
        .child(title_text)
}

/// Common divider style for sections
pub fn section_divider<T>(cx: &mut Context<T>) -> Div {
    div()
        .w_full()
        .h_1()
        .rounded_sm()
        .bg(cx.theme().description_list_label)
}

/// Common container style for sections
pub fn section_container<T>(cx: &mut Context<T>) -> Div {
    div()
        .v_flex()
        .gap_2()
        .p_4()
        .border_1()
        .border_color(cx.theme().border)
        .rounded_sm()
        .bg(cx.theme().background)
}
