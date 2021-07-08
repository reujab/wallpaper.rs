//! This library gets and sets the desktop wallpaper/background.
//!
//! The supported desktops are:
//! * Windows
//! * macOS
//! * GNOME
//! * KDE
//! * Cinnamon
//! * Unity
//! * Budgie
//! * XFCE
//! * LXDE
//! * MATE
//! * Deepin
//! * Most Wayland compositors (set only, requires swaybg)
//! * i3 (set only, requires feh)
//!
//! # Example
//! ```
//! use wallpaper;
//!
//!fn main() {
//!    println!("{:?}", wallpaper::get());
//!    wallpaper::set_from_path("/usr/share/backgrounds/gnome/adwaita-day.png").unwrap();
//!    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
//!    println!("{:?}", wallpaper::get());
//!}
//! ```

use std::error::Error;

#[cfg(feature = "from_url")]
use std::fs::File;

#[cfg(all(unix, not(target_os = "macos")))]
mod linux;

#[cfg(all(unix, not(target_os = "macos")))]
pub use crate::linux::*;

// macos
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::*;

// unsupported
#[cfg(not(any(unix, windows)))]
mod unsupported;

#[cfg(not(any(unix, windows)))]
pub use unsupported::*;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Clone, Debug)]
pub enum Mode {
    Center,
    Crop,
    Fit,
    Span,
    Stretch,
    Tile,
}

#[cfg(feature = "from_url")]
fn download_image(url: &str) -> Result<String> {
    let cache_dir = dirs::cache_dir().ok_or("no cache dir")?;
    let file_path = cache_dir.join("wallpaper");

    let mut file = File::create(&file_path)?;
    reqwest::blocking::get(url)?.copy_to(&mut file)?;

    Ok(file_path.to_str().to_owned().ok_or("no file path")?.into())
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
            output.status.code().unwrap_or(-1),
        )
        .into())
    }
}

#[cfg(unix)]
#[inline]
fn run(command: &str, args: &[&str]) -> Result<()> {
    get_stdout(command, args).map(|_| ())
}
