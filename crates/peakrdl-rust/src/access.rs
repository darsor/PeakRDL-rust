//! Marker types for read/write access

/// Marker trait for types representing read/write access
#[allow(private_bounds)]
pub trait Access: Sealed + Copy {}

/// Read-write register access token
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RW;

/// Read-only register access token
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct R;

/// Write-only register access token
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct W;

impl Access for R {}
impl Access for W {}
impl Access for RW {}

trait Sealed {}
impl Sealed for R {}
impl Sealed for W {}
impl Sealed for RW {}

/// Marker trait for read access
pub trait Read: Access {}
impl Read for RW {}
impl Read for R {}

/// Marker trait for write access
pub trait Write: Access {}
impl Write for RW {}
impl Write for W {}
