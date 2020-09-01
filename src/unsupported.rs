use Result;

pub fn get() -> Result<String, Box<dyn std::error::Error>> {
    Err("unsupported operating system".into())
}

pub fn set_from_path(_: &str) -> Result<(), Box<dyn std::error::Error>> {
    Err("unsupported operating system".into())
}

#[cfg("from-url")]
pub fn set_from_url(_: &str) -> Result<(), Box<dyn std::error::Error>> {
    Err("unsupported operating system".into())
}
