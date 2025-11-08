//! Marker types for read/write access

trait Sealed {}

#[allow(private_bounds)]
pub trait Access: Sealed + Copy {}

/// Read-write register access token
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RW;

/// Read-only register access token
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct R;

/// Write-only register access token
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct W;

impl Access for R {}
impl Access for W {}
impl Access for RW {}

impl Sealed for R {}
impl Sealed for W {}
impl Sealed for RW {}

pub trait Read: Access {}
impl Read for RW {}
impl Read for R {}

pub trait Write: Access {}
impl Write for RW {}
impl Write for W {}
