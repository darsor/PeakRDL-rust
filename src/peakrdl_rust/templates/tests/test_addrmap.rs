use bitfield_access::BitfieldAccess;
use TODO::components::{{ctx.name}};

mod memory;

/// Test all generated component addresses against the SystemRDL assigned address
#[test]
fn test_{{ctx.name}}_addresses() {
    const SIZE: usize = {{ctx.name}}::{{ctx.type_name}}::SIZE;
    let mut memory = memory::Memory::<SIZE>::new_zeroed();
    let base_addr = memory.as_mut_ptr();
    // SAFETY: this produces aliased mutable memory to simulate the hardware
    // mutability of actual hardware registers. All accesses through the DUT use
    // volatile reads/writes.
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

#[test]
fn test_{{ctx.name}}() {
    const SIZE: usize = {{ctx.name}}::{{ctx.type_name}}::SIZE;
    let mut memory = memory::Memory::<SIZE>::new_zeroed();
    // SAFETY: this produces aliased mutable memory to simulate the hardware
    // mutability of actual hardware registers. All accesses through the DUT use
    // volatile reads/writes.
    let dut = unsafe { {{ctx.name}}::{{ctx.type_name}}::from_ptr(memory.as_mut_ptr() as _) };

    {% for field in ctx.fields %}
    let field_range = memory::lsb0_to_msb0({{field.bit_offset + field.width - 1}}..={{field.bit_offset}}, {{field.reg_width}});
        {% for pattern in field.test_patterns %}
    // Test {{field.name}} with pattern: {{pattern.description}}
    {% if pattern.raw_write %}
    memory.at_mut(0x{{"%X" % field.address}}).write_field::<{{field.primitive}}>(field_range.clone(), {{pattern.raw_value}});
    {% elif "R" in field.reg_access %}
    dut.{{field.reg_method}}.modify(|r| {
        r.set_{{field.name}}({{pattern.value}});
    });
    {% else %}
    dut.{{field.reg_method}}.write(|r| {
        r.set_{{field.name}}({{pattern.value}});
    });
    {% endif %}
    {% if field.is_readable %}
        {% if field.has_encoding %}
    assert_eq!(
        dut.{{field.reg_method}}.read().{{field.name}}(),
        {% if pattern.is_valid_enum %}
        Some({{pattern.value}})
        {% else %}
        None
        {% endif %}
    );
        {% else %}
    assert_eq!(dut.{{field.reg_method}}.read().{{field.name}}(), {{pattern.value}});
        {% endif %}
    {% endif %}
    let mem_val = memory.at(0x{{"%X" % field.address}}).read_field::<{{field.primitive}}>(field_range.clone());
    assert_eq!(mem_val, {{pattern.raw_value}});

        {% endfor %}
    {% endfor %}
}
