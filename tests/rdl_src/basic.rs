use basic::Basic;
use peakrdl_rust::io::MockIO;

const SIZE: usize = Basic::<()>::SIZE;

#[allow(static_mut_refs)]
#[test]
fn test_basic_access() {
    let memory: MockIO<SIZE> = MockIO::new_zeroed();
    let top = unsafe { Basic::from_ptr_with(memory.base_ptr(), &memory) };

    top.basicreg_b().write(|reg| {
        reg.set_basicfield_c(12345);
    });
    assert_eq!(top.basicreg_b().read().basicfield_c(), 12345);

    // test write
    top.basicreg_e().write(|reg| {
        reg.set_basicfield_h(1);
        reg.set_basicfield_i(0x55);
        reg.set_basicfield_j(0xAA);
        reg.set_basicfield_k(255);
    });
    // test read
    let basicreg_e = top.basicreg_e().read();
    assert_eq!(basicreg_e.basicfield_h(), 1);
    assert_eq!(basicreg_e.basicfield_i(), 0x55);
    assert_eq!(basicreg_e.basicfield_j(), 0xAA);
    assert_eq!(basicreg_e.basicfield_k(), 255);
    // test modify
    top.basicreg_e().modify(|reg| {
        reg.set_basicfield_j(0x1C);
    });
    let basicreg_e = top.basicreg_e().read();
    assert_eq!(basicreg_e.basicfield_h(), 1);
    assert_eq!(basicreg_e.basicfield_i(), 0x55);
    assert_eq!(basicreg_e.basicfield_j(), 0x1C);
    assert_eq!(basicreg_e.basicfield_k(), 255);

    // test single-bit access
    for val in [true, false, true] {
        top.basicreg_g().write(|reg| {
            reg.set_basicfield_s(val);
        });
        assert_eq!(top.basicreg_g().read().basicfield_s(), val);
    }
}
