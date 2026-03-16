use peakrdl_rust::{encode::UnknownVariant, io::MockIO};
use peakrdl_rust_test::{
    components::enum_test1::named_types::{my_enum::MyEnum, self_::Self_},
    EnumTest1,
};

const SIZE: usize = EnumTest1::<()>::SIZE;

#[test]
fn test_enum_access() {
    let memory: MockIO<SIZE> = MockIO::new_zeroed();
    let top = unsafe { EnumTest1::from_ptr_with(memory.base_ptr(), &memory) };

    // write default value
    top.reg1().write(|_| {});

    // read and check reset values (name of variant + 1)
    let reg1 = top.reg1().read();
    assert_eq!(reg1.f_default(), Ok(MyEnum::Five));
    assert_eq!(reg1.f_zero(), Ok(MyEnum::One));
    assert_eq!(reg1.f_one(), Err(UnknownVariant(2)));
    assert_eq!(reg1.f_three(), Ok(MyEnum::Four));
    assert_eq!(reg1.f_four(), Ok(MyEnum::Five));
    assert_eq!(reg1.f_five(), Err(UnknownVariant(6)));

    // check write
    top.reg1().modify(|reg| reg.set_f_five(MyEnum::One));
    let reg1 = top.reg1().read();
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
    top.reg2().write(|_| {});
    let reg2 = top.reg2().read();
    assert_eq!(reg2.f0(), Self_::Mro);
    assert_eq!(reg2.f1(), Self_::Dict);
    assert_eq!(reg2.f2(), Self_::Name);
    assert_eq!(reg2.f3(), Self_::Value);
    assert_eq!(reg2.f4(), Self_::Missing);
    assert_eq!(reg2.f5(), Self_::Ignore);
    assert_eq!(reg2.f6(), Self_::Self_);
    assert_eq!(reg2.f7(), Self_::Break);
}
