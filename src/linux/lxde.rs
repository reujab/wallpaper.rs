use crate::{run, Error, Mode, Result};
use ini::Ini;
use std::env;

pub fn get() -> Result<String> {
    // DESKTOP_SESSION in used on Raspbian
    let session = env::var("DESKTOP_SESSION").unwrap_or_else(|_| "LXDE".into());
    let path = dirs::config_dir()
        .ok_or(Error::NoConfigDir)?
        .join(format!("pcmanfm/{}/desktop-items-0.conf", session));
    let ini = Ini::load_from_file(path)?;
    Ok(ini
        .section(Some("*"))
        .and_then(|ini| ini.get("wallpaper"))
        .ok_or(Error::NoImage("LXDE"))?
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
