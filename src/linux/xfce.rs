use crate::{get_stdout, run, Error, Mode, Result};
use std::path::Path;

fn get_desktop_props(key: &str) -> Result<Vec<String>> {
    let stdout = get_stdout("xfconf-query", &["--channel", "xfce4-desktop", "--list"])?;
    let desktops = stdout
        .split('\n')
        .filter(|line| line.ends_with(key))
        .map(|desktop| desktop.to_string())
        .collect::<Vec<String>>();

    if desktops.is_empty() {
        return Err(Error::XfceNoDesktops);
    }

    Ok(desktops)
}

pub fn get() -> Result<String> {
    let desktops = get_desktop_props("last-image")?;
    let path = get_stdout(
        "xfconf-query",
        &["--channel", "xfce4-desktop", "--property", &desktops[0]],
    )?;

    Ok(path.trim().to_string())
}

pub fn set<P>(path: P) -> Result<()>
where
    P: AsRef<Path> + std::fmt::Display,
{
    for desktop in get_desktop_props("last-image")? {
        run(
            "xfconf-query",
            &[
                "--channel",
                "xfce4-desktop",
                "--property",
                &desktop,
                "--set",
                match path.as_ref().to_str() {
                    Some(it) => it,
                    None => return Err(Error::XfceNoDesktops),
                },
            ],
        )?;
    }

    Ok(())
}

pub fn set_mode(mode: Mode) -> Result<()> {
    for property in get_desktop_props("image-style")? {
        run(
            "xfconf-query",
            &[
                "--channel",
                "xfce4-desktop",
                "--property",
                &property,
                "--set",
                match mode {
                    Mode::Center => "1",
                    Mode::Crop => "5",
                    Mode::Fit => "4",
                    Mode::Span => "5",
                    Mode::Stretch => "3",
                    Mode::Tile => "2",
                },
            ],
        )?;
    }

    Ok(())
}
