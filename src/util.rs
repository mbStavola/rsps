use std::{collections::HashSet, fs::OpenOptions, io::Read, path::PathBuf};

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use object::{Object, ObjectSymbol};
use sysinfo::{Process, ProcessExt};

lazy_static! {
    static ref RUST_MARKERS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("___rust_try");
        set.insert("___rust_alloc");
        set.insert("___rust_alloc_error_handler");
        set.insert("___rust_alloc_zeroed");
        set.insert("___rust_dealloc");
        set.insert("___rust_drop_panic");
        set.insert("___rust_foreign_exception");
        set.insert("___rust_panic_cleanup");
        set.insert("___rust_probestack");
        set.insert("___rust_realloc");
        set.insert("___rust_start_panic");
        set.insert("_rust_begin_unwind");
        set.insert("_rust_eh_personality");
        set.insert("_rust_oom");
        set.insert("_rust_panic");
        set
    };
}

pub fn is_process_rusty(process: &Process) -> Result<bool> {
    let pwd = process
        .environ()
        .iter()
        .find(|env| env.starts_with("PWD="))
        .map(|pwd| pwd.strip_prefix("PWD=").unwrap())
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("Process PWD doesn't seem to exist...?"))?;

    let mut file = OpenOptions::new()
        .read(true)
        .open(pwd.join(process.exe()))?;

    let mut data = vec![];
    file.read_to_end(&mut data)?;

    let object_file = object::File::parse(&data)?;

    let has_rust_symbol = object_file
        .symbols()
        .filter_map(|symbol| symbol.name().ok())
        .any(|symbol_name| RUST_MARKERS.contains(symbol_name));

    Ok(has_rust_symbol)
}
