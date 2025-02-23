use std::{collections::HashSet, convert::TryFrom, fs::OpenOptions, io::Read, path::PathBuf};

use anyhow::{Result, anyhow};
use lazy_static::lazy_static;
use object::{Object, ObjectSymbol};
use sysinfo::Process;

use crate::rsinfo::RsInfo;

lazy_static! {
    static ref RUST_MARKERS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("___rust_try");
        set.insert("__rust_try");
        set.insert("___rust_alloc");
        set.insert("__rust_alloc");
        set.insert("___rust_alloc_error_handler");
        set.insert("__rust_alloc_error_handler");
        set.insert("___rust_alloc_zeroed");
        set.insert("__rust_alloc_zeroed");
        set.insert("___rust_dealloc");
        set.insert("__rust_dealloc");
        set.insert("___rust_drop_panic");
        set.insert("__rust_drop_panic");
        set.insert("___rust_foreign_exception");
        set.insert("__rust_foreign_exception");
        set.insert("___rust_panic_cleanup");
        set.insert("__rust_panic_cleanup");
        set.insert("___rust_probestack");
        set.insert("__rust_probestack");
        set.insert("___rust_realloc");
        set.insert("__rust_realloc");
        set.insert("___rust_start_panic");
        set.insert("__rust_start_panic");
        set.insert("_rust_begin_unwind");
        set.insert("rust_begin_unwind");
        set.insert("_rust_eh_personality");
        set.insert("rust_eh_personality");
        set.insert("_rust_oom");
        set.insert("rust_oom");
        set.insert("_rust_panic");
        set.insert("rust_panic");
        set
    };
}

pub fn is_process_rusty(process: &Process) -> Result<Option<RsInfo>> {
    let Some(exe) = process.exe() else {
        return Ok(None);
    };

    let pwd = process
        .environ()
        .iter()
        .find(|env| env.to_string_lossy().starts_with("PWD="))
        .map(|pwd| {
            pwd.to_string_lossy()
                .strip_prefix("PWD=")
                .unwrap()
                .to_string()
        })
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("Process PWD doesn't seem to exist...?"))?;

    let mut file = OpenOptions::new().read(true).open(pwd.join(exe))?;

    let mut data = vec![];
    file.read_to_end(&mut data)?;

    let object_file = object::File::parse(&*data)?;
    let info = RsInfo::try_from(&object_file)?;
    if info.has_content() {
        return Ok(Some(info));
    }

    // Couldn't parse any metadata-- use fallback method of symbol detection
    let has_rust_symbol = object_file
        .symbols()
        .filter_map(|symbol| symbol.name().ok())
        .any(|symbol_name| RUST_MARKERS.contains(symbol_name));

    if has_rust_symbol {
        Ok(Some(info))
    } else {
        Ok(None)
    }
}
