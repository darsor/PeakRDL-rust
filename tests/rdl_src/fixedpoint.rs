use fixedpoint::components::top;

#[test]
fn test_reset_val() {
    let r1 = top::r1::R1::default();
    let val1 = r1.f_q8_8();
    assert_eq!(val1.to_bits(), 12);
    assert_eq!(val1.to_f32(), 12.0 * 2_f32.powi(-8));

    let val2 = r1.f_q32_n12();
    assert_eq!(val2.to_bits(), 12);
    assert_eq!(val2.to_f32(), 12.0 * 2_f32.powi(12));

    let val3 = r1.f_sqn8_32();
    assert_eq!(val3.to_bits(), 12);
    assert_eq!(val3.to_f32(), 12.0 * 2_f32.powi(-32));

    let val4 = r1.f_sqn6_7();
    assert_eq!(val4.to_bits(), -1);
    assert_eq!(val4.to_f32(), -1.0 * 2_f32.powi(-7));
}
