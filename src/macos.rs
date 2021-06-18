use enquote;
use get_stdout;
use run;
use Result;

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
pub fn set_from_path(path: &str) -> Result<()> {
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

/// No-op. Unable to change with AppleScript.
pub fn set_mode(_: Mode) -> Result<()> {
    Ok(())
}
