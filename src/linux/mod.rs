use std::env;
use std::process::Command;
use Result;

/// Returns the wallpaper of the current desktop environment.
pub fn get() -> Result<String> {
    let de = env::var("XDG_CURRENT_DESKTOP")?;

    if is_gnome_compliant(&de) {
        return parse_dconf(
            "gsettings",
            &["get", "org.gnome.desktop.background", "picture-uri"],
        );
    }

    match de.as_str() {
        "KDE" => Err("TODO".into()),
        "X-Cinnamon" => parse_dconf(
            "dconf",
            &["read", "/org/cinnamon/desktop/background/picture-uri"],
        ),
        "MATE" => parse_dconf(
            "dconf",
            &["read", "/org/mate/desktop/background/picture-filename"],
        ),
        "XFCE" => Err("TODO".into()),
        "LXDE" => Err("TODO".into()),
        "Deepin" => parse_dconf(
            "dconf",
            &[
                "read",
                "/com/deepin/wrap/gnome/desktop/background/picture-uri",
            ],
        ),
        _ => Err("unsupported desktop environment".into()),
    }
}

#[inline]
fn is_gnome_compliant(de: &str) -> bool {
    de.contains("GNOME") || de == "Unity" || de == "Pantheon"
}

fn parse_dconf(command: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(command).args(args).output()?;
    if !output.status.success() {
        return Err(format!(
            "{} exited with status code {}",
            command,
            output.status.code().unwrap_or(-1),
        ).into());
    }

    let mut stdout = String::from_utf8(output.stdout)?.trim().to_owned();

    // unquotes single quotes
    stdout.remove(0);
    stdout.pop();
    stdout = stdout.replace("\\'", "'");

    // removes file protocol
    if stdout.starts_with("file://") {
        stdout = stdout.split_at(7).1.into();
    }

    Ok(stdout)
}
