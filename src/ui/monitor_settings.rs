use gpui::*;
use gpui_component::dropdown::*;

use gpui_component::{IndexPath, StyledExt};

use crate::monitor::MonitorInfo;

pub struct MonitorSettings {
    monitor_id: u32,
    monitor_name: SharedString,
    resolution_dropdown: Entity<DropdownState<Vec<String>>>,
    refresh_dropdown: Entity<DropdownState<Vec<String>>>,
    position_dropdown: Entity<DropdownState<Vec<String>>>,
}

impl MonitorSettings {
    pub fn new(monitor: MonitorInfo, window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Get unique resolutions
        let mut resolutions: Vec<String> = monitor
            .available_modes
            .iter()
            .map(|m| m.resolution.clone())
            .collect();
        resolutions.sort();
        resolutions.dedup();

        let current_res_idx = resolutions
            .iter()
            .position(|r| r == &monitor.current_resolution);

        let resolution_dropdown = cx.new(|cx| {
            DropdownState::new(
                resolutions,
                current_res_idx.map(|idx| IndexPath::new(idx)),
                window,
                cx,
            )
        });

        // Get refresh rates for current resolution
        let refresh_rates: Vec<String> = monitor
            .available_modes
            .iter()
            .filter(|m| m.resolution == monitor.current_resolution)
            .map(|m| format!("{:.2}Hz", m.refresh_rate))
            .collect();

        let current_refresh_str = format!("{:.2}Hz", monitor.current_refresh_rate);
        let current_refresh_idx = refresh_rates.iter().position(|r| r == &current_refresh_str);

        let refresh_dropdown = cx.new(|cx| {
            DropdownState::new(
                refresh_rates,
                current_refresh_idx.map(|idx| IndexPath::new(idx)),
                window,
                cx,
            )
        });

        // Position options (placeholder)
        let positions = vec![
            "Left".to_string(),
            "Right".to_string(),
            "Top".to_string(),
            "Bottom".to_string(),
            format!("{}x{}", monitor.position.0, monitor.position.1),
        ];

        let position_dropdown = cx.new(|cx| {
            DropdownState::new(
                positions,
                Some(IndexPath::new(4)), // Default to current position
                window,
                cx,
            )
        });

        Self {
            monitor_id: monitor.id,
            monitor_name: monitor.name.into(),
            resolution_dropdown,
            refresh_dropdown,
            position_dropdown,
        }
    }
}

impl Render for MonitorSettings {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
                    .child(format!("{} (ID: {})", self.monitor_name, self.monitor_id)),
            )
            .child(
                div()
                    .h_flex()
                    .gap_4()
                    .items_center()
                    .child(div().min_w(px(120.0)).child("Resolution:"))
                    .child(Dropdown::new(&self.resolution_dropdown).min_w(px(200.0))),
            )
            .child(
                div()
                    .h_flex()
                    .gap_4()
                    .items_center()
                    .child(div().min_w(px(120.0)).child("Refresh Rate:"))
                    .child(Dropdown::new(&self.refresh_dropdown).min_w(px(200.0))),
            )
            .child(
                div()
                    .h_flex()
                    .gap_4()
                    .items_center()
                    .child(div().min_w(px(120.0)).child("Position:"))
                    .child(Dropdown::new(&self.position_dropdown).min_w(px(200.0))),
            )
    }
}
