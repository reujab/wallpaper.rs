extern crate dirs;
extern crate enquote;
extern crate reqwest;
extern crate url;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

use std::error::Error;
use std::fs::File;
use std::process::Command;
use url::Url;

type Result<T> = std::result::Result<T, Box<Error>>;

fn download_image(url: &Url) -> Result<String> {
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

fn run(command: &str, args: &[&str]) -> Result<()> {
    let output = Command::new(command).args(args).output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "{} exited with status code {}",
            command,
            output.status.code().unwrap_or(-1)
        ).into())
    }
}
