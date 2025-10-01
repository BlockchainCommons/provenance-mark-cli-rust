use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Args;
use dcbor::prelude::*;
use provenance_mark::{
    ProvenanceMarkGenerator, ProvenanceMarkInfo, util::parse_date,
};

use super::{info::InfoArgs, print::OutputFormat};
use crate::utils::read_existing_directory_path;

/// Generate the next provenance mark in a chain.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Path to the chain's directory. Must already exist.
    path: PathBuf,

    /// A comment to be included for the mark. (Comments are not part of the
    /// mark itself.)
    #[arg(short, long, default_value = "Blank.")]
    comment: String,

    /// The date of the next mark. If not supplied, the current date is used.
    #[arg(short, long, value_parser = parse_date)]
    date: Option<Date>,

    /// Suppress informational status output on stderr/stdout.
    #[arg(short, long)]
    quiet: bool,

    /// Output format for the mark.
    #[arg(long, value_enum, default_value_t = OutputFormat::Markdown)]
    format: OutputFormat,

    #[command(flatten)]
    info: InfoArgs,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        // Get the chain's directory path.
        let path = read_existing_directory_path(&self.path)?;

        // Read the generator from `path/generator.json`.
        let generator_path = path.join("generator.json");
        let generator_json = fs::read_to_string(&generator_path)?;
        let mut generator: ProvenanceMarkGenerator =
            serde_json::from_str(&generator_json)?;

        // Generate the next mark.
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
        let marks_path = path.join("marks");
        let mark_json = serde_json::to_string_pretty(&mark_info)?;
        let mark_path = marks_path.join(format!("mark-{}.json", mark.seq()));
        fs::write(&mark_path, mark_json)?;

        // Serialize `generator` to JSON and write it back to
        // `path/generator.json`.
        let generator_json = serde_json::to_string_pretty(&generator)?;
        fs::write(generator_path, generator_json)?;

        // Return output based on format.
        let status_line =
            format!("Mark {} written to: {}", mark.seq(), mark_path.display());

        match self.format {
            OutputFormat::Markdown => {
                let mut paragraphs: Vec<String> = Vec::new();
                if !self.quiet {
                    paragraphs.push(status_line);
                }
                paragraphs.push(mark_info.markdown_summary());
                Ok(paragraphs.join("\n\n"))
            }
            OutputFormat::Ur => {
                if !self.quiet {
                    eprintln!("{}", status_line);
                }
                Ok(mark_info.ur().to_string())
            }
            OutputFormat::Json => {
                if !self.quiet {
                    eprintln!("{}", status_line);
                }
                serde_json::to_string_pretty(&mark_info).map_err(Into::into)
            }
        }
    }
}
