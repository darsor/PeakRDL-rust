use basic::Basic;

/// A block of memory used for simulating hardware registers.
///
/// A forced alignment of 16 bytes allows access to internal registers as
/// primitive types.
#[repr(align(16))]
pub(crate) struct Memory<const N: usize>([u8; N]);

impl<const N: usize> Memory<N> {
    pub const fn new_zeroed() -> Self {
        Memory([0; N])
    }

    pub const fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

const SIZE: usize = Basic::SIZE;
static mut MEMORY: Memory<SIZE> = Memory::new_zeroed();
#[allow(static_mut_refs)]
const TOP: Basic = unsafe { Basic::from_ptr(MEMORY.as_mut_ptr() as _) };

#[test]
fn test_basic_access() {
    TOP.basicreg_b().write(|reg| {
        reg.set_basicfield_c(12345);
    });
    assert_eq!(TOP.basicreg_b().read().basicfield_c(), 12345);

    // test write
    TOP.basicreg_e().write(|reg| {
        reg.set_basicfield_h(1);
        reg.set_basicfield_i(0x55);
        reg.set_basicfield_j(0xAA);
        reg.set_basicfield_k(255);
    });
    // test read
    let basicreg_e = TOP.basicreg_e().read();
    assert_eq!(basicreg_e.basicfield_h(), 1);
    assert_eq!(basicreg_e.basicfield_i(), 0x55);
    assert_eq!(basicreg_e.basicfield_j(), 0xAA);
    assert_eq!(basicreg_e.basicfield_k(), 255);
    // test modify
    TOP.basicreg_e().modify(|reg| {
        reg.set_basicfield_j(0x1C);
    });
    let basicreg_e = TOP.basicreg_e().read();
    assert_eq!(basicreg_e.basicfield_h(), 1);
    assert_eq!(basicreg_e.basicfield_i(), 0x55);
    assert_eq!(basicreg_e.basicfield_j(), 0x1C);
    assert_eq!(basicreg_e.basicfield_k(), 255);

    // test single-bit access
    for val in [true, false, true] {
        TOP.basicreg_g().write(|reg| {
            reg.set_basicfield_s(val);
        });
        assert_eq!(TOP.basicreg_g().read().basicfield_s(), val);
    }
}
