use Result;

pub fn get() -> Result<String> {
    Err("unsupported operating system".into())
}

pub fn set_from_path(_: &str) -> Result<()> {
    Err("unsupported operating system".into())
}

pub fn set_from_url(_: &str) -> Result<()> {
    Err("unsupported operating system".into())
}
