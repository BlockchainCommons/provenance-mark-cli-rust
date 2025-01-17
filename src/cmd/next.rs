use std::{ fs, path::PathBuf };

use bc_envelope::prelude::*;
use clap::{ Args, ValueEnum };
use anyhow::{ bail, Result };
use provenance_mark::{ ProvenanceMarkGenerator, ProvenanceMarkInfo, ProvenanceMarkResolution };

use crate::utils::read_new_path;

/// Generate the next provenance mark in a chain.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Path to the chain's directory. Must already exist.
    path: Option<PathBuf>,

    /// The resolution of the provenance mark chain.
    #[arg(short, long, default_value = "quartile")]
    resolution: Resolution,

    /// A comment to be included for the genesis mark. (Comments are not part of
    /// the mark itself.)
    #[arg(short, long, default_value = "Genesis mark.")]
    comment: String,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Resolution {
    /// Good for physical works of art and applications requiring minimal mark size.
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
        todo!();
        // // Create the directory, ensuring it doesn't already exist.
        // let path = self.create_dir()?;

        // // Create the `marks` subdirectory inside `path`.
        // let marks_path = path.join("marks");
        // fs::create_dir(&marks_path)?;

        // // Create the generator
        // let mut generator = ProvenanceMarkGenerator::new_random(
        //     self.resolution.as_provenance_mark_resolution()
        // );

        // // Generate the genesis mark.
        // let mark = generator.next(dcbor::Date::now(), None::<CBOR>);
        // let mark_info = ProvenanceMarkInfo::new(mark.clone(), self.comment.clone());

        // // Serialize the mark to JSON and write it as `mark-seq.json` to
        // // `path/marks`.
        // let mark_json = serde_json::to_string_pretty(&mark_info)?;
        // let mark_path = marks_path.join(format!("mark-{}.json", mark.seq()));
        // fs::write(&mark_path, mark_json)?;

        // // Serialize `generator` to JSON and write it as `generator.json` to
        // // `path`.
        // let generator_json = serde_json::to_string_pretty(&generator)?;
        // let generator_path = path.join("generator.json");
        // fs::write(generator_path, generator_json)?;

        // // Return a markdown summary of the provenance mark chain and the
        // // genesis mark.
        // let mut paragraphs: Vec<String> = Vec::new();
        // paragraphs.push(format!("Provenance mark chain created at: {}", path.display()));
        // paragraphs.push(format!("Mark {} written to: {}", mark.seq(), mark_path.display()));
        // paragraphs.push(mark_info.markdown_summary());
        // Ok(paragraphs.join("\n\n"))
    }
}
