# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

All commands run from the `learn_rust/` directory.

```bash
cargo run                    # build and run main binary (src/main.rs)
cargo run --bin otherexec    # build and run the secondary binary
cargo build                  # compile only
cargo test                   # run tests (tests live in src/lib.rs)
cargo clean                  # remove target/ and compiled artifacts
```

## Project Structure

This is a Rust learning project using edition 2024. The crate is both a library and a binary.

- `learn_rust/src/main.rs` — main executable; imports `lib_one` via `mod lib_one` and uses its functions
- `learn_rust/src/lib.rs` — library crate root; re-exports `lib_one` as a public module so it's visible to external binaries
- `learn_rust/src/lib_one.rs` — shared library functions (`lib_test`, `null_out`, `null_out_ind`, `mymove`, `donothing`)
- `learn_rust/src/bin/otherexec.rs` — secondary binary; accesses the lib crate via `learn_rust::lib_one`
- `nvimnotes.txt` — NeoVim reference notes (not Rust code)

## Module Visibility Pattern

`lib_one` must be declared in **both** `main.rs` (as `mod lib_one`) and `lib.rs` (as `pub mod lib_one`) so it is accessible from both the binary crate and external binaries like `otherexec`.

Each additional binary in `src/bin/` requires a corresponding `[[bin]]` entry in `Cargo.toml`.
