use std::io::Write;

use anyhow::Result;
use clap::{Parser, Subcommand};
use rsps_vergen_emboss::rsps_vergen_emboss;
use sysinfo::RefreshKind;
use tabwriter::TabWriter;

#[cfg(target_os = "linux")]
use crate::commands::StackCommand;
use crate::commands::{InspectCommand, ListCommand, RspsSubcommand, TreeCommand};

mod commands;
mod rsinfo;
mod util;

// Embed some build information into the final binary that
// can be used by rsps... wait isn't that us?
rsps_vergen_emboss!(group = rsps);

/// List and debug Rust programs currently running on your system.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    /// Lists all running Rust processes
    List,
    /// Displays a tree of parent and child Rust processes
    Tree,
    /// Displays detailed information about a Rust process
    Inspect(InspectCommand),
    /// Dump the stack for a running Rust process
    #[cfg(target_os = "linux")]
    Stack(StackCommand),
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let subcommand = opts.subcmd.unwrap_or(SubCommand::List);
    let rsps_subcommand: &dyn RspsSubcommand = match &subcommand {
        SubCommand::List => &ListCommand,
        SubCommand::Tree => &TreeCommand,
        SubCommand::Inspect(command) => command,
        #[cfg(target_os = "linux")]
        SubCommand::Stack(command) => command,
    };

    let mut system = sysinfo::System::new_with_specifics(RefreshKind::everything());
    let mut users = sysinfo::Users::new_with_refreshed_list();

    let mut tw = TabWriter::new(Vec::<u8>::new());
    rsps_subcommand.exec(&mut system, &mut users, &mut tw)?;
    tw.flush()?;

    let output = tw.into_inner()?;
    let output = String::from_utf8(output)?;
    println!("{}", output);

    Ok(())
}
