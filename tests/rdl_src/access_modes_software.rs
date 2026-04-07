use access_modes_software::AccessModesTest;
use peakrdl_rust::io::MockIO;

const SIZE: usize = AccessModesTest::<()>::SIZE;

#[test]
fn test_sw_access_mode() {
    let memory: MockIO<SIZE> = MockIO::new_zeroed();
    let top = unsafe { AccessModesTest::from_ptr_with(memory.base_ptr(), &memory) };

    // write default
    top.sw_reg().write(|_| {});

    // test software read/write
    assert_eq!(top.sw_reg().read().sw_rw(), 0x00);
    top.sw_reg().write(|reg| {
        reg.set_sw_rw(0x12);
    });
    let readback = top.sw_reg().read();
    assert_eq!(readback.sw_rw(), 0x12);
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("../../rdl_src/compile_fail/access_modes_software/*.rs");
}
