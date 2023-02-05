use anyhow::{anyhow, Result};

use crate::time;

pub fn copy(path: String, version: String) -> Result<String> {
    let bu_name = backup_name(path.clone(), version);
    match std::fs::copy(path.clone(), bu_name.clone()) {
        Ok(_) => Ok(bu_name),
        Err(e) => {
            let msg = "Could not copy file";
            log::error!("{msg} {path:?} ({e:})");
            Err(anyhow!("{msg} {path:?} ({e:})"))
        }
    }
}

pub fn backup_name(path: String, version: String) -> String {
    format!("{path}-{}-v{version}", time::simple_timestamp())
}
