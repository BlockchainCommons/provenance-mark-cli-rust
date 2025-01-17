//! A command line tool for managing provenance mark chains and generating provenance marks. See the main repo [README](https://github.com/BlockchainCommons/bc-provenance-mark-cli-rust/blob/master/README.md).

#[doc(hidden)]
mod cmd;
#[doc(hidden)]
mod exec;
#[doc(hidden)]
mod styles;
#[doc(hidden)]
mod data_types;
#[doc(hidden)]
mod envelope_args;
#[doc(hidden)]
mod utils;

use clap::{Parser, Subcommand};
use anyhow::Result;

use crate::exec::Exec;

/// A tool for managing provenance mark chains and generating provenance marks.
#[derive(Debug, Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(styles=styles::get_styles())]
#[doc(hidden)]
struct Cli {
    #[command(subcommand)]
    command: MainCommands,
}

#[derive(Debug, Subcommand)]
#[doc(hidden)]
enum MainCommands {
    New(cmd::new::CommandArgs),
    Next(cmd::next::CommandArgs),
    Print(cmd::print::CommandArgs),
}

#[doc(hidden)]
fn main() -> Result<()> {
    bc_envelope::register_tags();

    let cli = Cli::parse();

    let output = match cli.command {
        MainCommands::New(args) => args.exec(),
        MainCommands::Next(args) => args.exec(),
        MainCommands::Print(args) => args.exec(),
    };
    let output = output?;
    if !output.is_empty() {
        println!("{}", output);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use provenance_mark::ProvenanceSeed;
    use serde_json::json;

    #[test]
    fn test1() {
        let seed_str = "Jgk3vBEDvOjpQtjGDLu3kNQpIEPwg+HDNCL32dvFAS0=";
        let seed: ProvenanceSeed = serde_json::from_value(json!(seed_str)).unwrap();
        println!("{:?}", seed);
    }
}
