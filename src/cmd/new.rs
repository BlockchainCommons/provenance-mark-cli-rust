use std::{fs, path::PathBuf};

use clap::Args;
use anyhow::{bail, Result};

use crate::utils::read_new_path;

/// Initialize a directory with a new provenance mark chain.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Output format.
    path: Option<PathBuf>,

}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let path = self.create_dir()?;
        Ok(format!("Directory created: {}", path.display()))
    }
}

impl CommandArgs {
    fn create_dir(&self) -> Result<PathBuf> {
        let path = read_new_path(self.path.as_ref())?;

        // Ensure the directory doesn't already exist.
        if path.exists() {
            bail!("Path already exists: {}", path.display());
        }

        // Ensure the parent directory exists.
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                bail!("Parent directory does not exist: {}", parent.display());
            }
        } else {
            bail!("Path has no parent directory: {}", path.display());
        }

        // Create the new directory.
        fs::create_dir(&path)?;

        Ok(path)
    }
}
