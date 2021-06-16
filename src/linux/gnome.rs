use super::parse_dconf;
use crate::{run, Result};

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
    run(
        "gsettings",
        &["set", "org.gnome.desktop.background", "picture-uri", &uri],
    )
}
