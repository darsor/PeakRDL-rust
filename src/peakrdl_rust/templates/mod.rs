//! Control/Status Registers generated with PeakRDL-rust
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::identity_op)]
#![allow(clippy::inline_always)]
#![allow(clippy::let_and_return)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::unnecessary_cast)]

#[cfg(not(doctest))]
pub mod components;

{% for top_node in ctx.top_nodes %}
#[cfg(not(doctest))]
pub use {{top_node}};
{% endfor %}
