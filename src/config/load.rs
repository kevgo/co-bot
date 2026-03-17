use crate::config::{self, Data};
use crate::errors::Result;

pub fn load() -> Result<Data> {
    let data = config::file::load()?;
    Ok(Data { file: data })
}
