//! `Havok SDK/hkpmotion.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMotion` (RTTI `.?AVhkpMotion@@` confirmed at the vtable `010c59f4`): its motion state
//! (`hkMotionState`, 0xAF bytes) sits at +0x10 and the two velocity vectors at +0xD0 and
//! +0xE0; the inverse mass is the fourth lane of the vector at +0xC0 (+0xCC).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `hkpMotion` (RTTI-confirmed class).
const MOTION_VTABLE: u32 = 0x010c_59f4;
/// Offset of the `hkMotionState` inside the motion.
const MOTION_STATE: u32 = 0x10;
/// The constructor of the motion state built at +0x10 (`00cf2410`, `this`, `(first, second)`).
const MOTION_STATE_CONSTRUCT: u32 = 0x00cf_2410;

/// `FISTP dword` in the truncating rounding mode: the 32-bit "integer indefinite" for NaN
/// and out-of-range values.
fn truncate_to_i32(value: f32) -> i32 {
    let truncated = f64::from(value).trunc();
    if value.is_nan() || !(-2147483648.0..2147483648.0).contains(&truncated) {
        i32::MIN
    } else {
        truncated as i32
    }
}

// Translated from 00c86430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::getMass` (Xbox PDB): the reciprocal of the inverse mass at +0xCC, rounded to
/// `float`; an inverse mass equal to zero gives `0` (a NaN is not equal, so it divides).
pub fn hkp_motion_get_mass(e: &mut Engine, this: Ptr) -> f32 {
    let inverse_mass = e.mem.f32(this.addr() + 0xcc);
    if inverse_mass == 0.0 {
        return 0.0;
    }
    if inverse_mass.is_nan() {
        return x87_quiet(inverse_mass);
    }
    (1.0 / f64::from(inverse_mass)) as f32
}

