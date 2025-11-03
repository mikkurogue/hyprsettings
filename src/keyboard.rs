use serde::Deserialize;
use std::collections::HashSet;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct HyprctlDevices {
    keyboards: Vec<Keyboard>,
}

#[derive(Debug, Deserialize)]
struct Keyboard {
    layout: String,
}

/// Get the currently configured keyboard locales from hyprctl
/// just parse the first keyboard's layout field as all keyboards are assumed to have the same
/// layouts
pub fn get_current_locales() -> anyhow::Result<HashSet<String>> {
    let output = Command::new("hyprctl").args(["devices", "-j"]).output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to execute hyprctl devices"));
    }

    let json_str = String::from_utf8(output.stdout)?;
    let devices: HyprctlDevices = serde_json::from_str(&json_str)?;

    let mut locales = HashSet::new();

    if devices.keyboards.is_empty() {
        return Ok(locales);
    }

    devices.keyboards.first().map(|kb| {
        for locale in kb.layout.split(',') {
            let trimmed = locale.trim();
            if !trimmed.is_empty() {
                locales.insert(trimmed.to_string());
            }
        }
    });

    Ok(locales)
}
