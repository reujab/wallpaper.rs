use crate::{run, Mode, Result};
use dirs;
use ini::Ini;
use std::env;

pub fn get() -> Result<String> {
    let session = env::var("DESKTOP_SESSION").unwrap_or_else(|_| "LXDE".into());
    let path = dirs::config_dir()
        .ok_or("could not find config directory")?
        .join(format!("pcmanfm/{}/desktop-items-0.conf", session));
    let ini = Ini::load_from_file(path)?;
    Ok(ini
        .section(Some("*"))
        .ok_or("no '*' section found")?
        .get("wallpaper")
        .ok_or("no lxde image found")?
        .clone())
}

pub fn set(path: &str) -> Result<()> {
    run("pcmanfm", &["-w", path])
}

pub fn set_mode(mode: Mode) -> Result<()> {
    run(
        "pcmanfm",
        &[
            "--wallpaper-mode",
            match mode {
                Mode::Center => "center",
                Mode::Crop => "crop",
                Mode::Fit => "fit",
                Mode::Span => "screen",
                Mode::Stretch => "stretch",
                Mode::Tile => "tile",
            },
        ],
    )
}
