// When the compile mode is NOT debug (i.e., release mode), the target
// will be built as a GUI application on Windows, NOT a console application.
// So, we can run it as a scheduled task without a console window pop-up.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Set my wallpaper to a random picture from a directory
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of picture directory
    #[arg(short, long, value_name = "DIR")]
    picture_dir: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let args: Args = clap::Parser::parse();

    let paths: Vec<_> = std::fs::read_dir(&args.picture_dir)?
        .map(|r| r.map(|d| d.path()).map_err(|e| e.to_string()))
        .collect::<Result<_, _>>()?;

    use rand::prelude::IndexedRandom;
    let selected_wallpaper = paths
        .choose(&mut rand::rng())
        .ok_or("No wallpaper found")?
        .to_str()
        .ok_or("Invalid path")?;

    wallpaper::set_from_path(selected_wallpaper)?;
    Ok(())
}
