# peakrdl-rust-build

Build-script helper for generating Rust register-access code from SystemRDL files using [PeakRDL-rust](https://github.com/darsor/PeakRDL-rust).

## Usage

In your crate's `Cargo.toml`, add:
```ignore
[dependencies]
# the generated code implements traits defined in this crate
peakrdl-rust = "0.2.2"

[build-dependencies]
peakrdl-rust-build = { version = "0.7.3", features = ["download-bin"] }
```

In your `build.rs`:

```rust,ignore
peakrdl_rust_build::Generator::new()
    .rdl_file("src/regs/my_block.rdl")
    .top("my_block")
    .generate()
    .unwrap();
```

The `Generator` is a builder type for configuring the generated code. For all the options see the [documentation](https://docs.rs/peakrdl-rust-build).

Then in your `src/lib.rs` (or whichever module you want to include the generated code):

```rust,ignore
mod my_block {
    include!(concat!(env!("OUT_DIR"), "/my_block/mod.rs"));
}
```

Check the project documentation on [Read the Docs](https://peakrdl-rust.readthedocs.io/) to find examples of the generated code and how it can be used.

## How it Works

The `.generate()` function calls the [PeakRDL-rust](https://github.com/darsor/PeakRDL-rust) python tool to perform the actual code generation. With the `download-bin` feature enabled, the generator will download a peakrdl-rust binary (built using PyInstaller) from GitHub, verify its checksum, cache it, then execute it.

This behavior can be overridden in two ways:
1. Setting the `PEAKRDL_RUST_BINARY` environment variable to a local copy of the peakrdl-rust binary.
2. Disabling the `download-bin` feature of this crate to completely remove all download logic and dependencies, requiring `PEAKRDL_RUST_BINARY` to be set.

Note that the `PEAKRDL_RUST_BINARY` environment variable could also point to a wrapper script such as [this one](https://github.com/darsor/PeakRDL-rust/blob/main/scripts/uv_peakrdl_rust.sh) that uses `uv` to run PeakRDL-rust.

## Versions

If using the `PEAKRDL_RUST_BINARY` environment variable, the version numbers of this crate and the peakrdl-rust binary must match.

The generated code relies on the [peakrdl-rust](https://crates.io/crates/peakrdl-rust) crate on [crates.io](https://crates.io), which is versioned separately. The generated code has a compile-time check to ensure a compatible version of the `peakrdl-rust` dependency is used.

## License

This crate is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](https://www.apache.org/licenses/LICENSE-2.0))
 * MIT license ([LICENSE-MIT](https://opensource.org/license/mit))

at your option.

The python exporter is licensed under [LGPL-2.1](https://spdx.org/licenses/LGPL-2.1-or-later.html). See the [docs](https://peakrdl-rust.readthedocs.io/en/latest/licensing.html) for more information and FAQs.
