use std::{
    fmt::{Display, Formatter},
    io::Write,
    str::FromStr,
};

use ansi_term::Color;
use anyhow::{anyhow, Result};
use sysinfo::{Process, System, SystemExt};
use tabwriter::TabWriter;

use crate::util;

mod inspect;
mod list;
#[cfg(target_os = "linux")]
mod stack;
mod tree;

pub use inspect::InspectCommand;
pub use list::ListCommand;
#[cfg(target_os = "linux")]
pub use stack::StackCommand;
pub use tree::TreeCommand;

pub trait RspsSubcommand {
    fn exec(&self, system: &mut System, tw: &mut TabWriter<Vec<u8>>) -> Result<()>;
}

pub enum ProcessArg {
    Pid(i32),
    Name(String),
}

impl ProcessArg {
    pub fn as_system_process<'a>(
        &self,
        system: &'a System,
        tw: &'_ mut TabWriter<Vec<u8>>,
    ) -> Result<&'a Process> {
        let process = match &self {
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
        };

        process
            .ok_or_else(|| anyhow!("Process not found"))
            .and_then(|process| {
                if util::is_process_rusty(process)? {
                    Ok(process)
                } else {
                    Err(anyhow!("This is not a Rust process"))
                }
            })
    }
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
