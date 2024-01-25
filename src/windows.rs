use crate::{Mode, Result};
use std::ffi::OsStr;
use std::io;
use std::iter;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::c_void;
use winapi::um::winuser::SystemParametersInfoW;
use winapi::um::winuser::SPIF_SENDCHANGE;
use winapi::um::winuser::SPIF_UPDATEINIFILE;
use winapi::um::winuser::SPI_GETDESKWALLPAPER;
use winapi::um::winuser::SPI_SETDESKWALLPAPER;
use winreg::enums::*;
use winreg::RegKey;

#[cfg(feature = "from_url")]
use crate::download_image;

/// Returns the current wallpaper.
pub fn get() -> Result<String> {
    unsafe {
        let buffer: [u16; 260] = mem::zeroed();
        let successful = SystemParametersInfoW(
            SPI_GETDESKWALLPAPER,
            buffer.len() as u32,
            buffer.as_ptr() as *mut c_void,
            0,
        ) == 1;

        if successful {
            let path = String::from_utf16(&buffer)?
                // removes trailing zeroes from buffer
                .trim_end_matches('\x00')
                .into();
            Ok(path)
        } else {
            Err(io::Error::last_os_error().into())
        }
    }
}

/// Sets the wallpaper from a file.
pub fn set_from_path(path: &str) -> Result<()> {
    unsafe {
        let path = std::fs::canonicalize(path)?;

        let path = path
            .as_os_str()
            .encode_wide()
            // append null byte
            .chain(iter::once(0))
            .collect::<Vec<u16>>();

        let successful = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            path.as_ptr() as *mut c_void,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        ) == 1;

        if successful {
            Ok(())
        } else {
            Err(io::Error::last_os_error().into())
        }
    }
}

/// Sets the wallpaper from a URL.
#[cfg(feature = "from_url")]
pub fn set_from_url(url: &str) -> Result<()> {
    let path = download_image(url)?;
    set_from_path(&path)
}

/// Sets the wallpaper style.
pub fn set_mode(mode: Mode) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (desktop, _) = hkcu.create_subkey(r"Control Panel\Desktop")?;

    desktop.set_value(
        "TileWallpaper",
        &match mode {
            Mode::Tile => "1",
            _ => "0",
        }
        .to_string(),
    )?;

    // copied from https://searchfox.org/mozilla-central/rev/5e955a47c4af398e2a859b34056017764e7a2252/browser/components/shell/nsWindowsShellService.cpp#493
    desktop.set_value(
        "WallpaperStyle",
        &match mode {
            // does not work with integers
            Mode::Center | Mode::Tile => "0",
            Mode::Fit => "6",
            Mode::Span => "22",
            Mode::Stretch => "2",
            Mode::Crop => "10",
        }
        .to_string(),
    )?;

    // updates wallpaper
    set_from_path(&get()?)
}
