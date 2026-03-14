#[allow(clippy::cast_lossless)]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::derivable_impls)]
#[allow(clippy::doc_markdown)]
#[allow(clippy::identity_op)]
#[allow(clippy::inline_always)]
#[allow(clippy::let_and_return)]
#[allow(clippy::trivially_copy_pass_by_ref)]
#[allow(clippy::unnecessary_cast)]
#[cfg(not(doctest))]
pub mod components;

{% for top_node in ctx.top_nodes %}
#[cfg(not(doctest))]
pub use {{top_node}};
{% endfor %}

/// Version of PeakRDL-rust used to generate this code
pub const PEAKRDL_RUST_VERSION: &str = "{{ctx.peakrdl_rust_version}}";

// Compile-time version check
#[allow(unused_comparisons)]
#[allow(clippy::absurd_extreme_comparisons)]
const _VERSION_CHECK: () = {
    use peakrdl_rust::version;
    const ERR_MSG: &str = "peakrdl-rust dependency must be >={{ctx.crate_min_version|join('.')}}, <{{ctx.crate_max_version|join('.')}}. Please update your Cargo.toml.";
    assert!(version::MAJOR == {{ctx.crate_min_version[0]}}, "{}", ERR_MSG);
{% if ctx.crate_min_version[0] == 0 %}
    assert!(version::MINOR == {{ctx.crate_min_version[1]}}, "{}", ERR_MSG);
{% else %}
    assert!(version::MINOR >= {{ctx.crate_min_version[1]}}, "{}", ERR_MSG);
{% endif %}
    assert!(version::PATCH >= {{ctx.crate_min_version[2]}}, "{}", ERR_MSG);
};
