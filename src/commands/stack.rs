use std::io::Write;

use ansi_term::Color;
use anyhow::{anyhow, Result};
use clap::Clap;
use rstack::TraceOptions;
use sysinfo::{ProcessExt, System, SystemExt};
use tabwriter::TabWriter;

use crate::{commands::RspsSubcommand, util};

#[derive(Clap)]
pub struct StackCommand {
    #[allow(unused)]
    pid: i32,
}

impl RspsSubcommand for StackCommand {
    fn exec(&self, system: &System, tw: &mut TabWriter<Vec<u8>>) -> Result<()> {
        let process = system
            .get_process(self.pid)
            .ok_or_else(|| anyhow!("Process not found"))?;

        if !util::is_process_rusty(process)? {
            return Err(anyhow!("This is not a Rust process"));
        }

        let header = format!(
            "{}\t{}\t{}\t{}\n",
            Color::Cyan.paint("PID"),
            Color::Cyan.paint("Parent"),
            Color::Cyan.paint("Name"),
            Color::Cyan.paint("Path"),
        );
        tw.write_all(header.as_bytes())?;

        let mut tracer = TraceOptions::new();
        tracer
            .symbols(true)
            .ptrace_attach(true)
            .snapshot(true)
            .thread_names(true);

        let trace = tracer.trace(process.pid() as u32)?;
        for thread in trace.threads() {
            let thread_stack_header = format!(
                "{}: {}\t{}: {}\n{}:\n",
                Color::Cyan.paint("Thread ID"),
                Color::Green.paint(thread.id().to_string()),
                Color::Cyan.paint("Name"),
                Color::Green.paint(thread.name().unwrap_or_else(|| "")),
                Color::Cyan.paint("Stack Dump"),
            );
            tw.write_all(thread_stack_header.as_bytes())?;
            for (i, frame) in thread.frames().iter().enumerate() {
                let output = match frame.symbol() {
                    Some(symbol) => {
                        format!(
                            "{} #{}: {}\t{}\n",
                            Color::Cyan.paint("Frame"),
                            Color::White.paint(i.to_string()),
                            Color::Yellow.paint(format!("{:0x}", symbol.address())),
                            Color::Green.paint(symbol.name()),
                        )
                    }
                    None => {
                        format!(
                            "{} #{}: {}\n",
                            Color::Cyan.paint("Frame"),
                            Color::White.paint(i.to_string()),
                            Color::Red.paint("<no symbol>"),
                        )
                    }
                };

                tw.write_all(output.as_bytes())?;
            }
        }

        Ok(())
    }
}
