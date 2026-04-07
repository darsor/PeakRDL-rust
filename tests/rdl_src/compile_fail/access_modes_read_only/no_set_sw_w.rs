use access_modes_read_only::components::access_modes_test::sw_reg::SwReg;
use access_modes_read_only::AccessModesTest;
use peakrdl_rust::io::MockIO;

const SIZE: usize = AccessModesTest::<()>::SIZE;

fn main() {
    let memory: MockIO<SIZE> = MockIO::new_zeroed();
    let top = unsafe { AccessModesTest::from_ptr_with(memory.base_ptr(), &memory) };

    let mut sw_reg_value: SwReg = SwReg::default();
    // the sw_w field shouldn't exist since it's not readable
    sw_reg_value.set_sw_w(0x55);
}
