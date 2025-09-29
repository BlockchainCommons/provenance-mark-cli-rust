use std::{fs, path::PathBuf};

use anyhow::{Result, bail};
use clap::{Args, ValueEnum};
use dcbor::prelude::*;
use provenance_mark::{
    ProvenanceMarkGenerator, ProvenanceMarkInfo, ProvenanceMarkResolution,
    ProvenanceSeed, util::parse_date,
};

use super::{info::InfoArgs, print::OutputFormat, seed};
use crate::utils::read_new_path;

/// Initialize a directory with a new provenance mark chain.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Path to directory to be created. Must not already exist.
    path: PathBuf,

    /// A seed to use for the provenance mark chain, encoded as base64.
    /// If not supplied, a random seed is generated.
    #[arg(short, long, value_parser = seed::parse_seed)]
    seed: Option<ProvenanceSeed>,

    /// The resolution of the provenance mark chain.
    #[arg(short, long, default_value = "quartile")]
    resolution: Resolution,

    /// A comment to be included for the genesis mark. (Comments are not part
    /// of the mark itself.)
    #[arg(short, long, default_value = "Genesis mark.")]
    comment: String,

    /// The date of the genesis mark. If not supplied, the current date is
    /// used.
    #[arg(short, long)]
    #[clap(value_parser = parse_date)]
    date: Option<Date>,

    /// Suppress informational status output on stderr/stdout.
    #[arg(short, long)]
    quiet: bool,

    /// Output format for the creation summary.
    #[arg(long, value_enum, default_value_t = OutputFormat::Markdown)]
    format: OutputFormat,

    #[command(flatten)]
    info: InfoArgs,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Resolution {
    /// Good for physical works of art and applications requiring minimal mark
    /// size.
    Low,
    /// Good for digital works of art.
    Medium,
    /// Good for general use.
    Quartile,
    /// Industrial strength, largest mark.
    High,
}

impl Resolution {
    fn as_provenance_mark_resolution(&self) -> ProvenanceMarkResolution {
        match self {
            Resolution::Low => ProvenanceMarkResolution::Low,
            Resolution::Medium => ProvenanceMarkResolution::Medium,
            Resolution::Quartile => ProvenanceMarkResolution::Quartile,
            Resolution::High => ProvenanceMarkResolution::High,
        }
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        // Create the directory, ensuring it doesn't already exist.
        let path = self.create_dir()?;

        // Create the `marks` subdirectory inside `path`.
        let marks_path = path.join("marks");
        fs::create_dir(&marks_path)?;

        let mut generator: ProvenanceMarkGenerator =
            if let Some(seed) = self.seed.clone() {
                ProvenanceMarkGenerator::new_with_seed(
                    self.resolution.as_provenance_mark_resolution(),
                    seed,
                )
            } else {
                ProvenanceMarkGenerator::new_random(
                    self.resolution.as_provenance_mark_resolution(),
                )
            };

        // Generate the genesis mark.
        let date = self.date.clone().unwrap_or_else(Date::now);
        let info = self.info.to_cbor()?;
        let mark = match info {
            Some(info_cbor) => generator.next(date, Some(info_cbor)),
            None => generator.next(date, None::<CBOR>),
        };
        let mark_info =
            ProvenanceMarkInfo::new(mark.clone(), self.comment.clone());

        // Serialize the mark to JSON and write it as `mark-seq.json` to
        // `path/marks`.
        let mark_json = serde_json::to_string_pretty(&mark_info)?;
        let mark_path = marks_path.join(format!("mark-{}.json", mark.seq()));
        fs::write(&mark_path, mark_json)?;

        // Serialize `generator` to JSON and write it as `generator.json` to
        // `path`.
        let generator_json = serde_json::to_string_pretty(&generator)?;
        let generator_path = path.join("generator.json");
        fs::write(generator_path, generator_json)?;

        // Return a markdown summary of the provenance mark chain and the
        // genesis mark.
        let status_lines = [
            format!("Provenance mark chain created at: {}", path.display()),
            format!("Mark {} written to: {}", mark.seq(), mark_path.display()),
        ];

        match self.format {
            OutputFormat::Markdown => {
                let mut paragraphs: Vec<String> = Vec::new();
                if !self.quiet {
                    paragraphs.extend(status_lines.iter().cloned());
                }
                paragraphs.push(mark_info.markdown_summary());
                Ok(paragraphs.join("\n\n"))
            }
            OutputFormat::Ur => {
                if !self.quiet {
                    for line in &status_lines {
                        eprintln!("{}", line);
                    }
                }
                Ok(mark_info.ur().to_string())
            }
            OutputFormat::Json => {
                if !self.quiet {
                    for line in &status_lines {
                        eprintln!("{}", line);
                    }
                }
                serde_json::to_string_pretty(&mark_info).map_err(Into::into)
            }
        }
    }
}

impl CommandArgs {
    fn create_dir(&self) -> Result<PathBuf> {
        let path = read_new_path(&self.path)?;

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
