mod gnome;
mod kde;
mod lxde;
pub(crate) mod xfce;

use crate::{get_stdout, run, Error, Mode, Result};
use std::{env, path::Path, process::Command};

#[cfg(feature = "from_url")]
use crate::download_image;

/// Returns the wallpaper of the current desktop.
pub fn get() -> Result<String> {
    let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    if gnome::is_compliant(&desktop) {
        return gnome::get();
    }

    match desktop.as_str() {
        "KDE" => kde::get(),
        "X-Cinnamon" => parse_dconf(
            "dconf",
            &["read", "/org/cinnamon/desktop/background/picture-uri"],
        ),
        "MATE" => parse_dconf(
            "dconf",
            &["read", "/org/mate/desktop/background/picture-filename"],
        ),
        "XFCE" => xfce::get(),
        "LXDE" => lxde::get(),
        "Deepin" => parse_dconf(
            "dconf",
            &[
                "read",
                "/com/deepin/wrap/gnome/desktop/background/picture-uri",
            ],
        ),
        _ => Err(Error::UnsupportedDesktop),
    }
}

/// Sets the wallpaper for the current desktop from a file path.
pub fn set_from_path<P>(path: P) -> Result<()>
where
    P: AsRef<Path> + std::fmt::Display,
{
    let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    if gnome::is_compliant(&desktop) {
        return gnome::set(&path);
    }

    match desktop.as_str() {
        "KDE" => kde::set(&path),
        "X-Cinnamon" => run(
            "dconf",
            &[
                "write",
                "/org/cinnamon/desktop/background/picture-uri",
                &enquote::enquote('"', &format!("file://{}", &path)),
            ],
        ),
        "MATE" => run(
            "dconf",
            &[
                "write",
                "/org/mate/desktop/background/picture-filename",
                &enquote::enquote('"', path.as_ref().to_str().ok_or(Error::InvalidPath)?),
            ],
        ),
        "XFCE" => xfce::set(path),
        "LXDE" => lxde::set(path),
        "Deepin" => run(
            "dconf",
            &[
                "write",
                "/com/deepin/wrap/gnome/desktop/background/picture-uri",
                &enquote::enquote('"', &format!("file://{}", &path)),
            ],
        ),
        _ => wlrs::set_from_path(path.as_ref().to_str().ok_or(Error::InvalidPath)?)
            .map_err(|_| Error::UnsupportedDesktop),
    }
}

#[cfg(feature = "from_url")]
/// Sets the wallpaper for the current desktop from a URL.
pub fn set_from_url(url: &str) -> Result<()> {
    let path = download_image(url)?;
    set_from_path(&path)
}

/// Sets the wallpaper style.
pub fn set_mode(mode: Mode) -> Result<()> {
    let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    if gnome::is_compliant(&desktop) {
        return gnome::set_mode(mode);
    }

    match desktop.as_str() {
        "KDE" => kde::set_mode(mode),
        "X-Cinnamon" => run(
            "dconf",
            &[
                "write",
                "/org/cinnamon/desktop/background/picture-options",
                &mode.get_gnome_string(),
            ],
        ),
        "MATE" => run(
            "dconf",
            &[
                "write",
                "/org/mate/desktop/background/picture-options",
                &mode.get_gnome_string(),
            ],
        ),
        "XFCE" => xfce::set_mode(mode),
        "LXDE" => lxde::set_mode(mode),
        "Deepin" => run(
            "dconf",
            &[
                "write",
                "/com/deepin/wrap/gnome/desktop/background/picture-options",
                &mode.get_gnome_string(),
            ],
        ),
        _ => Err(Error::UnsupportedDesktop),
    }
}

fn parse_dconf(command: &str, args: &[&str]) -> Result<String> {
    let mut stdout = enquote::unquote(&get_stdout(command, args)?)?;
    // removes file protocol
    if stdout.starts_with("file://") {
        stdout = stdout[7..].into();
    }
    Ok(stdout)
}
