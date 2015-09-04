# EnergyMon Rust Bindings

The `energymon-default-sys` crate provides declarations and linkage for the
`energymon-default` C library.
Following the *-sys package conventions, the `energymon-default-sys` crate does
not define higher-level abstractions over the native `energymon-default`
library functions.

The latest `EnergyMon` C libraries can be found at
[https://github.com/energymon/energymon](https://github.com/energymon/energymon).

## Dependencies

In order to use the `energymon-default-sys` crate, you must have the
`energymon-default` library installed to the system where it can be found by
`pkg-config`.

This crate depends on the `energymon-sys` crate.

## Usage
Add `energymon-default-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies.energymon-sys]
git = "https://github.com/energymon/energymon-default-sys.git"
```
