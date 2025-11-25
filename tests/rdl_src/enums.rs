use enums::components::enum_test1::named_types::{my_enum::MyEnum, self_::Self_};
use enums::encode::UnknownVariant;
use enums::EnumTest1;

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

const SIZE: usize = EnumTest1::SIZE;
static mut MEMORY: Memory<SIZE> = Memory::new_zeroed();
#[allow(static_mut_refs)]
const TOP: EnumTest1 = unsafe { EnumTest1::from_ptr(MEMORY.as_mut_ptr() as _) };

#[test]
fn test_enum_access() {
    // write default value
    TOP.reg1().write(|_| {});

    // read and check reset values (name of variant + 1)
    let reg1 = TOP.reg1().read();
    assert_eq!(reg1.f_default(), Ok(MyEnum::Five));
    assert_eq!(reg1.f_zero(), Ok(MyEnum::One));
    assert_eq!(reg1.f_one(), Err(UnknownVariant(2)));
    assert_eq!(reg1.f_three(), Ok(MyEnum::Four));
    assert_eq!(reg1.f_four(), Ok(MyEnum::Five));
    assert_eq!(reg1.f_five(), Err(UnknownVariant(6)));

    // check write
    TOP.reg1().modify(|reg| reg.set_f_five(MyEnum::One));
    let reg1 = TOP.reg1().read();
    assert_eq!(reg1.f_default(), Ok(MyEnum::Five));
    assert_eq!(reg1.f_zero(), Ok(MyEnum::One));
    assert_eq!(reg1.f_one(), Err(UnknownVariant(2)));
    assert_eq!(reg1.f_three(), Ok(MyEnum::Four));
    assert_eq!(reg1.f_four(), Ok(MyEnum::Five));
    assert_eq!(reg1.f_five(), Ok(MyEnum::One));

    // check bit values
    assert_eq!(MyEnum::Zero.bits(), 0);
    assert_eq!(MyEnum::One.bits(), 1);
    assert_eq!(MyEnum::Three.bits(), 3);
    assert_eq!(MyEnum::Four.bits(), 4);
    assert_eq!(MyEnum::Five.bits(), 5);

    // ensure reg2, which has an exhaustively encoded field
    // (3 bits storing 8 variants), returns the enum directly
    TOP.reg2().write(|_| {});
    let reg2 = TOP.reg2().read();
    assert_eq!(reg2.f0(), Self_::Mro);
    assert_eq!(reg2.f1(), Self_::Dict);
    assert_eq!(reg2.f2(), Self_::Name);
    assert_eq!(reg2.f3(), Self_::Value);
    assert_eq!(reg2.f4(), Self_::Missing);
    assert_eq!(reg2.f5(), Self_::Ignore);
    assert_eq!(reg2.f6(), Self_::Self_);
    assert_eq!(reg2.f7(), Self_::Break);
}
