use std::process::Command;

pub fn get_current_sensitivity() -> anyhow::Result<f32> {
    let output = Command::new("hyprctl")
        .args(["getoption", "input:sensitivity"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    parse_sens(&stdout)
}

pub fn get_accel_setting() -> anyhow::Result<bool> {
    let output = Command::new("hyprctl")
        .args(["getoption", "input:force_no_accel"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parts = stdout.trim().split_whitespace().collect::<Vec<&str>>();
    if parts.len() < 2 {
        return Err(anyhow::anyhow!("Invalid output from hyprctl getoption"));
    }
    let trimmed = parts[1]
        .parse::<i32>()
        .map_err(|e| anyhow::anyhow!("Failed to parse force_no_accel: {}", e))?;

    match trimmed {
        1 => Ok(true),
        0 => Ok(false),
        _ => Err(anyhow::anyhow!(
            "Unexpected output for force_no_accel: {}",
            trimmed
        )),
    }
}

fn parse_sens(output: &str) -> anyhow::Result<f32> {
    let parts: Vec<&str> = output.trim().split_whitespace().collect();

    // assume for now that the first 2 parts is float and the value. the rest are not important
    if parts.len() < 2 {
        return Err(anyhow::anyhow!("Invalid output from hyprctl getoption"));
    }

    parts[1]
        .parse::<f32>()
        .map_err(|e| anyhow::anyhow!("Failed to parse sensitivity: {}", e))
}
