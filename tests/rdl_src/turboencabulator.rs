#![allow(unused_variables)]

use turboencabulator::{
    access,
    components::turbo_encab::{
        self, TurboEncab, ctrl::ReluctanceFixedPoint, grammeter::status::state::GrammeterStateE,
    },
    encode::UnknownVariant,
    reg,
};

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

const SIZE: usize = TurboEncab::SIZE;
static mut MEMORY: Memory<SIZE> = Memory::new_zeroed();
#[allow(static_mut_refs)]
const TURBO_ENCAB: TurboEncab = unsafe { TurboEncab::from_ptr(MEMORY.as_mut_ptr() as _) };

#[test]
fn test_read() {
    // Get a representation of the status register for the Turbo Encabulator
    let status_reg: reg::Reg<turbo_encab::status::Status, access::R> = TURBO_ENCAB.status();
    // Read the register, returning a newtype of its 32-bit value
    let status: turbo_encab::status::Status = status_reg.read();
    // Access register fields from the previously read value
    let side_fumbling: u16 = status.side_fumbling();
    let stator_rpm: u16 = status.stator_rpm();
}

#[test]
fn test_write() {
    // Get a representation of the control register for the Turbo Encabulator
    let ctrl_reg: reg::Reg<turbo_encab::ctrl::Ctrl, access::RW> = TURBO_ENCAB.ctrl();
    // Writing to the register takes a closure
    ctrl_reg.write(|ctrl: &mut turbo_encab::ctrl::Ctrl| {
        // The input to the closure is the default value of the register,
        // which can be updated via field setter methods before being
        // written to hardware.
        ctrl.set_reset(false);
        ctrl.set_diractance(100);
        ctrl.set_reluctance((-0.375).into()); // fixed-point field
    });
    // Read the value back to check its fields
    let ctrl_value = ctrl_reg.read();
    assert_eq!(ctrl_value.reset(), false);
    assert_eq!(ctrl_value.diractance(), 100);
    assert_eq!(ctrl_value.reluctance().to_f32(), -0.375);
}

#[test]
fn test_modify() {
    TURBO_ENCAB.ctrl().write(|ctrl| {
        ctrl.set_reset(false);
        ctrl.set_diractance(100);
    });

    // Get a representation of the control register for the Turbo Encabulator
    let ctrl_reg: reg::Reg<turbo_encab::ctrl::Ctrl, access::RW> = TURBO_ENCAB.ctrl();
    // The `modify` method performs a read-modify-write access.
    ctrl_reg.modify(|ctrl: &mut turbo_encab::ctrl::Ctrl| {
        // The closure argument is the current value of the register
        assert_eq!(ctrl.reset(), false);
        assert_eq!(ctrl.diractance(), 100);
        // The value can be updated, and is then written back to hardware
        ctrl.set_reset(true)
    });
    // Read the value back to check its fields
    let ctrl_value = ctrl_reg.read();
    assert_eq!(ctrl_value.reset(), true);
    assert_eq!(ctrl_value.diractance(), 100);
}

#[test]
fn test_array() {
    // The SystemRDL source defines an array of 12 grammeters. Handles to each
    // are accessed by the getter method. These lightweight handles store nothing
    // but an address to the corresponding component.
    let grammeters: [turbo_encab::grammeter::Grammeter; 12] = TURBO_ENCAB.grammeter();
    // If only one grammeter is required, the compiler optimizes out the computations
    // for the other addresses.
    let sync_failed: bool = TURBO_ENCAB.grammeter()[3].status().read().sync_failed();
}

#[test]
fn test_enum() {
    // Enum-encoded fields
    match TURBO_ENCAB.grammeter()[3].status().read().state() {
        // Fields with the "encode" property are represented by a Rust enum
        Ok(GrammeterStateE::Reset) => println!("Grammeter 3 is in reset"),
        Ok(GrammeterStateE::Sync) => println!("Grammeter 3 is synchronizing"),
        Ok(GrammeterStateE::Ready) => println!("Grammeter 3 is ready"),
        Ok(GrammeterStateE::SyncFail) => println!("Grammeter 3 is in a failed state"),
        // When reading, a Result::Err is returned if the value doesn't match
        // any encoded variant. The Enum is returned directly (instead of a
        // Result type) if all possible states are encoded.
        Err(UnknownVariant(value)) => println!("Unknown state: {value}"),
    }
}

#[test]
fn test_memory() {
    // TODO
}

#[test]
fn test_fixedpoint() {
    let ctrl_reg = TURBO_ENCAB.ctrl();
    ctrl_reg.write(|ctrl| {
        // The SystemRDL source describes the `reluctance` field as a signed fixed-point
        // number with 1 integer bit and 7 fractional bits. The field accessor methods
        // take and return a `FixedPoint` type, generic over the intwidth and fracwidth.
        //
        // `ReluctanceFixedPoint` is a type alias for `FixedPoint<i8, 1, 7>`
        ctrl.set_reluctance(ReluctanceFixedPoint::from_bits(0xD0_u8 as i8));
        ctrl.set_reluctance(ReluctanceFixedPoint::from_bits(-48));
        ctrl.set_reluctance((-0.375).into());
    });

    // The three field values above are equivalent
    let ctrl_value = ctrl_reg.read();
    assert_eq!(ctrl_value.reluctance().to_bits(), 0xD0_u8 as i8);
    assert_eq!(ctrl_value.reluctance().to_bits(), -48);
    assert_eq!(ctrl_value.reluctance().to_f32(), -0.375);

    // Several convenience methods are available on the type:
    assert_eq!(ReluctanceFixedPoint::intwidth(), 1);
    assert_eq!(ReluctanceFixedPoint::fracwidth(), 7);
    assert_eq!(ReluctanceFixedPoint::is_signed(), true);
    assert_eq!(ReluctanceFixedPoint::resolution().to_f32(), 0.0078125);
    assert_eq!(ReluctanceFixedPoint::min_value().to_f32(), -1.0);
    assert_eq!(ReluctanceFixedPoint::max_value().to_f32(), 0.9921875);
    assert_eq!(ReluctanceFixedPoint::quantize(1.0_f32), 0.9921875);
}
