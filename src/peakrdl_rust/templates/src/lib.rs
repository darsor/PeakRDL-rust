#![no_std]

pub mod access;
#[cfg(not(doctest))]
pub mod components;
pub mod encode;
{% if ctx.has_fixedpoint %}
pub mod fixedpoint;
{% endif %}
pub mod mem;
pub mod reg;

// TODO: pub use addrmap
// TODO: pub const addrmap
