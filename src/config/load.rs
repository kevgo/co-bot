use crate::config::{self, Config};
use crate::errors::Result;

pub fn load() -> Result<Config> {
    let data = config::file::load()?;
    Ok(Config { data })
}
