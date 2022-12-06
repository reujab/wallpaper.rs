use crate::{get_stdout, run, Mode, Result};

#[cfg(feature = "from_url")]
use crate::download_image;

/// Returns the current wallpaper.
pub fn get() -> Result<String> {
    get_stdout(
        "osascript",
        &[
            "-e",
            r#"tell application "Finder" to get POSIX path of (get desktop picture as alias)"#,
        ],
    )
}

// Sets the wallpaper from a file.
pub fn set_from_path<P>(path: P) -> Result<()>
where
    P: AsRef<Path> + std::fmt::Display,
{
    run(
        "osascript",
        &[
            "-e",
            &format!(
                r#"tell application "System Events" to tell every desktop to set picture to {}"#,
                enquote::enquote('"', path),
            ),
        ],
    )
}

#[cfg(feature = "from_url")]
// Sets the wallpaper from a URL.
pub fn set_from_url(url: &str) -> Result<()> {
    let path = download_image(url)?;
    set_from_path(&path)
}

/// No-op. Unable to change with AppleScript.
pub fn set_mode(_: Mode) -> Result<()> {
    Err("unsupported on macos".into())
}
