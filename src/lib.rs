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
//! * i3 (set only)
//!
//! # Examples
//! ```
//! extern crate wallpaper;
//!
//! fn main() {
//!     println!("{:?}", wallpaper::get());
//!     wallpaper::set("/usr/share/backgrounds/gnome/Tree.jpg").unwrap();
//!     println!("{:?}", wallpaper::get());
//! }
//! ```

use std::error::Error;

// i really wish you could group multiple lines using a single #[cfg]

// common
#[cfg(any(unix, windows))]
extern crate dirs;

// unix
#[cfg(unix)]
extern crate enquote;

// linux and *bsd
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

type Result<T> = std::result::Result<T, Box<dyn Error>>;

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
    get_stdout(command, args)?;
    Ok(())
}
