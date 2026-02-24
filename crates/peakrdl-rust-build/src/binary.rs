//! Locates or downloads the `peakrdl` generator binary.
//!
//! Resolution order:
//! 1. `PEAKRDL_RUST_BINARY` env var — lets users point at a local build or
//!    air-gapped install.
//! 2. The per-version cache at `<cache_dir>/peakrdl-rust/<version>/peakrdl[.exe]`.
//! 3. Download from GitHub Releases, verify checksum, cache, then use.

use std::path::PathBuf;

use crate::{Error, Result};

pub(crate) fn resolve_generator_binary() -> Result<PathBuf> {
    // 1. Explicit override from environment.
    if let Ok(path) = std::env::var("PEAKRDL_RUST_BINARY") {
        let p = PathBuf::from(path);
        if p.exists() {
            return Ok(p);
        } else {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "PEAKRDL_RUST_BINARY is set to '{}' but that path does not exist",
                    p.display()
                ),
            )));
        }
    }

    #[cfg(not(feature = "download-bin"))]
    {
        return Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "the `download-bin` feature is disabled but PEAKRDL_RUST_BINARY is not set",
        )));
    }
    #[cfg(feature = "download-bin")]
    {
        // 2. Check cache.
        let (asset_name, exe_name) = download::platform_info()?;
        let cache_path = download::cache_binary_path(&exe_name)?;

        if cache_path.exists() {
            return Ok(cache_path);
        }

        // 3. Download, verify, extract, and cache.
        download::download_binary(asset_name, &exe_name, &cache_path)?;
        Ok(cache_path)
    }
}

#[cfg(feature = "download-bin")]
pub(crate) mod download {
    use crate::{Error, Result};
    use std::io::{Read as _, Write};
    use std::path::PathBuf;

    /// GitHub Releases base URL.
    const GITHUB_RELEASES_BASE: &str = "https://github.com/darsor/PeakRDL-rust/releases/download";

    /// Per-platform binary info: (target_os, target_arch, asset_filename).
    const PLATFORM_ASSETS: &[(&str, &str, &str)] = &[
        ("linux", "x86_64", "peakrdl-rust-linux-x86_64.tar.gz"),
        ("linux", "aarch64", "peakrdl-rust-linux-aarch64.tar.gz"),
        ("macos", "x86_64", "peakrdl-rust-darwin-x86_64.tar.gz"),
        ("macos", "aarch64", "peakrdl-rust-darwin-aarch64.tar.gz"),
        ("windows", "x86_64", "peakrdl-rust-windows-x86_64.tar.gz"),
    ];

    /// The version of PeakRDL-rust whose binary we bundle/download.
    const fn peakrdl_rust_version() -> &'static str {
        // peakrdl-rust-build package version kept in sync with python version
        env!("CARGO_PKG_VERSION")
    }

    pub(crate) fn platform_info() -> Result<(&'static str, &'static str)> {
        let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
        let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

        for &(p_os, p_arch, asset) in PLATFORM_ASSETS {
            if os == p_os && arch == p_arch {
                let exe = if os == "windows" {
                    "peakrdl.exe"
                } else {
                    "peakrdl"
                };
                return Ok((asset, exe));
            }
        }

        Err(Error::UnsupportedPlatform { os, arch })
    }

    pub(crate) fn cache_binary_path(exe_name: &str) -> Result<PathBuf> {
        let dir = dirs::cache_dir()
            .ok_or(Error::NoCacheDir)?
            .join("peakrdl-rust-build")
            .join(peakrdl_rust_version());

        std::fs::create_dir_all(&dir)?;
        Ok(dir.join(exe_name))
    }

    pub(crate) fn download_binary(asset_name: &str, exe_name: &str, dest: &PathBuf) -> Result<()> {
        let expected_sha256 = fetch_checksum(asset_name)?;

        let url = format!(
            "{}/v{}/{}",
            GITHUB_RELEASES_BASE,
            peakrdl_rust_version(),
            asset_name
        );

        eprintln!("cargo:warning=Downloading peakrdl-rust binary from {url}");

        let response = ureq::get(&url).call().map_err(|e| Error::DownloadFailed {
            url: url.clone(),
            reason: e.to_string(),
        })?;

        let mut archive_bytes: Vec<u8> = Vec::new();
        response
            .into_body()
            .into_reader()
            .read_to_end(&mut archive_bytes)
            .map_err(|e| Error::DownloadFailed {
                url: url.clone(),
                reason: e.to_string(),
            })?;

        // Verify the checksum of the archive before extracting.
        use sha2::Digest as _;
        let actual_sha256 = hex::encode(sha2::Sha256::digest(&archive_bytes));
        if actual_sha256 != expected_sha256 {
            return Err(Error::ChecksumMismatch {
                expected: expected_sha256,
                actual: actual_sha256,
            });
        }

        use flate2::read::GzDecoder;
        use tar::Archive;

        // Extract the binary from the tar.gz.
        let cursor = std::io::Cursor::new(archive_bytes);
        let mut archive = Archive::new(GzDecoder::new(cursor));

        let mut binary_data: Option<Vec<u8>> = None;
        for entry in archive.entries().map_err(Error::Io)? {
            let mut entry = entry.map_err(Error::Io)?;
            let path = entry.path().map_err(Error::Io)?;
            if path.file_name().map(|n| n == exe_name).unwrap_or(false) {
                let mut data = Vec::new();
                std::io::Read::read_to_end(&mut entry, &mut data)?;
                binary_data = Some(data);
                break;
            }
        }

        let binary_data = binary_data.ok_or_else(|| {
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("'{exe_name}' not found in downloaded archive"),
            ))
        })?;

        // Write to a temp file first, then rename for atomic replacement.
        let tmp = dest.with_extension("tmp");
        {
            let mut f = std::fs::File::create(&tmp)?;
            f.write_all(&binary_data)?;
        }

        // Make executable on Unix.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&tmp)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&tmp, perms)?;
        }

        std::fs::rename(&tmp, dest)?;
        Ok(())
    }

    fn fetch_checksum(asset_name: &str) -> Result<String> {
        let url = format!(
            "{}/v{}/{}.sha256",
            GITHUB_RELEASES_BASE,
            peakrdl_rust_version(),
            asset_name
        );

        let response = ureq::get(&url).call().map_err(|e| Error::DownloadFailed {
            url: url.clone(),
            reason: e.to_string(),
        })?;

        let body = response
            .into_body()
            .read_to_string()
            .map_err(|e| Error::DownloadFailed {
                url: url.clone(),
                reason: e.to_string(),
            })?;

        // The file is in `sha256sum` format: "<hex>  <filename>\n"
        // We only want the hex portion.
        body.split_whitespace()
            .next()
            .ok_or_else(|| Error::DownloadFailed {
                url,
                reason: "checksum file was empty or malformed".to_string(),
            })
            .map(str::to_string)
    }
}
