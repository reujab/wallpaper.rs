# wallpaper [![crate](https://img.shields.io/crates/v/wallpaper.svg)](https://crates.io/crates/wallpaper) [![docs](https://docs.rs/wallpaper/badge.svg)](https://docs.rs/wallpaper)
This Rust library gets and sets the desktop wallpaper/background.

The supported desktops are:
* Windows
* macOS
* GNOME
* KDE
* Cinnamon
* Unity
* Budgie
* XFCE
* LXDE
* MATE
* Deepin
* i3 (set only, requires feh)

## Example
```rust
extern crate wallpaper;

fn main() {
    println!("{:?}", wallpaper::get().unwrap());
    wallpaper::set_from_path("/usr/share/backgrounds/gnome/adwaita-day.png").unwrap();
}
```
