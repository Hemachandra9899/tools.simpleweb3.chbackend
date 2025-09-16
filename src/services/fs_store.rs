use anyhow::Context;
use std::path::Path;
use tokio::fs;

use crate::models::item::DataFile;

pub async fn read_data<P: AsRef<Path>>(path: P) -> anyhow::Result<DataFile> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(DataFile::empty());
    }
    let bytes = fs::read(path)
        .await
        .with_context(|| format!("reading {}", path.display()))?;
    if bytes.is_empty() {
        return Ok(DataFile::empty());
    }
    let data: DataFile = serde_json::from_slice(&bytes)
        .with_context(|| format!("parsing JSON {}", path.display()))?;
    Ok(data)
}

pub async fn write_data<P: AsRef<Path>>(path: P, data: &DataFile) -> anyhow::Result<()> {
    let path = path.as_ref();
    let json = serde_json::to_vec_pretty(data).context("serializing JSON")?;
    fs::write(path, json)
        .await
        .with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}
