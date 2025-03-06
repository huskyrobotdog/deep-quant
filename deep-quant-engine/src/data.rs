use std::path::Path;

use crate::config::DataConfig;
use anyhow::Result;
use heed::EnvOpenOptions;

pub fn new(config: &DataConfig, filename: &str) -> Result<()> {
    let path = Path::new(&config.dir).join(filename);
    let env = unsafe { EnvOpenOptions::default().open(path)? };
    let mut asd = env.write_txn()?;
    Ok(())
}
