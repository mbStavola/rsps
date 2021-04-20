use std::io::Write;

use ansi_term::Color;
use anyhow::{anyhow, Result};
use byte_unit::{Byte, ByteUnit};
use clap::Clap;
use sysinfo::{ProcessExt, System, SystemExt, User, UserExt};
use tabwriter::TabWriter;

use crate::commands::{ProcessArg, RspsSubcommand};

#[derive(Clap)]
pub struct InspectCommand {
    process: ProcessArg,
}

impl RspsSubcommand for InspectCommand {
    fn exec(&self, system: &mut System, tw: &mut TabWriter<Vec<u8>>) -> Result<()> {
        // Quickly refresh a few times to get a nice CPU usage sample
        system.refresh_processes();
        let pid = {
            let process = self.process.as_system_process(system, tw)?;
            process.pid()
        };
        system.refresh_process(pid);

        let process = system
            .get_process(pid)
            .ok_or_else(|| anyhow!("Process disappeared since last sample"))?;

        let cpu_usage = format!("{:.2}%", process.cpu_usage());

        let memory_utilization = process.memory() as f32 / system.get_available_memory() as f32;
        let memory_usage = format!(
            "{} ({})",
            Byte::from_unit(process.memory() as f64, ByteUnit::KB)?.get_appropriate_unit(true),
            format!("{:.2}%", memory_utilization * 100.0)
        );

        let user_id = process.uid;
        let user = system
            .get_users()
            .iter()
            .find(|user| *user.get_uid() == user_id)
            .map(User::get_name);

        let inspect_output = format!(
            "{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n",
            Color::Cyan.paint("PID"),
            Color::White.paint(process.pid().to_string()),
            Color::Cyan.paint("Parent"),
            Color::White.paint(
                process
                    .parent()
                    .map(|parent| parent.to_string())
                    .unwrap_or_else(|| "".to_string())
            ),
            Color::Cyan.paint("User"),
            Color::White.paint(user.unwrap_or("")),
            Color::Cyan.paint("Name"),
            Color::White.paint(process.name()),
            Color::Cyan.paint("Command"),
            Color::White.paint(process.exe().to_str().unwrap_or("")),
            Color::Cyan.paint("CPU Usage"),
            Color::White.paint(cpu_usage),
            Color::Cyan.paint("Memory Usage"),
            Color::White.paint(memory_usage),
        );
        tw.write_all(inspect_output.as_bytes())?;

        Ok(())
    }
}
