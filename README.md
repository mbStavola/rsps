# rsps

A command line tool to help find and debug Rust programs on your machine. Inspired by [gops][1] and a fit of boredom.

This only works if the binaries haven't be stripped beforehand.

# Installation

```bash
$ cargo install rsps
```

# Usage

## List (default)

Lists all running Rust processes.

```bash
$ rsps # or rsps list
PID    Parent  Name  Path
58988  54401   rg    /usr/local/bin/rg
58989  47182   rsps  target/debug/rsps
```

## Tree

Displays Rust processes in a tree-like format.

```bash
$ rsps tree
@
├── 59235  [treem]
│          └── 59236  [branch]
│                     ├── 59238  [leaf]
│                     ├── 59237  [branch]
│                     │          ├── 59240  [branch]
│                     │          │          ├── 59244  [leaf]
│                     │          │          ├── 59242  [branch]
│                     │          │          │          └── 59245  [leaf]
│                     │          │          └── 59243  [branch]
│                     │          │                     └── 59246  [leaf]
│                     │          └── 59241  [leaf]
│                     └── 59239  [leaf]
└── 59262  [rsps]
```

## Inspect

Display detailed information about a specific Rust process.

```bash
$ rsps inspect cargo # or rsps inspect <pid>
PID: 78632
Parent: 78274
User: matt
Name: cargo
Command: /Users/matt/.rustup/toolchains/stable-x86_64-apple-darwin/bin/cargo
CPU Usage: 6.57%
Memory Usage: 24.66 MiB (1.11%)
```

## Stack (Linux only)

Dumps the stack of a running Rust process, given the name or pid.

```bash
$ rsps stack cargo # or rsps stack <pid>
Thread ID: 279  Name: cargo
Stack Dump:
Frame #0: 7fe294f5e100   pthread_cond_timedwait
Frame #1: 564691cee080   std::sys::unix::condvar::Condvar::wait_timeout::h37a8048107691687
Frame #2: 5646915f9fd0   std::sync::condvar::Condvar::wait_timeout_while::hbfb743bd05dd9fc0
Frame #3: 564691415440   cargo::util::queue::Queue<T>::pop::h7fb70103b44dc37f
Frame #4: 56469154fbd0   cargo::core::compiler::job_queue::DrainState::drain_the_queue::h5b61c7275f6c30ad
Frame #5: 564691504160   std::panic::catch_unwind::h7e0c92cfb502709b
Frame #6: 5646914af4c0   crossbeam_utils::thread::scope::haf6d0e4c1aacb903
Frame #7: 56469154ec30   cargo::core::compiler::job_queue::JobQueue::execute::heaab11ff0a2fe80c
Frame #8: 564691437360   cargo::core::compiler::context::Context::compile::h35d7a4cf82b1b826
Frame #9: 564691697d00   cargo::ops::cargo_compile::compile_ws::hd140871262d59407
Frame #10: 56469147c350  cargo::ops::cargo_install::install_one::h63708159c342d27f
Frame #11: 56469147af40  cargo::ops::cargo_install::install::h101310e8cf160103
Frame #12: 564691365c20  cargo::commands::install::exec::h064cce367657894c
Frame #13: 564691358d60  cargo::cli::main::h71faf507b10707d7
Frame #14: 5646913c11d0  cargo::main::h6746203463cb9d36
Frame #15: 5646913b29c0  std::sys_common::backtrace::__rust_begin_short_backtrace::h715a397fa07175af
Frame #16: 5646913b29e0  std::rt::lang_start::{{closure}}::h3a8e3bf998c29384
Frame #17: 564691cebc10  std::rt::lang_start_internal::hd5b67df56ca01dae
Frame #18: 5646913c3580  main
Frame #19: 7fe294c2afb0  __libc_start_main
Frame #20: <no symbol>

Thread ID: 297  Name: cargo
Stack Dump:
Frame #0: 7fe294cf57d0  __poll
Frame #1: 564691cacb60  jobserver::imp::Client::acquire_allow_interrupts::h4f87d446882f6e88
Frame #2: 564691cae370  jobserver::HelperState::for_each_request::h80f41bf960986b48
Frame #3: 564691cae680  std::sys_common::backtrace::__rust_begin_short_backtrace::hcb30739b281791a0
Frame #4: 564691caf0e0  core::ops::function::FnOnce::call_once{{vtable.shim}}::h6fcaf617e71843a9
Frame #5: 564691cf5210  std::sys::unix::thread::Thread::new::thread_start::hb5e40d3d934ebb7a
Frame #6: 7fe294f57eb0  start_thread
Frame #7: <no symbol>

[...]
```

Currently limited to Linux only due to [rstack][2] only building on Linux.

# License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE][3] or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT][4] or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[1]: https://github.com/google/gops
[2]: https://github.com/sfackler/rstack
[3]: ./LICENSE-APACHE
[4]: ./LICENSE-MIT
