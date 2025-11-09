use std::{collections::HashSet, path::Path};

use crate::{
    config_writer::{
        ConfigLine, ConfigObjectKey, ConfigWriter, DeviceSetting, HYPR_CONFIG_PATH,
        HYPR_OVERRIDES_PATH,
    },
    monitor::MonitorMode,
};
use dirs::home_dir;
use std::io::Write;

const MONITOR_CONFIG_PREFIX: &str = "monitor=";
const KEYBOARD_LAYOUT_PREFIX: &str = "input:kb_layout=";
const MOUSE_SENSITIVITY_PREFIX: &str = "input:sensitivity=";
const MOUSE_FORCE_NO_ACCEL_PREFIX: &str = "input:force_no_accel=";

struct MonitorConfig;
struct KeyboardLayoutConfig;
// I dont like this but i am too stupid and tired to think
struct MouseSensitivityConfig;
struct MouseForceNoAccelConfig;

impl ConfigLine for MonitorConfig {
    fn prefix(&self) -> &str {
        MONITOR_CONFIG_PREFIX
    }

    fn extract_key(&self, line: &str) -> Option<String> {
        let trimmed = line.trim();
        if let Some(config) = trimmed.strip_prefix(self.prefix())
            && let Some(comma_pos) = config.find(',')
        {
            return Some(config[..comma_pos].to_string());
        }
        None
    }
}

impl ConfigLine for KeyboardLayoutConfig {
    fn prefix(&self) -> &str {
        KEYBOARD_LAYOUT_PREFIX
    }

    fn extract_key(&self, line: &str) -> Option<String> {
        if line.trim().starts_with(self.prefix()) {
            // For keyboard layout, we use a constant key since there's only one
            Some("kb_layout".to_string())
        } else {
            None
        }
    }
}

impl ConfigLine for MouseSensitivityConfig {
    fn prefix(&self) -> &str {
        MOUSE_SENSITIVITY_PREFIX
    }

    fn extract_key(&self, line: &str) -> Option<String> {
        if line.trim().starts_with(self.prefix()) {
            // For mouse sensitivity, we use a constant key since there's only one
            Some("sensitivity".to_string())
        } else {
            None
        }
    }
}

impl ConfigLine for MouseForceNoAccelConfig {
    fn prefix(&self) -> &str {
        MOUSE_FORCE_NO_ACCEL_PREFIX
    }

    fn extract_key(&self, line: &str) -> Option<String> {
        if line.trim().starts_with(self.prefix()) {
            // For mouse force_no_accel, we use a constant key since there's only one
            Some("force_no_accel".to_string())
        } else {
            None
        }
    }
}

/// Registry of all known config line types
pub fn get_config_handlers() -> Vec<Box<dyn ConfigLine>> {
    vec![
        Box::new(MonitorConfig),
        Box::new(KeyboardLayoutConfig),
        Box::new(MouseSensitivityConfig),
        Box::new(MouseForceNoAccelConfig),
    ]
}

/// Create the overrides configuration file for hyprland.
/// This file is created at `~./config/hypr/conf-overrides.conf`
/// If the file already exists, it will not be overwritten.
/// This function will only run once if the file does not exist.
/// It will also edit the main file `~/.config/hypr/hyprland.conf` to include the overrides file as
/// a source file at the bottom of the main configuration file to ensure that all exisiting
/// settings are overwritten but not removed.
pub fn create_overrides() -> anyhow::Result<()> {
    let home_dir = home_dir().ok_or_else(|| {
        anyhow::anyhow!("Could not determine home directory for the current user")
    })?;

    let hypr_config_path = home_dir.join(HYPR_CONFIG_PATH);
    let hypr_overrides_path = home_dir.join(HYPR_OVERRIDES_PATH);

    if !Path::new(&hypr_config_path).exists() {
        return Err(anyhow::anyhow!(
            "Hyprland configuration file not found at {}, Hyprland is either not installed or not configured",
            HYPR_CONFIG_PATH
        ));
    }

    if !Path::new(&hypr_overrides_path).exists() {
        std::fs::write(&hypr_overrides_path, "# Hyprland configuration overrides\n")?;

        // append the file source line to main conf
        let mut hypr_config_file = std::fs::OpenOptions::new()
            .append(true)
            .open(&hypr_config_path)?;

        writeln!(
            hypr_config_file,
            "\n# Include overrides configuration\nsource = ~/{}",
            HYPR_OVERRIDES_PATH
        )?;
    }

    Ok(())
}

