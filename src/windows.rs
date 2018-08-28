use download_image;
use std::ffi::OsStr;
use std::io;
use std::iter;
use std::mem;
use std::os::raw::c_void;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winuser::SystemParametersInfoW;
use winapi::um::winuser::SPIF_SENDCHANGE;
use winapi::um::winuser::SPIF_UPDATEINIFILE;
use winapi::um::winuser::SPI_GETDESKWALLPAPER;
use winapi::um::winuser::SPI_SETDESKWALLPAPER;
use Result;

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
                .trim_right_matches('\x00')
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
        let path = OsStr::new(path)
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
pub fn set_from_url(url: &str) -> Result<()> {
    let path = download_image(&url.parse()?)?;
    set_from_path(&path)
}
