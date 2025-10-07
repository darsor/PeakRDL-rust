#![no_std]

#[cfg(not(doctest))]
pub mod components;
{% if ctx.has_fixedpoint %}
pub mod fixedpoint;
{% endif %}
pub mod mem;
pub mod reg;

// TODO: pub use addrmap
// TODO: pub const addrmap
