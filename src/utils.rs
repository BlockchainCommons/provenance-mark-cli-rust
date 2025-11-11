#![allow(dead_code)]

use std::{env, io::Read, path::PathBuf};

use anyhow::{Result, bail};
use glob::glob;

/// Read a new path, supporting globbing, and resolving relative paths.
pub fn read_new_path(path: &PathBuf) -> Result<PathBuf> {
    let mut matches = glob(path.to_str().unwrap_or(""))?
        .filter_map(|entry| entry.ok())
        .collect::<Vec<_>>();

    if matches.len() > 1 {
        bail!("Glob pattern matches multiple paths; specify a single path.");
    }

    let effective_path = if matches.is_empty() {
        if path.is_relative() {
            let current_dir = env::current_dir()?;
            current_dir.join(path)
        } else {
            path.clone()
        }
    } else {
        matches.remove(0)
    };

    let cleaned_path = effective_path.components().collect::<PathBuf>();
    Ok(cleaned_path)
}

/// Read an existing directory path, supporting globbing, and resolving relative
/// paths.
pub fn read_existing_directory_path(path: &PathBuf) -> Result<PathBuf> {
    let effective_path = read_new_path(path)?;
    if !effective_path.is_dir() {
        bail!("Path is not a directory: {:?}", effective_path);
    }
    Ok(effective_path)
}

pub fn read_argument(argument: Option<&str>) -> Result<String> {
    let string = if let Some(arg) = argument {
        arg.to_string()
    } else {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s)?;
        s
    };
    if string.is_empty() {
        bail!("No argument provided");
    }
    Ok(string)
}
