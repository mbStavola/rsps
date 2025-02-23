use std::{ffi::OsString, io::Write, str::FromStr};

use ansi_term::Color;
use anyhow::{Result, anyhow};
use sysinfo::{Pid, Process, System, Users};
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

use crate::rsinfo::RsInfo;

pub trait RspsSubcommand {
    fn exec(
        &self,
        system: &mut System,
        users: &mut Users,
        tw: &mut TabWriter<Vec<u8>>,
    ) -> Result<()>;
}

#[derive(Debug, Clone)]
pub enum ProcessArg {
    Pid(usize),
    Name(String),
}

impl ProcessArg {
    pub fn as_system_process<'a>(
        &self,
        system: &'a System,
        tw: &'_ mut TabWriter<Vec<u8>>,
    ) -> Result<(&'a Process, RsInfo)> {
        let process = match &self {
            ProcessArg::Pid(pid) => system.process(Pid::from(*pid)),
            ProcessArg::Name(name) => {
                let processes = system
                    .processes_by_name(&OsString::from(name))
                    .collect::<Vec<_>>();
                if processes.len() > 1 {
                    let warning = format!(
                        "{} {}\n\n",
                        Color::Yellow.paint("Multiple processes have this name."),
                        "Selecting the first."
                    );

                    tw.write_all(warning.as_bytes())?;
                }

                processes.iter().copied().next()
            }
        };

        process
            .ok_or_else(|| anyhow!("Process not found"))
            .and_then(|process| {
                util::is_process_rusty(process)?
                    .map(|info| (process, info))
                    .ok_or_else(|| anyhow!("This is not a Rust process"))
            })
    }
}

impl FromStr for ProcessArg {
    type Err = Box<dyn std::error::Error + Send + Sync>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(pid) = s.parse::<usize>() {
            return Ok(ProcessArg::Pid(pid));
        }

        Ok(ProcessArg::Name(s.to_owned()))
    }
}
