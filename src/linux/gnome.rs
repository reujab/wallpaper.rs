use super::parse_dconf;
use crate::{run, Mode, Result};

#[inline]
pub fn is_compliant(desktop: &str) -> bool {
    desktop.contains("GNOME") || desktop == "Unity" || desktop == "Pantheon"
}

pub fn get() -> Result<String> {
    parse_dconf(
        "gsettings",
        &["get", "org.gnome.desktop.background", "picture-uri"],
    )
}

pub fn set(path: &str) -> Result<()> {
    let uri = enquote::enquote('"', &format!("file://{}", path));
    let res = run(
        "gsettings",
        &["set", "org.gnome.desktop.background", "picture-uri", &uri],
    );
    run(
        "gsettings",
        &[
            "set",
            "org.gnome.desktop.background",
            "picture-uri-dark",
            &uri,
        ],
    )
    // Ignore the result because in Gnome < 42 the cmd could fail since
    // key "picture-uri-dark" does not exists
    .or_else(|_| res)
}

pub fn set_mode(mode: Mode) -> Result<()> {
    run(
        "gsettings",
        &[
            "set",
            "org.gnome.desktop.background",
            "picture-options",
            &mode.get_gnome_string(),
        ],
    )
}

impl Mode {
    pub(crate) fn get_gnome_string(self) -> String {
        enquote::enquote(
            '"',
            match self {
                Mode::Center => "centered",
                Mode::Crop => "zoom",
                Mode::Fit => "scaled",
                Mode::Span => "spanned",
                Mode::Stretch => "stretched",
                Mode::Tile => "wallpaper",
            },
        )
    }
}
