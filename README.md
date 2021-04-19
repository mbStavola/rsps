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
```

Currently limited to Linux only due to [rstack][4] only building on Linux.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE][3] or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT][4] or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[1]: https://github.com/google/gops
[2]: https://github.com/sfackler/rstack
[3]: ./LICENSE-MIT
[4]: ./LICENSE-APACHE