use gpui::*;
use gpui_component::{
    StyledExt,
    scroll::{Scrollable, ScrollbarAxis},
};

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
    div().w_full().h_1().rounded_sm().bg(cx.theme().background)
}

/// Main container for the application and any subsequent modals/dialogs/popups/sub-windows
pub fn main_container<T>(cx: &mut Context<T>) -> Scrollable<Div> {
    div()
        .v_flex()
        .gap_4()
        .size_full()
        .scrollable(ScrollbarAxis::Vertical)
        .p_4()
        .bg(cx.theme().background)
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

/// Common sub-container style for sections
pub fn section_sub_container<T>(cx: &mut Context<T>) -> Div {
    div()
        .h_flex()
        .gap_2()
        .p_3()
        .border_1()
        .border_color(cx.theme().border)
        .rounded_sm()
        .bg(cx.theme().background)
}
