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
$ rsps
PID    Parent  Name  Path
58988  54401   rg    /usr/local/bin/rg
58989  47182   rsps  target/debug/rsps

$ rsps list
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

## Stack (Linux only)

Dumps the stack of a running Rust process, given the pid.

```bash
$ rsps stack 1337
[...]
Thread ID: 23145  Name: cargo
Stack Dump:
Frame #0: 7f1576df4330   __poll
Frame #1: 555fafbf8f10   _ZN5cargo4util5read23imp5read217h5dfd4a45566d9e2bE
Frame #2: 555fafcbc9f0   _ZN5cargo4util15process_builder14ProcessBuilder19exec_with_streaming17h1e969eb74acfee45E
Frame #3: 555fafc87270   _ZN90_$LT$cargo..core..compiler..DefaultExecutor$u20$as$u20$cargo..core..compiler..Executor$GT$4exec17h9462582fab4f4a44E
Frame #4: 555faf9a1ee0   _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h6f9755c689f644bbE.llvm.4873155011723304831
Frame #5: 555faf9a01f0   _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h2ad050d4d2f5bbe4E.llvm.4873155011723304831
Frame #6: 555faf9a01f0   _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h2ad050d4d2f5bbe4E.llvm.4873155011723304831
Frame #7: 555fafa24e20   _ZN15crossbeam_utils6thread19ScopedThreadBuilder5spawn28_$u7b$$u7b$closure$u7d$$u7d$17h22c03f3f1af100c2E
Frame #8: 555fafa69a00   _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hee1957dee06f64dcE
Frame #9: 555fafba1780   _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h2b02cb2d1923376dE
Frame #10: 555fb0285550  _ZN3std3sys4unix6thread6Thread3new12thread_start17h4afaeade0da13617E
Frame #11: 7f1576b981b0  start_thread
Frame #12: <no symbol>
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
