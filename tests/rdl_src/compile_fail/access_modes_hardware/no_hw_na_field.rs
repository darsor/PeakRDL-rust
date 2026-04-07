use access_modes_hardware::AccessModesTest;
use peakrdl_rust::io::MockIO;

const SIZE: usize = AccessModesTest::<()>::SIZE;

fn main() {
    let memory: MockIO<SIZE> = MockIO::new_zeroed();
    let top = unsafe { AccessModesTest::from_ptr_with(memory.base_ptr(), &memory) };

    // the hw_na field shouldn't even be generated since it's marked hw=na
    let hw_na_value = top.hw_reg().read().hw_na();
}
