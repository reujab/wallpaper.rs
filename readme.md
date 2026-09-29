# wallpaper-ng

This Rust library gets and sets the desktop wallpaper/background.

The supported desktops are:

- Windows
- macOS
- GNOME
- KDE
- Cinnamon
- Unity
- Budgie
- XFCE
- LXDE
- MATE
- Deepin
- Most Wayland compositors (set only, requires swaybg)
- i3 (set only, requires feh)

## Credits

This project is a maintained fork of the [wallpaper](https://github.com/reujab/wallpaper.rs) crate. We would like to acknowledge the work of the original authors, who dedicated their code to the public domain via The Unlicense.

## Examples

```rust
fn main() {
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper_ng::get());
    // Sets the wallpaper for the current desktop from a file path.
    wallpaper_ng::set_from_path("/usr/share/backgrounds/gnome/adwaita-day.png").unwrap();
    // Sets the wallpaper style.
    wallpaper_ng::set_mode(wallpaper_ng::Mode::Crop).unwrap();
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper_ng::get());
}
```

If you want to set an image as background via an URL, make sure you activated the `from_url` feature of the wallpaper-ng crate on Cargo.toml:

```toml
[dependencies]
wallpaper-ng = { version = "0.1", features = ["from_url"] }
```

Then, on your main.rs:

```rust
fn main() {
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper_ng::get());
    // Sets the wallpaper for the current desktop from a URL.
    wallpaper_ng::set_from_url("https://source.unsplash.com/random").unwrap();
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper_ng::get());
}
```

## Contributing

This project follows the [Gitflow](https://nvie.com/posts/a-successful-git-branching-model/) branching model:

| Branch                | Purpose                                                   | Branches off | Merges into          |
|-----------------------|-----------------------------------------------------------|--------------|----------------------|
| `master`              | Production-ready code. Every commit is a tagged release   | —            | —                    |
| `develop`             | Integration branch for the next release                   | `master`     | —                    |
| `feature/<issue>-<short-description>` | New features                              | `develop`    | `develop`            |
| `release/<version>`   | Release preparation (version bump, final fixes)           | `develop`    | `master` and `develop` |
| `hotfix/<issue>-<short-description>`  | Urgent fixes for production               | `master`     | `master` and `develop` |

If you’d like to contribute:

1. Fork this repository
2. Create an issue describing the feature or bug
3. Create a branch that corresponds to the issue, following the naming conventions above:
   - Features: branch off `develop` as `feature/[issue number]-[short description]` (e.g. `feature/37-automatise-upload-of-versions`)
   - Hotfixes: branch off `master` as `hotfix/[issue number]-[short description]`
4. Commit your changes
5. Push the branch
6. Open a Pull Request against `develop` (features) or `master` (hotfixes). Please ensure that an issue exists before submitting your contribution as a pull request

> `master` and `develop` are never committed to directly. `release/*` branches are created by the maintainers.
