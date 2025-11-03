use gpui::*;
use gpui_component::button::Button;
use gpui_component::dropdown::*;
use std::collections::HashSet;

use gpui_component::StyledExt;

use crate::conf::{self, write_override_line};

pub struct KeyboardSettings {
    selected_locales: HashSet<String>,
    locale_dropdown: Entity<DropdownState<Vec<String>>>,
}

impl KeyboardSettings {
    pub fn new(window: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> Self {
        // example locales for now
        let locales = vec![
            "us".to_string(),
            "gb".to_string(),
            "fi".to_string(),
            "dk".to_string(),
            "no".to_string(),
            "de".to_string(),
        ];

        // Get current locales from hyprctl
        let selected_locales = crate::keyboard::get_current_locales().unwrap_or_else(|e| {
            eprintln!("Failed to get current locales: {}, using default", e);
            let mut default_set = HashSet::new();
            default_set.insert("fi".to_string());
            default_set
        });

        // Set initial dropdown selection to first locale in the set
        let current_locale_idx = selected_locales
            .iter()
            .next()
            .and_then(|locale| locales.iter().position(|l| l == locale));

        let locale_dropdown = cx.new(|cx| {
            DropdownState::new(
                locales.clone(),
                current_locale_idx.map(gpui_component::IndexPath::new),
                window,
                cx,
            )
        });

        // Subscribe to dropdown selection events
        cx.subscribe(
            &locale_dropdown,
            |this, _dropdown, event: &DropdownEvent<Vec<String>>, cx| {
                if let DropdownEvent::Confirm(Some(selected_value)) = event {
                    // HashSet automatically handles uniqueness
                    this.selected_locales.insert(selected_value.clone());
                    cx.notify();
                }
            },
        )
        .detach();

        KeyboardSettings {
            selected_locales,
            locale_dropdown,
        }
    }

    fn remove_locale(&mut self, locale: &str, cx: &mut Context<Self>) {
        self.selected_locales.remove(locale);
        cx.notify();
    }
}

impl Render for KeyboardSettings {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_locales = self.selected_locales.clone();

        div()
            .v_flex()
            .gap_2()
            .p_4()
            .border_1()
            .border_color(rgb(0x404040))
            .rounded_lg()
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_lg()
                    .child("Keyboard Settings".to_string()),
            )
            .child(
                div()
                    .h_flex()
                    .gap_4()
                    .items_center()
                    .child(div().min_w(px(120.0)).child("Locale:"))
                    .child(Dropdown::new(&self.locale_dropdown).min_w(px(200.0))),
            )
            .child(
                div()
                    .h_flex()
                    .gap_4()
                    .items_center()
                    .child(div().min_w(px(120.0)).child("Selected:"))
                    .child(
                        div().h_flex().gap_2().flex_wrap().children(
                            self.selected_locales
                                .iter()
                                .enumerate()
                                .map(|(idx, locale)| {
                                    let locale_clone = locale.clone();
                                    div()
                                        .h_flex()
                                        .gap_1()
                                        .px_2()
                                        .py_1()
                                        .border_1()
                                        .border_color(rgb(0x606060))
                                        .rounded_md()
                                        .bg(rgb(0x2a2a2a))
                                        .items_center()
                                        .child(div().text_sm().child(locale.clone()))
                                        .child(Button::new(("remove", idx)).label("×").on_click(
                                            cx.listener(move |this, _, _, cx| {
                                                this.remove_locale(&locale_clone, cx);
                                            }),
                                        ))
                                }),
                        ),
                    ),
            )
            .child(
                div()
                    .h_flex()
                    .gap_4()
                    .items_center()
                    .child(div().min_w(px(120.0)))
                    .child(
                        Button::new("apply-keyboard-settings")
                            .label("Apply keyboard config")
                            .on_click(move |_, _, _cx| {
                                // TODO: remove the clone here this is just dirty hack to get it
                                // working
                                let override_str = conf::locale_override(selected_locales.clone());

                                // DEBUG
                                println!("Generated locale override string: {}", override_str);

                                write_override_line(&override_str).unwrap_or_else(|e| {
                                    println!("Failed to write override line: {}", e);
                                });
                            }),
                    ),
            )
    }
}
