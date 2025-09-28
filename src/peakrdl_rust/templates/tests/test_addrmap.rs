use {{ctx.crate_name}}::components::{{ctx.name}};

/// Test all generated component addresses against the SystemRDL assigned address
#[test]
fn test_{{ctx.name}}_addresses() {
    const SIZE: usize = {{ctx.name}}::{{ctx.type_name}}::SIZE;
    let mut memory = [0u8; SIZE];
    let base_addr = memory.as_mut_ptr();
    let dut = unsafe { {{ctx.name}}::{{ctx.type_name}}::from_ptr(base_addr as _) };

    // SAFETY: we're using unsafe pointer arithmetic, but never deference the
    // resulting pointer and should never go outside the bounds of the memory
    // allocation.
    unsafe {
        assert_eq!(dut.as_ptr() as *mut u8, base_addr);
        {% for address in ctx.addresses %}
        assert_eq!(dut.{{address.dut_method}}.as_ptr() as *mut u8, base_addr.byte_add(0x{{"%x" % address.absolute_addr}}));
        {% endfor %}
    }
}
