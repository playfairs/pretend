use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use directories::BaseDirs;

use crate::error::{Error, Result};

pub fn config_dir() -> Result<PathBuf> {
    let base_dirs = BaseDirs::new().ok_or_else(|| Error::Config("base directories unavailable".into()))?;
    let path = base_dirs.config_dir().join("pretend");
    fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn applications_path() -> Result<PathBuf> {
    let path = config_dir()?.join("applications.json");
    if \!path.exists() {
        let fallback = repository_applications_path();
        if let Some(source) = fallback {
            fs::copy(source, &path)?;
        }
    }
    Ok(path)
}

pub fn repository_applications_path() -> Option<PathBuf> {
    let manifest_dir = PathBuf::from(env\!("CARGO_MANIFEST_DIR"));
    let candidate = manifest_dir.join("../../data/applications.json");
    let path = candidate.canonicalize().ok()?;
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

pub fn ensure_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}
