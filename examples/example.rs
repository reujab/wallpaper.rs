fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let path = args.next().unwrap();

    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper::get());
    // Sets the wallpaper for the current desktop from a file path.
    wallpaper::set_from_path(&path).unwrap();
    // Sets the wallpaper style.
    wallpaper::set_mode(wallpaper::Mode::Center).unwrap();
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper::get());
}
