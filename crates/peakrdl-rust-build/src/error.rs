//! Error types

use std::path::PathBuf;
use std::process::ExitStatus;

/// Errors that can occur when generating code with PeakRDL-rust.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// No input `.rdl` files were provided.
    #[error("no input .rdl files provided")]
    NoInputs,

    /// `$OUT_DIR` is not set and no output directory was provided.
    /// This usually means the crate is not being used from a build script.
    #[error(
        "no output directory provided and $OUT_DIR is not set; are you calling this from a build.rs?"
    )]
    NoOutDir,

    /// No top addrmap specified.
    #[error("no top-level addrmap specified")]
    NoTop,

    /// Could not determine a suitable cache directory for the generator binary.
    #[error(
        "could not determine a cache directory; set PEAKRDL_RUST_BINARY to a path to the generator binary"
    )]
    NoCacheDir,

    /// The current platform is not supported.
    #[error(
        "unsupported host platform: {host}; open an issue to request support: https://github.com/darsor/PeakRDL-rust/issues"
    )]
    UnsupportedPlatform { host: String },

    /// Failed to download the generator binary.
    #[error("failed to download generator binary from {url}: {reason}")]
    DownloadFailed { url: String, reason: String },

    /// The downloaded binary's checksum did not match the expected value.
    #[error("generator binary checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    /// Failed to spawn the generator binary.
    #[error("failed to spawn {}: {source}", binary.display())]
    SpawnFailed {
        binary: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// The generator binary exited with a non-zero status.
    #[error("peakrdl-rust exited with {status}\nstdout:\n{stdout}\nstderr:\n{stderr}")]
    GeneratorFailed {
        status: ExitStatus,
        stdout: String,
        stderr: String,
    },

    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
