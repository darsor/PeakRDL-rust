#![no_std]
#![allow(clippy::derivable_impls)]
#![allow(clippy::identity_op)]
#![allow(clippy::let_and_return)]
#![allow(clippy::unnecessary_cast)]

pub mod access;
#[cfg(not(doctest))]
pub mod components;
pub mod encode;
{% if ctx.has_fixedpoint %}
pub mod fixedpoint;
{% endif %}
pub mod mem;
pub mod reg;
