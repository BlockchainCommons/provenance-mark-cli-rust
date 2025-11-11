use std::{fs, path::PathBuf};

use anyhow::{Result, bail};
use bc_ur::URDecodable;
use clap::{Args, ValueEnum};
use provenance_mark::{
    ProvenanceMark, ProvenanceMarkInfo, ValidationReportFormat,
};

use crate::utils::read_existing_directory_path;

/// Validate one or more provenance marks.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// One or more provenance mark URs to validate.
    #[arg(required_unless_present = "dir")]
    marks: Vec<String>,

    /// Path to a chain directory containing marks to validate.
    #[arg(short, long, conflicts_with = "marks")]
    dir: Option<PathBuf>,

    /// Report issues as warnings without failing.
    #[arg(short, long)]
    warn: bool,

    /// Output format for the validation report.
    #[arg(long, value_enum, default_value_t = Format::Text)]
    format: Format,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum Format {
    Text,
    JsonCompact,
    JsonPretty,
}

impl Format {
    fn as_validation_report_format(&self) -> ValidationReportFormat {
        match self {
            Format::Text => ValidationReportFormat::Text,
            Format::JsonCompact => ValidationReportFormat::JsonCompact,
            Format::JsonPretty => ValidationReportFormat::JsonPretty,
        }
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        // Collect marks from either URs or directory
        let marks = if let Some(dir_path) = &self.dir {
            self.load_marks_from_dir(dir_path)?
        } else {
            self.parse_marks_from_urs(&self.marks)?
        };

        // Validate the marks
        let report = ProvenanceMark::validate(marks);

        // Format the output
        let output = report.format(self.format.as_validation_report_format());

        // Determine if we should fail
        if report.has_issues() && !self.warn {
            bail!("Validation failed with issues:\n{}", output);
        }

        Ok(output)
    }
}

impl CommandArgs {
    fn parse_marks_from_urs(
        &self,
        ur_strings: &[String],
    ) -> Result<Vec<ProvenanceMark>> {
        let mut marks = Vec::new();
        for ur_string in ur_strings {
            let mark = ProvenanceMark::from_ur_string(ur_string.trim())
                .map_err(|e| {
                    anyhow::anyhow!(
                        "Failed to parse provenance mark from '{}': {}",
                        ur_string,
                        e
                    )
                })?;
            marks.push(mark);
        }
        Ok(marks)
    }

    fn load_marks_from_dir(
        &self,
        dir_path: &PathBuf,
    ) -> Result<Vec<ProvenanceMark>> {
        // Get the chain's directory path
        let path = read_existing_directory_path(dir_path)?;

        // Get the marks subdirectory
        let marks_path = path.join("marks");
        if !marks_path.exists() || !marks_path.is_dir() {
            bail!("Marks subdirectory not found: {}", marks_path.display());
        }

        // Read all JSON files from the marks directory
        let entries = fs::read_dir(&marks_path)?;
        let mut mark_files: Vec<_> = entries
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("json")
                    {
                        Some(path)
                    } else {
                        None
                    }
                })
            })
            .collect();

        // Sort the files by name to ensure proper ordering
        mark_files.sort();

        if mark_files.is_empty() {
            bail!("No mark JSON files found in: {}", marks_path.display());
        }

        // Parse each JSON file and extract the mark
        let mut marks = Vec::new();
        for mark_file in mark_files {
            let json_content = fs::read_to_string(&mark_file).map_err(|e| {
                anyhow::anyhow!("Failed to read {}: {}", mark_file.display(), e)
            })?;

            let mark_info: ProvenanceMarkInfo =
                serde_json::from_str(&json_content).map_err(|e| {
                    anyhow::anyhow!(
                        "Failed to parse JSON from {}: {}",
                        mark_file.display(),
                        e
                    )
                })?;

            marks.push(mark_info.mark().clone());
        }

        Ok(marks)
    }
}
