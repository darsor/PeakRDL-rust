//! Version constants for this crate

/// Major crate version
pub const MAJOR: u64 = {
    match u64::from_str_radix(env!("CARGO_PKG_VERSION_MAJOR"), 10) {
        Ok(major) => major,
        Err(_) => panic!("Failed to parse major version"),
    }
};

/// Minor crate version
pub const MINOR: u64 = {
    match u64::from_str_radix(env!("CARGO_PKG_VERSION_MINOR"), 10) {
        Ok(minor) => minor,
        Err(_) => panic!("Failed to parse minor version"),
    }
};

/// Patch crate version
pub const PATCH: u64 = {
    match u64::from_str_radix(env!("CARGO_PKG_VERSION_PATCH"), 10) {
        Ok(patch) => patch,
        Err(_) => panic!("Failed to parse patch version"),
    }
};
