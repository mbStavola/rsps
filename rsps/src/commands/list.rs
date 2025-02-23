use std::io::Write;

use ansi_term::Color;
use anyhow::Result;
use rayon::prelude::*;
use sysinfo::{System, Users};
use tabwriter::TabWriter;

use crate::{commands::RspsSubcommand, util};

#[derive(Debug)]
pub struct ListCommand;

impl RspsSubcommand for ListCommand {
    fn exec(
        &self,
        system: &mut System,
        _users: &mut Users,
        tw: &mut TabWriter<Vec<u8>>,
    ) -> Result<()> {
        let header = format!(
            "{}\t{}\t{}\t{}\t{}\t{}\n",
            Color::Cyan.paint("PID"),
            Color::Cyan.paint("Parent"),
            Color::Cyan.paint("Name"),
            Color::Cyan.paint("Command"),
            Color::Cyan.paint("Rust Ver."),
            Color::Cyan.paint("Program Ver."),
        );
        tw.write_all(header.as_bytes())?;

        let processes = system
            .processes()
            .values()
            .par_bridge()
            .filter_map(|process| {
                util::is_process_rusty(process)
                    .ok()
                    .flatten()
                    .map(|info| (process, info))
            })
            .collect::<Vec<_>>();

        for (process, info) in processes {
            let row = format!(
                "{}\t{}\t{}\t{}\t{}\t{}\n",
                Color::Green.paint(process.pid().to_string()),
                Color::Green.paint(
                    process
                        .parent()
                        .map(|parent| parent.to_string())
                        .unwrap_or_else(|| "".to_string())
                ),
                Color::Green.paint(process.name().to_string_lossy()),
                Color::Green.paint(process.exe().and_then(|exe| exe.to_str()).unwrap_or("")),
                Color::Green.paint(
                    info.rust_version()
                        .map(String::as_str)
                        .unwrap_or("<unknown>")
                ),
                Color::Green.paint(
                    info.program_version()
                        .map(String::as_str)
                        .unwrap_or("<unknown>")
                ),
            );

            tw.write_all(row.as_bytes())?;
        }

        Ok(())
    }
}