/// Write a line to the overrides configuartion file.
/// Uses dynamic config handlers to determine if the line should replace an existing one.
/// For monitor configs, it will replace lines with the same monitor name.
/// For keyboard layout, it will replace the existing layout line.
pub fn write_override_line(line: &str) -> anyhow::Result<()> {
    let home_dir = home_dir().ok_or_else(|| {
        anyhow::anyhow!("Could not determine home directory for the current user")
    })?;

    let hypr_overrides_path = home_dir.join(HYPR_OVERRIDES_PATH);

    // Read existing content
    let content = std::fs::read_to_string(&hypr_overrides_path)?;
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    let handlers = get_config_handlers();
    let mut replaced = false;

    // Try to find a handler that matches this line
    for handler in &handlers {
        if let Some(new_key) = handler.extract_key(line) {
            if handler.should_replace() {
                // Find and replace existing line with the same key
                for existing_line in lines.iter_mut() {
                    if let Some(existing_key) = handler.extract_key(existing_line)
                        && existing_key == new_key
                    {
                        *existing_line = line.to_string();
                        replaced = true;
                        break;
                    }
                }
            }
            break;
        }
    }

    // If not replaced, append the new line
    if !replaced {
        lines.push(line.to_string());
    }

    // Write back to file
    let updated_content = lines.join("\n") + "\n";
    std::fs::write(&hypr_overrides_path, updated_content)?;

    Ok(())
}

/// Generate a monitor override string for hyprland configuration.
/// Format: monitor=name,resolution@refreshrate,position,scale
/// Example: monitor=DP-3,2560x1440@155,0x0,1
pub fn monitor_override(
    monitor_name: String,
    settings: MonitorMode,
    position: (i32, i32),
) -> String {
    let position_string = format!("{}x{}", position.0, position.1);
    let config_string = format!(
        "{}@{},{},1",
        settings.resolution, settings.refresh_rate, position_string
    );

    format!(
        "{}{},{}",
        MONITOR_CONFIG_PREFIX, monitor_name, config_string
    )
}

// pub fn set_keyboard_device_layout(device: String, locale: String) -> anyhow::Result<()> {
//     let input_device = DeviceSetting {
//         key: ConfigObjectKey::Device,
//         device_name: device,
//         kb_layout: locale,
//     };
//
//     ConfigWriter::build(input_device)?.write()?;
//
//     Ok(())
// }

/// Generate a mouse sensitivity override string for hyprland configuration.
pub fn mouse_sensitivity_override(sensitivity: f32) -> String {
    format!("{}{}", MOUSE_SENSITIVITY_PREFIX, sensitivity)
}

/// Generate a mouse force_no_accel override string for hyprland configuration.
pub fn mouse_force_no_accel_override(force_no_accel: bool) -> String {
    format!(
        "{}{}",
        MOUSE_FORCE_NO_ACCEL_PREFIX,
        if force_no_accel { 1 } else { 0 }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_config_extract_key() {
        let monitor_config = MonitorConfig;
        assert_eq!(
            monitor_config.extract_key("monitor=DP-3,2560x1440@155,0x0,1"),
            Some("DP-3".to_string())
        );
        assert_eq!(
            monitor_config.extract_key("  monitor=HDMI-A-1,1920x1080@60,1920x0,1  "),
            Some("HDMI-A-1".to_string())
        );
        assert_eq!(monitor_config.extract_key("# comment line"), None);
        assert_eq!(monitor_config.extract_key("some other config"), None);
    }

    #[test]
    fn test_monitor_override_with_position() {
        let result = monitor_override(
            "DP-3".to_string(),
            MonitorMode {
                resolution: "2560x1440".to_string(),
                refresh_rate: 155.0,
            },
            (0, 0),
        );
        assert_eq!(result, "monitor=DP-3,2560x1440@155,0x0,1");

        let result2 = monitor_override(
            "HDMI-A-1".to_string(),
            MonitorMode {
                resolution: "1920x1080".to_string(),
                refresh_rate: 60.0,
            },
            (1920, 0),
        );
        assert_eq!(result2, "monitor=HDMI-A-1,1920x1080@60,1920x0,1");

        // Test negative position (monitor above primary)
        let result3 = monitor_override(
            "DP-2".to_string(),
            MonitorMode {
                resolution: "1920x1080".to_string(),
                refresh_rate: 144.0,
            },
            (0, -1080),
        );
        assert_eq!(result3, "monitor=DP-2,1920x1080@144,0x-1080,1");
    }

    #[test]
    fn test_keyboard_config_extract_key() {
        let kb_config = KeyboardLayoutConfig;
        assert_eq!(
            kb_config.extract_key("input:kb_layout=us,fi"),
            Some("kb_layout".to_string())
        );
        assert_eq!(kb_config.extract_key("monitor=DP-3,auto"), None);
    }
}
