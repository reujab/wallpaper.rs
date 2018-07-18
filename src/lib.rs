use std::error::Error;

// i really wish you could group multiple lines using a single #[cfg]

// common
#[cfg(any(unix, windows))]
extern crate dirs;
#[cfg(any(unix, windows))]
extern crate reqwest;
#[cfg(any(unix, windows))]
extern crate url;

#[cfg(any(unix, windows))]
use std::fs::File;
#[cfg(any(unix, windows))]
use url::Url;

// unix
#[cfg(all(unix, not(target_os = "macos")))]
extern crate enquote;
#[cfg(all(unix, not(target_os = "macos")))]
extern crate ini;

#[cfg(all(unix, not(target_os = "macos")))]
mod linux;

#[cfg(all(unix, not(target_os = "macos")))]
pub use linux::*;

// macos
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::*;

// windows
#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::*;

// unsupported
#[cfg(not(any(unix, windows)))]
mod unsupported;

#[cfg(not(any(unix, windows)))]
pub use unsupported::*;

type Result<T> = std::result::Result<T, Box<Error>>;

#[cfg(any(unix, windows))]
pub fn download_image(url: &Url) -> Result<String> {
    let cache_dir = dirs::cache_dir().ok_or("no cache dir")?;
    let segments = url.path_segments().ok_or("no path segments")?;
    let mut file_name = segments.last().ok_or("no file name")?;
    if file_name.is_empty() {
        file_name = "wallpaper";
    }
    let file_path = cache_dir.join(file_name);

    let mut file = File::create(&file_path)?;
    reqwest::get(url.as_str())?.copy_to(&mut file)?;

    Ok(file_path.to_str().to_owned().unwrap().into())
}

#[cfg(unix)]
fn get_stdout(command: &str, args: &[&str]) -> Result<String> {
    use std::process::Command;

    let output = Command::new(command).args(args).output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().into())
    } else {
        Err(format!(
            "{} exited with status code {}",
            command,
            output.status.code().unwrap_or(-1)
        ).into())
    }
}

#[cfg(unix)]
#[inline]
fn run(command: &str, args: &[&str]) -> Result<()> {
    get_stdout(command, args)?;
    Ok(())
}
