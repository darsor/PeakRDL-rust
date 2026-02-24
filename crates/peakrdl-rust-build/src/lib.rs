//! Build-script helper for generating Rust register-access code from SystemRDL files
//! using [PeakRDL-rust](https://github.com/darsor/PeakRDL-rust).
//!
//! # Usage
//!
//! In your crate's `Cargo.toml`:
//! ```toml
//! [build-dependencies]
//! peakrdl-rust-build = "0.6"
//! ```
//!
//! In your `build.rs`:
//! ```rust,no_run
//! fn main() {
//!     peakrdl_rust_build::Generator::new()
//!         .file("src/regs/my_block.rdl")
//!         .top("my_block")
//!         .generate()
//!         .unwrap();
//! }
//! ```
//!
//! Then in your `src/lib.rs` (or wherever you want to use the generated code):
//! ```rust,ignore
//! include!(concat!(env!("OUT_DIR"), "/my_block/mod.rs"));
//! ```

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

mod binary;
mod error;

pub use error::Error;

/// Result type for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Byte or word endianness
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Endian {
    /// Big endian byte/word ordering
    Big,
    /// Little endian byte/word ordering
    Little,
}

/// Builder for configuring and running the PeakRDL-rust code generator.
#[derive(Debug)]
pub struct Generator {
    /// A PeakRDL configuration TOML file.
    config_file: Option<PathBuf>,
    /// Input `.rdl` files, in dependency order (dependencies first, top last).
    files: Vec<PathBuf>,
    /// Search directry for files included with \`include "filename"
    incdir: Option<PathBuf>,
    /// Pre-defined Verilog-style preprocessor macros
    macros: HashMap<String, Option<String>>,
    /// Top-level SystemRDL parameters
    parameters: HashMap<String, String>,
    /// Top-level addrmap name.
    top: Option<String>,
    /// Override the top-component's instantiated name. By default, the instantiated name is the same as
    /// the component's type name
    rename: Option<String>,
    /// Output directory. Defaults to `$OUT_DIR`. The generated module is placed in a subfolder with
    /// the name of the instantiated top addrmap.
    out_dir: Option<PathBuf>,
    /// Overwrite the generated module directory if it already exists. Defaults to true.
    force: bool,
    /// Format the generated rust code using `rustfmt`.
    fmt: bool,
    /// Ordering of bytes within `accesswidth`-sized accesses to the register file.
    /// By default uses the `littleendian` and `bigendian` addrmap properties,
    /// or little endian if not defined.
    byte_endian: Option<Endian>,
    /// Ordering of `accesswidth`-sized words within a wide register.
    /// By default uses the `littleendian` and `bigendian` addrmap properties,
    /// or little endian if not defined.
    word_endian: Option<Endian>,
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

impl Generator {
    /// Create a new, empty configuration.
    pub fn new() -> Self {
        Self {
            force: true,
            config_file: None,
            files: Vec::new(),
            incdir: None,
            macros: HashMap::new(),
            parameters: HashMap::new(),
            top: None,
            rename: None,
            out_dir: None,
            fmt: false,
            byte_endian: None,
            word_endian: None,
        }
    }

    /// Specify the path to a PeakRDL configuration TOML file.
    pub fn config_file(&mut self, path: impl AsRef<Path>) -> &mut Self {
        self.config_file = Some(path.as_ref().to_path_buf());
        self
    }

    /// Add a single input `.rdl` file.
    pub fn rdl_file(&mut self, path: impl AsRef<Path>) -> &mut Self {
        self.files.push(path.as_ref().to_path_buf());
        self
    }

    /// Add multiple input `.rdl` files (dependencies first, top-level last).
    pub fn rdl_files(&mut self, paths: impl IntoIterator<Item = impl AsRef<Path>>) -> &mut Self {
        self.files
            .extend(paths.into_iter().map(|p| p.as_ref().to_path_buf()));
        self
    }

    /// Search directry for files included with \`include "filename"
    pub fn include_dir(&mut self, dir: impl AsRef<Path>) -> &mut Self {
        self.incdir = Some(dir.as_ref().to_path_buf());
        self
    }

    /// Add a pre-defined a Verilog-style preprocessor macro.
    pub fn proprocessor_macro(
        &mut self,
        name: impl Into<String>,
        value: Option<impl Into<String>>,
    ) -> &mut Self {
        self.macros.insert(name.into(), value.map(|v| v.into()));
        self
    }

    /// Add a top-level SystemRDL parameter
    pub fn parameter(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.parameters.insert(name.into(), value.into());
        self
    }

    /// Override the top-level addrmap to elaborate. By default PeakRDL picks
    /// the last addrmap declared in the root namespace.
    pub fn top(&mut self, name: impl Into<String>) -> &mut Self {
        self.top = Some(name.into());
        self
    }

    /// Override the top-component's instantiated name. By default, the instantiated name is the same as
    /// the top component's type name
    pub fn rename(&mut self, name: impl Into<String>) -> &mut Self {
        self.rename = Some(name.into());
        self
    }

