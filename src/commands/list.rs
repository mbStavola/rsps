use std::io::Write;

use ansi_term::Color;
use anyhow::Result;
use clap::Clap;
use rayon::prelude::*;
use sysinfo::{ProcessExt, System, SystemExt};
use tabwriter::TabWriter;

use crate::{commands::RspsSubcommand, util};

#[derive(Clap)]
pub struct ListCommand;

impl RspsSubcommand for ListCommand {
    fn exec(&self, system: &System, tw: &mut TabWriter<Vec<u8>>) -> Result<()> {
        let header = format!(
            "{}\t{}\t{}\t{}\n",
            Color::Cyan.paint("PID"),
            Color::Cyan.paint("Parent"),
            Color::Cyan.paint("Name"),
            Color::Cyan.paint("Path"),
        );
        tw.write_all(header.as_bytes())?;

        let processes = system
            .get_processes()
            .values()
            .par_bridge()
            .filter(|process| util::is_process_rusty(process).unwrap_or(false))
            .collect::<Vec<_>>();

        for process in processes {
            let row = format!(
                "{}\t{}\t{}\t{}\n",
                Color::Green.paint(process.pid().to_string()),
                Color::Green.paint(
                    process
                        .parent()
                        .map(|parent| parent.to_string())
                        .unwrap_or_else(|| "".to_string())
                ),
                Color::Green.paint(process.name()),
                Color::Green.paint(process.exe().to_str().unwrap_or("")),
            );

            tw.write_all(row.as_bytes())?;
        }

        Ok(())
    }
}
