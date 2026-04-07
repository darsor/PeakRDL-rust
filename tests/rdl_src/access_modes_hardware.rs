use access_modes_hardware::AccessModesTest;
use peakrdl_rust::io::MockIO;

const SIZE: usize = AccessModesTest::<()>::SIZE;

#[test]
fn test_hw_access_mode() {
    let memory: MockIO<SIZE> = MockIO::new_zeroed();
    let top = unsafe { AccessModesTest::from_ptr_with(memory.base_ptr(), &memory) };

    // write default
    top.hw_reg().write(|_| {});

    // test "hardware" read/write
    assert_eq!(top.hw_reg().read().hw_rw(), 0x55);
    top.hw_reg().write(|reg| {
        reg.set_hw_rw(0x89);
    });
    let readback = top.hw_reg().read();
    assert_eq!(readback.hw_rw(), 0x89);
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}