// Translated from 00c86660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMotionState::operator=` (Xbox PDB): copies the first 0xA0 bytes as sixteen-byte
/// blocks, the three floats at +0xA0, +0xA4 and +0xA8 through the x87 (a signalling NaN
/// comes out quiet) and the bytes at +0xAC, +0xAD and +0xAE.
pub fn hk_motion_state_assign(e: &mut Engine, this: Ptr, source: Ptr) {
    let (to, from) = (this.addr(), source.addr());
    for offset in (0..0xa0).step_by(4) {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    for offset in [0xa0, 0xa4, 0xa8] {
        let bits = x87_quiet_bits(e.mem.u32(from + offset));
        e.mem.set_u32(to + offset, bits);
    }
    for offset in [0xac, 0xad, 0xae] {
        let byte = e.mem.u8(from + offset);
        e.mem.set_u8(to + offset, byte);
    }
}

// Translated from 00c86610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::getMotionStateAndVelocitiesAndDeactivationType` (Xbox PDB): copies this
/// motion's state (+0x10, `operator=`), both velocity vectors (+0xD0, +0xE0) and the byte
/// at +9 into `other`.
pub fn hkp_motion_get_motion_state_and_velocities_and_deactivation_type(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) {
    let from = Ptr::new(this.addr() + MOTION_STATE);
    let to = Ptr::new(other.addr() + MOTION_STATE);
    hk_motion_state_assign(e, to, from);
    for offset in (0..0x20).step_by(4) {
        let word = e.mem.u32(this.addr() + 0xd0 + offset);
        e.mem.set_u32(other.addr() + 0xd0 + offset, word);
    }
    let byte = e.mem.u8(this.addr() + 9);
    e.mem.set_u8(other.addr() + 9, byte);
}

// Translated from 00c86530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion` constructor, named by its body (the map gives it no name; the vtable it
/// stores is the RTTI-confirmed `hkpMotion` one): sets the word at +6 to 1, the vtable and
/// zero in +0xD0..0xEF, builds the motion state at +0x10 from `(first, second)`
/// (`00cf2410`), clears +0xB4, +0xB8 and the byte at +8. The byte at +9 is `0xFF` when
/// `use_class` is `0`, else the first float of `first` truncated, low four bits. Then clears
/// +0xA, +0xC, +0xF0..0x10F, +0x110 and +0x114 and stores `0x3F80` (the high half of 1.0f)
/// at +0x11E. Returns `this`.
pub fn fn_00c86530(e: &mut Engine, this: Ptr, first: Ptr, second: u32, use_class: u8) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base, MOTION_VTABLE);
    for offset in (0..0x20).step_by(4) {
        e.mem.set_u32(base + 0xd0 + offset, 0);
    }
    e.call(
        MOTION_STATE_CONSTRUCT,
        &args![base + MOTION_STATE, first, second],
    );
    e.mem.set_f32(base + 0xb4, 0.0);
    e.mem.set_f32(base + 0xb8, 0.0);
    e.mem.set_u8(base + 8, 0);
    if use_class == 0 {
        e.mem.set_u8(base + 9, 0xff);
    } else {
        let truncated = truncate_to_i32(e.mem.f32(first.addr()));
        e.mem.set_u8(base + 9, (truncated & 0xf) as u8);
    }
    e.mem.set_u16(base + 0xa, 0);
    e.mem.set_u16(base + 0xc, 0);
    for offset in (0..0x20).step_by(4) {
        e.mem.set_u32(base + 0xf0 + offset, 0);
    }
    e.mem.set_u32(base + 0x110, 0);
    e.mem.set_u32(base + 0x114, 0);
    e.mem.set_u16(base + 0x11e, 0x3f80);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c8_6430, hkp_motion_get_mass(Ptr) -> f32),
        entry!(0x00c8_6660, hk_motion_state_assign(Ptr, Ptr)),
        entry!(
            0x00c8_6610,
            hkp_motion_get_motion_state_and_velocities_and_deactivation_type(Ptr, Ptr)
        ),
        entry!(0x00c8_6530, fn_00c86530(Ptr, Ptr, u32, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_mass_is_the_reciprocal_unless_the_inverse_mass_is_zero() {
        let mut e = Engine::new();
        let motion = e.mem.alloc(0x130);
        e.mem.set_f32(motion + 0xcc, 4.0);
        assert_eq!(e.call(0x00c8_6430, &args![motion]).f32(), 0.25);
        e.mem.set_f32(motion + 0xcc, 0.0);
        assert_eq!(e.call(0x00c8_6430, &args![motion]).f32(), 0.0);
        e.mem.set_f32(motion + 0xcc, -0.0);
        assert_eq!(e.call(0x00c8_6430, &args![motion]).f32().to_bits(), 0);
        e.mem.set_f32(motion + 0xcc, f32::NAN);
        assert!(e.call(0x00c8_6430, &args![motion]).f32().is_nan());
    }

    #[test]
    fn state_assign_copies_the_block_and_quiets_the_three_floats() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0xb0);
        let dest = e.mem.alloc(0xb0);
        for offset in (0..0xb0).step_by(4) {
            e.mem.set_u32(source + offset, 0x1000 + offset);
        }
        e.mem.set_u32(source + 0xa0, 0x7f80_0001);
        e.mem.set_u32(source + 0xa4, 0x3f80_0000);
        e.mem.set_u32(source + 0xa8, 0xff80_1234);
        e.mem.set_u8(source + 0xac, 1);
        e.mem.set_u8(source + 0xad, 2);
        e.mem.set_u8(source + 0xae, 3);
        e.call(0x00c8_6660, &args![dest, source]);
        for offset in (0..0xa0).step_by(4) {
            assert_eq!(e.mem.u32(dest + offset), 0x1000 + offset);
        }
        assert_eq!(e.mem.u32(dest + 0xa0), 0x7fc0_0001);
        assert_eq!(e.mem.u32(dest + 0xa4), 0x3f80_0000);
        assert_eq!(e.mem.u32(dest + 0xa8), 0xffc0_1234);
        let tail = [
            e.mem.u8(dest + 0xac),
            e.mem.u8(dest + 0xad),
            e.mem.u8(dest + 0xae),
        ];
        assert_eq!(tail, [1, 2, 3]);
    }

    #[test]
    fn get_motion_state_copies_state_velocities_and_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x130);
        let other = e.mem.alloc(0x130);
        for offset in (0x10..0xc0).step_by(4) {
            e.mem.set_u32(this + offset, 0x2000 + offset);
        }
        for offset in (0xd0..0xf0).step_by(4) {
            e.mem.set_u32(this + offset, 0x3000 + offset);
        }
        e.mem.set_u8(this + 9, 7);
        e.mem.set_u32(other + 0xcc, 0x55);
        e.call(0x00c8_6610, &args![this, other]);
        assert_eq!(e.mem.u32(other + 0x10), 0x2010);
        assert_eq!(e.mem.u32(other + 0xa0), 0x2000 + 0xa0);
        assert_eq!(e.mem.u32(other + 0xd0), 0x30d0);
        assert_eq!(e.mem.u32(other + 0xec), 0x30ec);
        assert_eq!(e.mem.u8(other + 9), 7);
        assert_eq!(e.mem.u32(other + 0xcc), 0x55);
    }

    fn construct(first_value: f32, use_class: u8) -> (Engine, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x130);
        for offset in (0..0x130).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        let first = e.mem.alloc(0x10);
        e.mem.set_f32(first, first_value);
        e.register(MOTION_STATE_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c8_6530, &args![this, first, 0x1234u32, use_class]);
        (e, this)
    }

    #[test]
    fn constructor_sets_the_fields_and_the_class_byte() {
        let (mut e, this) = construct(37.9, 1);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), MOTION_VTABLE);
        assert_eq!(e.mem.u8(this + 8), 0);
        assert_eq!(e.mem.u8(this + 9), 37 & 0xf);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0xb4), 0);
        assert_eq!(e.mem.u32(this + 0xb8), 0);
        for offset in (0xd0..0xf0).step_by(4).chain((0xf0..0x118).step_by(4)) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u16(this + 0x11e), 0x3f80);
        assert_eq!(e.mem.u32(this + 0x118), 0xdead_beef);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|c| c.0 == MOTION_STATE_CONSTRUCT)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this + MOTION_STATE);
    }

    #[test]
    fn constructor_class_byte_is_ff_without_the_flag_and_indefinite_gives_zero() {
        let (e, this) = construct(5.0, 0);
        assert_eq!(e.mem.u8(this + 9), 0xff);
        let (e, this) = construct(f32::NAN, 1);
        assert_eq!(e.mem.u8(this + 9), 0);
        let (e, this) = construct(1.0e20, 1);
        assert_eq!(e.mem.u8(this + 9), 0);
    }
}
