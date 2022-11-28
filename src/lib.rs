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
//!println!("{:?}", wallpaper::get());
//!wallpaper::set_from_path("/usr/share/backgrounds/gnome/adwaita-day.png").unwrap();
//!wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
//!println!("{:?}", wallpaper::get());
//! ```

mod error;
pub use error::Error;

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

// from_url feature
#[cfg(feature = "from_url")]
mod from_url;

#[cfg(feature = "from_url")]
pub(crate) use from_url::download_image;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug)]
pub enum Mode {
    Center,
    Crop,
    Fit,
    Span,
    Stretch,
    Tile,
}

#[cfg(unix)]
fn get_stdout(command: &str, args: &[&str]) -> Result<String> {
    use std::process::Command;

    let output = Command::new(command).args(args).output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().into())
    } else {
        Err(Error::CommandFailed {
            command: command.to_string(),
            code: output.status.code().unwrap_or(-1),
        })
    }
}

#[cfg(unix)]
#[inline]
fn run(command: &str, args: &[&str]) -> Result<()> {
    get_stdout(command, args).map(|_| ())
}
