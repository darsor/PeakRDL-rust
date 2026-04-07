use access_modes_read_only::AccessModesTest;
use peakrdl_rust::io::MockIO;

const SIZE: usize = AccessModesTest::<()>::SIZE;

#[test]
fn test_read_only() {
    let memory: MockIO<SIZE> = MockIO::new_zeroed();
    let top = unsafe { AccessModesTest::from_ptr_with(memory.base_ptr(), &memory) };

    let readback = top.sw_reg().read();
    assert_eq!(readback.sw_rw(), 0x00);
    assert_eq!(readback.sw_r(), 0x00);
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}
