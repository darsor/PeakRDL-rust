use anyhow::Result;
use peakrdl_rust_build::Generator;

fn main() -> Result<()> {
    unsafe {
        std::env::set_var(
            "PEAKRDL_RUST_BINARY",
            "../../../scripts/uv_local_peakrdl_rust.sh",
        )
    };
    Generator::new()
        .rdl_file("../../../src/peakrdl_rust/udps/udps.rdl")
        .rdl_file("../../../tests/rdl_src/turboencabulator.rdl")
        .top("turbo_encab")
        .format_output(true)
        .generate()?;
    Ok(())
}
