use std::error::Error;
use {get_stdout, run};

#[derive(Debug)]
struct NoDesktopsError;

impl Error for NoDesktopsError {}

impl std::fmt::Display for NoDesktopsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "no desktops found")
    }
}

fn get_desktops() -> Result<Vec<String>, Box<dyn Error>> {
    let stdout = get_stdout("xfconf-query", &["--channel", "xfce4-desktop", "--list"])?;
    let desktops = stdout
        .split('\n')
        .filter(|line| line.ends_with("last-image"))
        .map(|desktop| desktop.to_string())
        .collect::<Vec<String>>();

    if desktops.is_empty() {
        return Err(Box::new(NoDesktopsError));
    }

    Ok(desktops)
}

pub fn get() -> Result<String, Box<dyn Error>> {
    let desktops = get_desktops()?;
    let path = get_stdout(
        "xfconf-query",
        &["--channel", "xfce4-desktop", "--property", &desktops[0]],
    )?;

    Ok(path.trim().to_string())
}

pub fn set(path: &str) -> Result<(), Box<dyn Error>> {
    for desktop in get_desktops()? {
        run(
            "xfconf-query",
            &[
                "--channel",
                "xfce4-desktop",
                "--property",
                &desktop,
                "--set",
                path,
            ],
        )?;
    }

    Ok(())
}
