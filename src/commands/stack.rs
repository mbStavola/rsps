use std::io::Write;

use ansi_term::Color;
use anyhow::Result;
use clap::Clap;
use rstack::TraceOptions;
use sysinfo::{ProcessExt, System};
use tabwriter::TabWriter;

use crate::commands::{ProcessArg, RspsSubcommand};

#[derive(Clap)]
pub struct StackCommand {
    process: ProcessArg,
}

impl RspsSubcommand for StackCommand {
    fn exec(&self, system: &mut System, tw: &mut TabWriter<Vec<u8>>) -> Result<()> {
        let process = self.process.as_system_process(system, tw)?;

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
