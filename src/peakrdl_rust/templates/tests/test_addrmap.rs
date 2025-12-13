use {{ctx.crate_name}}::components::{{ctx.name|kw_filter}};

/// Test all generated component addresses against the SystemRDL assigned address
#[test]
fn test_{{ctx.name}}_addresses() {
    const SIZE: usize = {{ctx.name|kw_filter}}::{{ctx.type_name}}::SIZE;
    let mut memory = [0u8; SIZE];
    let base_addr = memory.as_mut_ptr();
    let dut = unsafe { {{ctx.name|kw_filter}}::{{ctx.type_name}}::from_ptr(base_addr as _) };

    assert_eq!(dut.as_ptr() as *mut u8, base_addr);
    {% for address in ctx.addresses %}
    assert_eq!(dut.{{address.dut_method}}.as_ptr() as *mut u8, base_addr.wrapping_byte_add({{"0x{:_X}".format(address.absolute_addr)}}));
    {% endfor %}
}
