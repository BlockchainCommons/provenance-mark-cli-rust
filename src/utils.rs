#![allow(dead_code)]

use std::{collections::HashSet, env, io::Read, path::PathBuf};
use anyhow::{ bail, Result };
use bc_envelope::prelude::*;
use glob::glob;

/// Utility function to read a new path, supporting globbing, and resolving relative paths.
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

pub fn read_password(prompt: &str, password: Option<&str>) -> Result<String> {
    if let Some(password) = password {
        Ok(password.to_string())
    } else {
        rpassword::prompt_password(prompt).map_err(Into::into)
    }
}

pub fn read_argument(argument: Option<&str>) -> Result<String> {
    let mut string = String::new();
    if argument.is_none() {
        std::io::stdin().read_to_string(&mut string)?;
    } else {
        string = argument.as_ref().unwrap().to_string();
    }
    if string.is_empty() {
        bail!("No argument provided");
    }
    Ok(string.to_string())
}

pub fn read_envelope(envelope: Option<&str>) -> Result<Envelope> {
    let mut ur_string = String::new();
    if envelope.is_none() {
        std::io::stdin().read_line(&mut ur_string)?;
    } else {
        ur_string = envelope.as_ref().unwrap().to_string();
    }
    if ur_string.is_empty() {
        bail!("No envelope provided");
    }
    // Just try to parse the envelope as a ur:envelope string first
    if let Ok(envelope) = Envelope::from_ur_string(ur_string.trim()) {
        Ok(envelope)
    // If that fails, try to parse the envelope as a ur:<any> string
    } else if let Ok(ur) = UR::from_ur_string(ur_string.trim()) {
        let cbor = ur.cbor();
        // Try to parse the CBOR into an envelope
        if let Ok(envelope) = Envelope::from_tagged_cbor(cbor) {
            Ok(envelope)
        } else {
            todo!();
        }
    } else {
        bail!("Invalid envelope");
    }
}

pub fn parse_digest(target: &str) -> Result<Digest> {
    let ur = UR::from_ur_string(target)?;
    let digest = match ur.ur_type_str() {
        "digest" => { Digest::from_ur(&ur)? }
        "envelope" => { Envelope::from_ur(&ur)?.digest().into_owned() }
        _ => {
            bail!("Invalid digest type: {}", ur.ur_type_str());
        }
    };
    Ok(digest)
}

pub fn parse_digests(target: &str) -> Result<HashSet<Digest>> {
    let target = target.trim();
    if target.is_empty() {
        Ok(HashSet::new())
    } else {
        target.split(' ').map(parse_digest).collect::<Result<HashSet<Digest>>>()
    }
}
