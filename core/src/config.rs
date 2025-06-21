use anyhow::{Context, Result};
use std::{env, fs, path::PathBuf};
use toml_edit::DocumentMut;

fn config_path() -> Result<PathBuf> {
    let home = env::var("HOME").context("HOME env var not set")?;
    Ok(PathBuf::from(home).join(".config/chezmoi/chezmoi.toml"))
}

pub fn load_config() -> Result<(DocumentMut, PathBuf)> {
    let path = config_path()?;
    let s =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;
    let doc = s.parse::<DocumentMut>().context("Failed to parse TOML")?;
    Ok((doc, path))
}

pub fn save_config(doc: &DocumentMut, path: &PathBuf) -> Result<()> {
    fs::write(path, doc.to_string())
        .with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}
