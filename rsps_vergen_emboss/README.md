# RSPS Vergen Emboss

A simple macro to emboss vergen-based environment variables into your binary, primarily for rsps support.

## Usage

To get started, include both `vergen` and `rsps_vergen_emboss` in your `Cargo.toml`:

```toml
[package]
# <snip>
build = "build.rs"

[build-dependencies]
vergen = "9"

[dependencies]
rsps_vergen_emboss = "0.1.0"
```

Set up your `build.rs` to utilize `vergen`:

```rust
use vergen::{BuildBuilder, CargoBuilder, Emitter, RustcBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build = BuildBuilder::all_build()?;
    let cargo = CargoBuilder::all_cargo()?;
    let rustc = RustcBuilder::all_rustc()?;

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&rustc)?
        .emit()?;

    Ok(())
}
```

Finally, in your `main.rs`, import and call `rsps_vergen_emboss`:

```rust
use rsps_vergen_emboss::rsps_vergen_emboss;

// Includes every rsps compatible env var provided by vergen
rsps_vergen_emboss!(group=rsps);
```

If all went well, following a build, you should see some `vergen` attributes in the final binary:

```bash
$ strings target/debug/kana | grep VERGEN
VERGEN_RUSTC_CHANNEL=stable
VERGEN_RUSTC_COMMIT_DATE=2021-05-09
VERGEN_RUSTC_COMMIT_HASH=9bc8c42bb2f19e745a63f3445f1ac248fb015e53
VERGEN_RUSTC_HOST_TRIPLE=x86_64-apple-darwin
VERGEN_RUSTC_LLVM_VERSION=12.0
VERGEN_RUSTC_SEMVER=1.52.1
```

## Config

```rust
// Includes `VERGEN_BUILD_*`
rsps_vergen_emboss!(group=build);

// Includes `VERGEN_GIT_*`
rsps_vergen_emboss!(group=git);

// Includes `VERGEN_RUSTC_*`
rsps_vergen_emboss!(group=rustc);

// Includes `VERGEN_CARGO_*`
rsps_vergen_emboss!(group=cargo);

// Includes `VERGEN_SYSINFO_*`
rsps_vergen_emboss!(group=sysinfo);

// Includes both the rustc and cargo groups
rsps_vergen_emboss!(group=rust);

// Includes the following environment variables:
//  VERGEN_BUILD_TIMESTAMP
//  VERGEN_BUILD_SEMVER
//  VERGEN_RUSTC_SEMVER 
//  VERGEN_CARGO_PROFILE 
//  VERGEN_CARGO_FEATURES 
// Which rsps can use to display detailed information about your binary when it runs
rsps_vergen_emboss!(group=rsps);

// An alias for the above
rsps_vergen_emboss!();

// You can also specify multiple groups at once
// This will include both `VERGEN_SYSINFO_*` and `VERGEN_GIT_*`
rsps_vergen_emboss!(groups=sysinfo,git);
```
