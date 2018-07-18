use download_image;
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
pub fn set_from_file(path: &str) -> Result<()> {
    run(
        "osascript",
        &[
            "-e",
            &format!(
                r#"tell application "Finder" to set desktop picture to POSIX file {}"#,
                path,
            ),
        ],
    )
}

// Sets the wallpaper from a URL.
pub fn set_from_url(url: &str) -> Result<()> {
    let path = download_image(&url.parse()?)?;
    set_from_file(&path)
}
