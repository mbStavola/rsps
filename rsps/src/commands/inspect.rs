use std::io::Write;

use ansi_term::Color;
use anyhow::{Result, anyhow};
use byte_unit::{Byte, Unit, UnitType};
use clap::Args;
use sysinfo::{ProcessesToUpdate, System, User, Users};
use tabwriter::TabWriter;

use crate::commands::{ProcessArg, RspsSubcommand};

#[derive(Debug, Args)]
pub struct InspectCommand {
    process: ProcessArg,
}

impl RspsSubcommand for InspectCommand {
    fn exec(
        &self,
        system: &mut System,
        users: &mut Users,
        tw: &mut TabWriter<Vec<u8>>,
    ) -> Result<()> {
        // Quickly refresh a few times to get a nice CPU usage sample
        system.refresh_processes(ProcessesToUpdate::All, false);
        let (pid, info) = {
            let (process, info) = self.process.as_system_process(system, tw)?;
            (process.pid(), info)
        };
        system.refresh_processes(ProcessesToUpdate::Some(&[pid]), false);

        let process = system
            .process(pid)
            .ok_or_else(|| anyhow!("Process disappeared since last sample"))?;

        let cpu_usage = format!("{:.2}%", process.cpu_usage());

        let memory_utilization = process.memory() as f32 / system.available_memory() as f32;
        let memory_usage = format!(
            "{:.2} ({:.2}%)",
            Byte::from_u64_with_unit(process.memory(), Unit::B)
                .map(|unit| unit.get_appropriate_unit(UnitType::Binary))
                .ok_or_else(|| anyhow!("process memory too large"))?,
            memory_utilization * 100.0
        );

        let user = process
            .user_id()
            .and_then(|uid| users.get_user_by_id(uid))
            .map(User::name);

        let sysinfo_output = format!(
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
            Color::White.paint(process.name().to_string_lossy()),
            Color::Cyan.paint("Command"),
            Color::White.paint(process.exe().and_then(|exe| exe.to_str()).unwrap_or("")),
            Color::Cyan.paint("CPU Usage"),
            Color::White.paint(cpu_usage),
            Color::Cyan.paint("Memory Usage"),
            Color::White.paint(memory_usage),
        );
        tw.write_all(sysinfo_output.as_bytes())?;

        if info.has_content() {
            tw.write_all("\n".as_bytes())?;
            let emboss_output = format!(
                "{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n",
                Color::Cyan.paint("Rust Version"),
                Color::White.paint(
                    info.rust_version()
                        .map(String::as_str)
                        .unwrap_or("<unknown>")
                ),
                Color::Cyan.paint("Program Version"),
                Color::White.paint(
                    info.program_version()
                        .map(String::as_str)
                        .unwrap_or("<unknown>")
                ),
                Color::Cyan.paint("Cargo Build Profile"),
                Color::White.paint(
                    info.build_profile()
                        .map(String::as_str)
                        .unwrap_or("<unknown>")
                ),
                Color::Cyan.paint("Cargo Features"),
                Color::White.paint(
                    info.cargo_features()
                        .map(String::as_str)
                        .unwrap_or("<unknown>")
                ),
                Color::Cyan.paint("Build Timestamp"),
                Color::White.paint(info.build_time().map(String::as_str).unwrap_or("<unknown>")),
            );
            tw.write_all(emboss_output.as_bytes())?;
        }

        Ok(())
    }
}
