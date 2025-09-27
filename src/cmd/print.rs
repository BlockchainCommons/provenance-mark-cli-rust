use std::{fs, path::PathBuf};

use anyhow::{Result, bail};
use clap::{Args, ValueEnum};
use provenance_mark::{ProvenanceMarkGenerator, ProvenanceMarkInfo};

use crate::utils::read_existing_directory_path;

/// Prints provenance marks in a chain.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Path to the chain's directory. Must already exist.
    path: PathBuf,

    /// The sequence number of the first mark to print. If not supplied, the
    /// first mark (genesis mark) is used.
    #[arg(short, long, default_value = "0")]
    start: u32,

    /// The sequence number of the last mark to print. If not supplied, the
    /// last mark in the chain is used.
    #[arg(short, long)]
    end: Option<u32>,

    /// Output format for the rendered marks.
    #[arg(long, value_enum, default_value_t = OutputFormat::Markdown)]
    format: OutputFormat,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Markdown,
    Ur,
    Json,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        // Get the chain's directory path.
        let path = read_existing_directory_path(&self.path)?;

        // Read the generator from `path/generator.json`.
        let generator_path = path.join("generator.json");
        let generator_json = fs::read_to_string(generator_path)?;
        let generator: ProvenanceMarkGenerator =
            serde_json::from_str(&generator_json)?;

        // Validate the start and end sequence numbers.
        let last_valid_seq = generator.next_seq() - 1;
        let start_seq = self.start;
        let end_seq = self.end.unwrap_or(last_valid_seq);
        if start_seq > end_seq {
            bail!(
                "The start sequence number must be less than or equal to the end sequence number."
            );
        }
        if end_seq > last_valid_seq {
            bail!(
                "The end sequence number must be less than or equal to the last valid sequence number."
            );
        }

        // Collect the requested marks.
        let mut mark_infos: Vec<ProvenanceMarkInfo> = Vec::new();
        for seq in start_seq..=end_seq {
            let mark_path =
                path.join("marks").join(format!("mark-{}.json", seq));
            let mark_json = fs::read_to_string(&mark_path)?;
            let mark_info: ProvenanceMarkInfo =
                serde_json::from_str(&mark_json)?;
            mark_infos.push(mark_info);
        }

        match self.format {
            OutputFormat::Markdown => {
                let summaries: Vec<String> = mark_infos
                    .iter()
                    .map(ProvenanceMarkInfo::markdown_summary)
                    .collect();
                Ok(summaries.join("\n"))
            }
            OutputFormat::Ur => {
                let urs: Vec<String> = mark_infos
                    .iter()
                    .map(|info| info.ur().to_string())
                    .collect();
                Ok(urs.join("\n"))
            }
            OutputFormat::Json => {
                serde_json::to_string_pretty(&mark_infos).map_err(Into::into)
            }
        }
    }
}
