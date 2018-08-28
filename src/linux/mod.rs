mod kde;
mod lxde;

use download_image;
use enquote;
use get_stdout;
use run;
use std::env;
use Result;

/// Returns the wallpaper of the current desktop.
pub fn get() -> Result<String> {
    let desktop = env::var("XDG_CURRENT_DESKTOP")?;

    if is_gnome_compliant(&desktop) {
        return parse_dconf(
            "gsettings",
            &["get", "org.gnome.desktop.background", "picture-uri"],
        );
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
        "XFCE" => get_stdout(
            "xfconf-query",
            &[
                "-c",
                "xfce4-desktop",
                "-p",
                "/backdrop/screen0/monitor0/workspace0/last-image",
            ],
        ),
        "LXDE" => lxde::get(),
        "Deepin" => parse_dconf(
            "dconf",
            &[
                "read",
                "/com/deepin/wrap/gnome/desktop/background/picture-uri",
            ],
        ),
        _ => Err("unsupported desktop".into()),
    }
}

/// Sets the wallpaper for the current desktop from a file path.
pub fn set_from_path(path: &str) -> Result<()> {
    let desktop = env::var("XDG_CURRENT_DESKTOP")?;

    if is_gnome_compliant(&desktop) {
        let uri = enquote::enquote('"', &format!("file://{}", path));
        return run(
            "gsettings",
            &["set", "org.gnome.desktop.background", "picture-uri", &uri],
        );
    }

    match desktop.as_str() {
        "KDE" => kde::set(path),
        "X-Cinnamon" => run(
            "dconf",
            &[
                "write",
                "/org/cinnamon/desktop/background/picture-uri",
                &enquote::enquote('"', &format!("file://{}", path)),
            ],
        ),
        "MATE" => run(
            "dconf",
            &[
                "write",
                "/org/mate/desktop/background/picture-filename",
                &enquote::enquote('"', &path),
            ],
        ),
        "XFCE" => run(
            "xfconf-query",
            &[
                "-c",
                "xfce4-desktop",
                "-p",
                "/backdrop/screen0/monitor0/workspace0/last-image",
                "-s",
                &path,
            ],
        ),
        "LXDE" => run("pcmanfm", &["-w", &path]),
        "Deepin" => run(
            "dconf",
            &[
                "write",
                "/com/deepin/wrap/gnome/desktop/background/picture-uri",
                &enquote::enquote('"', &format!("file://{}", path)),
            ],
        ),
        "i3" => run("feh", &["--bg-fill", &path]),
        _ => Err("unsupported desktop".into()),
    }
}

/// Sets the wallpaper for the current desktop from a URL.
pub fn set_from_url(url: &str) -> Result<()> {
    let desktop = env::var("XDG_CURRENT_DESKTOP")?;

    match desktop.as_str() {
        // only some GNOME-based desktops support urls for picture-uri
        "GNOME" | "ubuntu:GNOME" => run(
            "gsettings",
            &[
                "set",
                "org.gnome.desktop.background",
                "picture-uri",
                &enquote::enquote('"', url),
            ],
        ),
        "i3" => run("feh", &["--bg-fill", url]),
        _ => {
            let path = download_image(&url.parse()?)?;
            set_from_path(&path)
        }
    }
}

#[inline]
fn is_gnome_compliant(desktop: &str) -> bool {
    desktop.contains("GNOME") || desktop == "Unity" || desktop == "Pantheon"
}

fn parse_dconf(command: &str, args: &[&str]) -> Result<String> {
    let mut stdout = enquote::unquote(&get_stdout(command, args)?)?;
    // removes file protocol
    if stdout.starts_with("file://") {
        stdout = stdout[7..].into();
    }
    Ok(stdout)
}
