use dirs;
use enquote;
use run;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use Result;

/// Returns the wallpaper of KDE.
pub fn get() -> Result<String> {
    let path = dirs::config_dir()
        .ok_or("could not find config directory")?
        .join("plasma-org.kde.plasma.desktop-appletsrc");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("Image=") {
            let mut uri = line[6..].trim();
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
    run(
        "qdbus",
        &[
            "org.kde.plasmashell",
            "/PlasmaShell",
            "org.kde.PlasmaShell.evaluateScript",
            &format!(
                r#"
const monitors = desktops()
for (var i = 0; i < monitors.length; i++) {{
    monitors[i].currentConfigGroup = ["Wallpaper"]
    monitors[i].writeConfig("Image", {})
}}"#,
                enquote::enquote('"', &format!("file://{}", path)),
            ),
        ],
    )
}
