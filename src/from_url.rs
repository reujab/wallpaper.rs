use crate::{Error, Result};
use std::fs::File;

pub fn download_image(url: &str) -> Result<String> {
    let cache_dir = dirs::cache_dir().ok_or(Error::NoConfigDir)?;
    let file_path = cache_dir.join("wallpaper.jpg");

    let mut file = File::create(&file_path)?;
    reqwest::blocking::get(url)?.copy_to(&mut file)?;

    Ok(file_path.to_str().to_owned().ok_or(Error::InvalidPath)?.into())
}
