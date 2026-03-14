use anyhow::Result;
use peakrdl_rust_build::Generator;

fn main() -> Result<()> {
    Generator::new()
        .rdl_file("../../tests/rdl_src/basic.rdl")
        .top("basic")
        .format_output(true)
        .generate()?;
    Ok(())
}
