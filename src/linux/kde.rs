use crate::{run, Mode, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Returns the wallpaper of KDE.
pub fn get() -> Result<String> {
    let path = dirs::config_dir()
        .ok_or("could not find config directory")?
        .join("plasma-org.kde.plasma.desktop-appletsrc");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if let Some(end) = line.strip_prefix("Image=") {
            let mut uri = end.trim();
            if uri.starts_with("file://") {
                uri = &uri[7..];
            }
            return Ok(uri.into());
        }
    }

    Err("no kde image found".into())
}

/// Sets the wallpaper for KDE.
pub fn set(path: &str) -> Result<()> {
    eval(&format!(
        r#"
for (const desktop of desktops()) {{
    desktop.currentConfigGroup = ["Wallpaper", "org.kde.image", "General"]
    desktop.writeConfig("Image", {})
}}"#,
        enquote::enquote('"', &format!("file://{}", path)),
    ))
}

pub fn set_mode(mode: Mode) -> Result<()> {
    eval(&format!(
        r#"
for (const desktop of desktops()) {{
    desktop.currentConfigGroup = ["Wallpaper", "org.kde.image", "General"]
    desktop.writeConfig("FillMode", {})
}}"#,
        match mode {
            Mode::Center => 6,
            Mode::Crop => 2,
            Mode::Fit => 1,
            Mode::Span => 2,
            Mode::Stretch => 0,
            Mode::Tile => 3,
        }
    ))
}

fn eval(script: &str) -> Result<()> {
    run(
        "qdbus",
        &[
            "org.kde.plasmashell",
            "/PlasmaShell",
            "org.kde.PlasmaShell.evaluateScript",
            script,
        ],
    )
}
