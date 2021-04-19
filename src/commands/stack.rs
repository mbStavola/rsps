use std::{
    fmt::{Display, Formatter},
    io::Write,
    str::FromStr,
};

use ansi_term::Color;
use anyhow::{anyhow, Result};
use clap::Clap;
use rstack::TraceOptions;
use sysinfo::{ProcessExt, System, SystemExt};
use tabwriter::TabWriter;

use crate::{commands::RspsSubcommand, util};

#[derive(Clap)]
pub struct StackCommand {
    process: ProcessArg,
}

pub enum ProcessArg {
    Pid(i32),
    Name(String),
}

impl FromStr for ProcessArg {
    type Err = ParseProcessArgError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(pid) = s.parse::<i32>() {
            return Ok(ProcessArg::Pid(pid));
        }

        Ok(ProcessArg::Name(s.to_owned()))
    }
}

#[derive(Debug)]
pub struct ParseProcessArgError {
    _priv: (),
}

impl Display for ParseProcessArgError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Could not parse process arg")
    }
}

impl RspsSubcommand for StackCommand {
    fn exec(&self, system: &System, tw: &mut TabWriter<Vec<u8>>) -> Result<()> {
        let process = match &self.process {
            ProcessArg::Pid(pid) => system.get_process(*pid),
            ProcessArg::Name(name) => {
                let processes = system.get_process_by_name(name);
                if processes.len() > 1 {
                    let warning = format!(
                        "{} {}\n\n",
                        Color::Yellow.paint("Multiple processes have this name."),
                        "Dumping the stack of the first."
                    );

                    tw.write_all(warning.as_bytes())?;
                }

                processes.into_iter().next()
            }
        }
        .ok_or_else(|| anyhow!("Process not found"))?;

        if !util::is_process_rusty(process)? {
            return Err(anyhow!("This is not a Rust process"));
        }

        let mut tracer = TraceOptions::new();
        tracer
            .symbols(true)
            .ptrace_attach(true)
            // TODO(Matt): Investigate this
            //  For some reason, this causes rsps to blow up with permissions errors,
            //  even when strace is working just fine...
            // .snapshot(true)
            .thread_names(true);

        let trace = tracer.trace(process.pid() as u32)?;
        for thread in trace.threads() {
            let thread_stack_header = format!(
                "{}: {}\t{}: {}\n{}:\n",
                Color::Cyan.paint("Thread ID"),
                Color::Green.paint(thread.id().to_string()),
                Color::Cyan.paint("Name"),
                Color::Green.paint(thread.name().unwrap_or("")),
                Color::Cyan.paint("Stack Dump"),
            );
            tw.write_all(thread_stack_header.as_bytes())?;
            for (i, frame) in thread.frames().iter().enumerate() {
                let output = match frame.symbol() {
                    Some(symbol) => {
                        let symbol_name = rustc_demangle::demangle(symbol.name());

                        format!(
                            "{} #{}: {}\t{}\n",
                            Color::Cyan.paint("Frame"),
                            Color::White.paint(i.to_string()),
                            Color::Yellow.paint(format!("{:0x}", symbol.address())),
                            Color::Green.paint(format!("{}", symbol_name)),
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
            tw.write_all("\n".as_bytes())?;
        }

        Ok(())
    }
}
