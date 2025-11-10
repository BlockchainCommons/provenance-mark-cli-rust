use anyhow::{Result, bail};
use bc_ur::URDecodable;
use clap::Args;
use provenance_mark::ProvenanceMark;

/// Validate one or more provenance marks.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// One or more provenance mark URs to validate.
    #[arg(required = true)]
    marks: Vec<String>,

    /// Report issues as warnings without failing.
    #[arg(short, long)]
    warn: bool,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        // Parse the marks from UR strings
        let mut marks = Vec::new();
        for ur_string in &self.marks {
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

        // Validate the marks
        let report = ProvenanceMark::validate(marks);

        // Format the output
        let output = report.format();

        // Determine if we should fail
        if report.has_issues() && !self.warn {
            bail!("Validation failed with issues:\n{}", output);
        }

        Ok(output)
    }
}
