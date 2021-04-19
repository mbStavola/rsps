use anyhow::Result;
use sysinfo::System;
use tabwriter::TabWriter;

mod list;
mod tree;

#[cfg(target_os = "linux")]
mod stack;

pub use list::ListCommand;
#[cfg(target_os = "linux")]
pub use stack::StackCommand;
pub use tree::TreeCommand;

pub trait RspsSubcommand {
    fn exec(&self, system: &System, tw: &mut TabWriter<Vec<u8>>) -> Result<()>;
}