    /// Override the output directory. Defaults to `$OUT_DIR` set by Cargo.
    /// The generated module is placed in a subfolder with the name of the instantiated top addrmap.
    pub fn out_dir(&mut self, path: impl AsRef<Path>) -> &mut Self {
        self.out_dir = Some(path.as_ref().to_path_buf());
        self
    }

    /// By default, the output directory (a subdirectory within [`Self::out_dir`]) will be completely
    /// overwritten. Set this to false to only allow the generator to run if the output dir
    /// does not yet exist.
    pub fn overwrite_output(&mut self, overwrite: bool) -> &mut Self {
        self.force = overwrite;
        self
    }

    /// Set to `true` to format the generated rust code using `rustfmt`.
    pub fn format_output(&mut self, format: bool) -> &mut Self {
        self.fmt = format;
        self
    }

    /// Set the ordering of bytes within `accesswidth`-sized accesses to the register file.
    /// By default uses the `littleendian` and `bigendian` addrmap properties,
    /// or little endian if not defined.
    pub fn byte_endian(&mut self, endian: Endian) -> &mut Self {
        self.byte_endian = Some(endian);
        self
    }

    /// Set the ordering of `accesswidth`-sized words within a wide register.
    /// By default uses the `littleendian` and `bigendian` addrmap properties,
    /// or little endian if not defined.
    pub fn word_endian(&mut self, endian: Endian) -> &mut Self {
        self.word_endian = Some(endian);
        self
    }

    /// Run the code generator.
    ///
    /// This will:
    /// 1. Locate (or download) the `peakrdl-rust` binary.
    /// 2. Emit `cargo:rerun-if-changed` directives for all input files.
    /// 3. Invoke `peakrdl rust` with the specified options.
    pub fn generate(&self) -> Result<()> {
        if self.files.is_empty() {
            return Err(Error::NoInputs);
        }

        let Some(top) = &self.top else {
            return Err(Error::NoTop);
        };

        let out_dir = match &self.out_dir {
            Some(p) => p.clone(),
            None => PathBuf::from(std::env::var("OUT_DIR").map_err(|_| Error::NoOutDir)?),
        };

        // Tell Cargo to re-run this build script if any input file changes.
        for file in &self.files {
            println!("cargo:rerun-if-changed={}", file.display());
        }
        if let Some(config_file) = &self.config_file {
            println!("cargo:rerun-if-changed={}", config_file.display());
        }
        if let Some(include_dir) = &self.incdir {
            println!("cargo:rerun-if-changed={}", include_dir.display());
        }

        // Also re-run if the user overrides the binary path.
        println!("cargo:rerun-if-env-changed=PEAKRDL_RUST_BINARY");

        let generator = binary::resolve_generator_binary()?;
        println!("cargo:rerun-if-changed={}", generator.display());

        let mut cmd = Command::new(&generator);

        // config file
        if let Some(config_file) = &self.config_file {
            cmd.args(["--peakrdl-cfg", config_file.to_str().unwrap()]);
        }

        // include file
        if let Some(include_dir) = &self.incdir {
            cmd.args(["-I", include_dir.to_str().unwrap()]);
        }

        // preprocessor macros
        for (name, value) in self.macros.iter() {
            cmd.arg("-D");
            match value {
                Some(value) => cmd.arg(format!("{name}={value}")),
                None => cmd.arg(name),
            };
        }

        // top
        cmd.args(["--top", top]);

        // rename
        if let Some(rename) = &self.rename {
            cmd.args(["--rename", rename]);
        }

        // parameters
        for (name, value) in self.parameters.iter() {
            cmd.args(["-D", &format!("{name}={value}")]);
        }

        // output directory
        let top_name = self.rename.as_ref().unwrap_or(top);
        let top_dir = out_dir.join(top_name);
        cmd.args(["-o", top_dir.to_str().unwrap()]);

        // force
        if self.force {
            cmd.arg("--force");
        }

        // format
        if self.fmt {
            cmd.arg("--fmt");
        }

        // endianness
        match self.byte_endian {
            Some(Endian::Big) => {
                cmd.args(["--byte-endian", "big"]);
            }
            Some(Endian::Little) => {
                cmd.args(["--byte-endian", "little"]);
            }
            _ => (),
        };
        match self.word_endian {
            Some(Endian::Big) => {
                cmd.args(["--word-endian", "big"]);
            }
            Some(Endian::Little) => {
                cmd.args(["--word-endian", "little"]);
            }
            _ => (),
        };

        for file in &self.files {
            cmd.arg(file);
        }

        let output = cmd.output().map_err(|e| Error::SpawnFailed {
            binary: generator.clone(),
            source: e,
        })?;

        if !output.status.success() {
            return Err(Error::GeneratorFailed {
                status: output.status,
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
                stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            });
        }

        eprintln!(
            "cargo:warning=Generated {top_name} code in {}",
            top_dir.display()
        );

        Ok(())
    }
}
