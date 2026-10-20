//! `NiAnimation/nibsplinetransforminterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplineTransformInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_B_SPLINE_TRANSFORM_INTERPOLATOR_RTTI: u32 = 0x011f_3fe4;

// Translated from 00a4ffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_b_spline_transform_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_B_SPLINE_TRANSFORM_INTERPOLATOR_RTTI)
}

// Translated from 00a50070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a57f80`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x48 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_b_spline_transform_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_7f80;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x48u32]);
    }
    this
}

/// `NiBSplineInterpolator`'s constructor (`00a57f20`; `thiscall`: data, basis data) and
/// member copy (`00a57fd0`), and `NiBSplineInterpolator::LoadBinary` (`00a58080`).
const B_SPLINE_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a5_7f20;
const B_SPLINE_INTERPOLATOR_COPY_MEMBERS: u32 = 0x00a5_7fd0;
const B_SPLINE_INTERPOLATOR_LOAD_BINARY: u32 = 0x00a5_8080;
/// `NiQuatTransform::LoadBinary` (`00a38bb0`; `thiscall` on the transform: stream).
const QUAT_TRANSFORM_LOAD_BINARY: u32 = 0x00a3_8bb0;
/// The `NiBSplineTransformInterpolator` virtual table (`0109957c`).
const B_SPLINE_TRANSFORM_INTERPOLATOR_VTABLE: u32 = 0x0109_957c;
/// The words the constructor copies into the transform value: the invalid translation
/// (`011a8400`, three words), the invalid rotation (`011f3704`, four words) and the float
/// for the invalid scale (`01096b8c`, through the x87 stack).
const INVALID_TRANSLATE: u32 = 0x011a_8400;
const INVALID_ROTATE: u32 = 0x011f_3704;
const INVALID_SCALE: u32 = 0x0109_6b8c;
/// `NiQuaternion::FastNormalize`'s constants: the doubles at `01099680` and `01099690`, the
/// floats at `011f3fec` and `011f3ff0` (computed by the game's static initializers) and the
/// float at `01099688`.
const NORMALIZE_CENTER: u32 = 0x0109_9680;
const NORMALIZE_SECOND_LIMIT: u32 = 0x0109_9690;
const NORMALIZE_SLOPE: u32 = 0x011f_3fec;
const NORMALIZE_BASE: u32 = 0x011f_3ff0;
const NORMALIZE_THIRD_LIMIT: u32 = 0x0109_9688;

/// One 4-byte read into `destination` through the stream's input object (`NiStream`
/// `+0x24C`; read function at `+8`, `cdecl` (input, destination, 4, address of the
/// component size 4, 1)); the answer added to the input's word at `+4`.
fn read_word(e: &mut Engine, stream: Ptr, destination: u32) {
    let input = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(input + 8);
    let answer = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(read, &args![input, destination, 4u32, size, 1u32])
            .u32()
    });
    let total = e.mem.u32(input + 4).wrapping_add(answer);
    e.mem.set_u32(input + 4, total);
}

// Translated from 00a4fff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator`'s constructor (unnamed in the engine map; `thiscall`,
/// `RET 0x14`: data, translation handle, rotation handle, scale handle, basis data). The
/// `NiBSplineInterpolator` constructor (`00a57f20`, data and basis data), the class's
/// virtual table, the transform value set to the invalid translation, rotation and scale,
/// then the three control point handles. Answers `this`.
pub fn ni_b_spline_transform_interpolator_constructor(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    translate_handle: u32,
    rotate_handle: u32,
    scale_handle: u32,
    basis_data: u32,
) -> Ptr {
    e.call(
        B_SPLINE_INTERPOLATOR_CONSTRUCTOR,
        &args![this, data, basis_data],
    );
    let object = this.addr();
    e.mem
        .set_u32(object, B_SPLINE_TRANSFORM_INTERPOLATOR_VTABLE);
    for index in 0..3 {
        let word = e.global::<u32>(INVALID_TRANSLATE + 4 * index);
        e.mem.set_u32(object + 0x1c + 4 * index, word);
    }
    for index in 0..4 {
        let word = e.global::<u32>(INVALID_ROTATE + 4 * index);
        e.mem.set_u32(object + 0x28 + 4 * index, word);
    }
    let scale = x87_quiet_bits(e.global::<u32>(INVALID_SCALE));
    e.mem.set_u32(object + 0x38, scale);
    e.mem.set_u32(object + 0x3c, translate_handle);
    e.mem.set_u32(object + 0x40, rotate_handle);
    e.mem.set_u32(object + 0x44, scale_handle);
    this
}

// Translated from 00a4fec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::CopyMembers` (unnamed in the engine map; `thiscall` on
/// the source, `RET 8`: clone, cloning process): the `NiBSplineInterpolator` copy
/// (`00a57fd0`), then the transform value (eight words, `REP MOVSD`) and the three control
/// point handles as they are.
pub fn ni_b_spline_transform_interpolator_copy_members(
    e: &mut Engine,
    this: Ptr,
    clone: Ptr,
    cloning: Ptr,
) {
    e.call(
        B_SPLINE_INTERPOLATOR_COPY_MEMBERS,
        &args![this, clone, cloning],
    );
    for offset in (0x1c..0x48).step_by(4) {
        let word = e.mem.u32(this.addr() + offset);
        e.mem.set_u32(clone.addr() + offset, word);
    }
}

// Translated from 00a50470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`: stream):
/// `NiBSplineInterpolator::LoadBinary` (`00a58080`), the transform value
/// (`NiQuatTransform::LoadBinary`, `00a38bb0`, on `this + 0x1C`), then the three handles
/// (one 4-byte read each).
pub fn ni_b_spline_transform_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(B_SPLINE_INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
    e.call(
        QUAT_TRANSFORM_LOAD_BINARY,
        &args![this.addr() + 0x1c, stream],
    );
    read_word(e, stream, this.addr() + 0x3c);
    read_word(e, stream, this.addr() + 0x40);
    read_word(e, stream, this.addr() + 0x44);
}

/// One Newton step of [`ni_quaternion_fast_normalize`] on the stored estimate
/// `estimate` (float bits): `estimate * ((estimate^2 * length) - center) * slope + base`,
/// each intermediate stored as a float as the exe does.
fn newton_step(estimate: u32, length: f64, center: f64, slope: f64, base: f64) -> u32 {
    let value = x87_load32(estimate);
    let squared = x87_store32(x87_mul(x87_mul(value, value), length));
    let offset = x87_sub(x87_load32(squared), center);
    let factor = x87_store32(x87_add(x87_mul(offset, slope), base));
    x87_store32(x87_mul(value, x87_m32(factor)))
}

// Translated from 00a4ff00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternion::FastNormalize` (engine map; `this` in `ECX`, plain `RET`): scales the
/// quaternion (w, x, y, z at `+0`..`+0xC`) by an estimate of `1 / sqrt(length^2)`: a linear
/// guess around the double at `01099680` (slope and base from `011f3fec`, `011f3ff0`), one
/// Newton step when the squared length is at most the double at `01099690` and another
/// when it is at most the float at `01099688` (a NaN length takes none). x87 code, each
/// stored intermediate rounded to a float.
pub fn ni_quaternion_fast_normalize(e: &mut Engine, this: Ptr) {
    let q = this.addr();
    let y = x87_load32(e.mem.u32(q + 8));
    let x = x87_load32(e.mem.u32(q + 4));
    let z = x87_load32(e.mem.u32(q + 0xc));
    let w = x87_load32(e.mem.u32(q));
    let x_squared = x87_mul(x, x);
    let y_squared = x87_mul(y, y);
    let sum = x87_add(y_squared, x_squared);
    let z_squared = x87_mul(z, z);
    let sum = x87_add(sum, z_squared);
    let w_squared = x87_mul(w, w);
    let length = x87_load32(x87_store32(x87_add(sum, w_squared)));
    let center = x87_load64(e.mem.u64(NORMALIZE_CENTER));
    let slope = x87_load32(e.global::<u32>(NORMALIZE_SLOPE));
    let base = x87_load32(e.global::<u32>(NORMALIZE_BASE));
    let guess = x87_add(x87_mul(x87_sub(length, center), slope), base);
    let mut estimate = x87_store32(guess);
    // FCOM with the double: C3/C0 test, so at most the limit (ordered) iterates.
    if length <= f64::from_bits(e.mem.u64(NORMALIZE_SECOND_LIMIT)) {
        estimate = newton_step(estimate, length, center, slope, base);
        // FCOMP of the float limit with the length: no carry means limit >= length.
        if x87_load32(e.global::<u32>(NORMALIZE_THIRD_LIMIT)) >= length {
            estimate = newton_step(estimate, length, center, slope, base);
        }
    }
    let scale = x87_load32(estimate);
    let x = x87_load32(e.mem.u32(q + 4));
    e.mem.set_u32(q + 4, x87_store32(x87_mul(x, scale)));
    let y = x87_mul(scale, x87_m32(e.mem.u32(q + 8)));
    e.mem.set_u32(q + 8, x87_store32(y));
    let z = x87_load32(e.mem.u32(q + 0xc));
    e.mem.set_u32(q + 0xc, x87_store32(x87_mul(z, scale)));
    let w = x87_mul(scale, x87_m32(e.mem.u32(q)));
    e.mem.set_u32(q, x87_store32(w));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4ffe0, ni_b_spline_transform_interpolator_get_rtti() -> Ptr),
        entry!(0x00a50070, ni_b_spline_transform_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a4_fff0, ni_b_spline_transform_interpolator_constructor(Ptr, u32, u32, u32, u32, u32) -> Ptr),
        entry!(
            0x00a4_fec0,
            ni_b_spline_transform_interpolator_copy_members(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00a5_0470,
            ni_b_spline_transform_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a4_ff00, ni_quaternion_fast_normalize(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_b_spline_transform_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a4ffe0, &args![]).u32(),
            NI_B_SPLINE_TRANSFORM_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_b_spline_transform_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a5_7f80, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a50070, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a50070, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x48]]);
        assert_eq!(calls_to(&e, 0x00a5_7f80), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b037_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn constructor_sets_the_invalid_transform_and_the_handles() {
        let mut e = Engine::new();
        e.register(B_SPLINE_INTERPOLATOR_CONSTRUCTOR, |_, _| Ret::default());
        e.map(INVALID_TRANSLATE, 12);
        e.map(INVALID_ROTATE, 16);
        e.map(INVALID_SCALE, 4);
        for index in 0..3 {
            e.set_global(INVALID_TRANSLATE + 4 * index, 0x100 + index);
        }
        for index in 0..4 {
            e.set_global(INVALID_ROTATE + 4 * index, 0x200 + index);
        }
        e.set_global(INVALID_SCALE, 0xff80_0001u32);
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        let answer = e
            .call(
                0x00a4_fff0,
                &args![this, 0x11u32, 1u32, 2u32, 3u32, 0x22u32],
            )
            .u32();
        assert_eq!(answer, this);
        assert_eq!(
            calls(&e, B_SPLINE_INTERPOLATOR_CONSTRUCTOR),
            vec![vec![this, 0x11, 0x22]]
        );
        assert_eq!(e.mem.u32(this), B_SPLINE_TRANSFORM_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0x1c), 0x100);
        assert_eq!(e.mem.u32(this + 0x24), 0x102);
        assert_eq!(e.mem.u32(this + 0x28), 0x200);
        assert_eq!(e.mem.u32(this + 0x34), 0x203);
        assert_eq!(e.mem.u32(this + 0x38), 0xffc0_0001);
        assert_eq!(
            (
                e.mem.u32(this + 0x3c),
                e.mem.u32(this + 0x40),
                e.mem.u32(this + 0x44)
            ),
            (1, 2, 3)
        );
    }

    #[test]
    fn copy_members_copies_the_transform_and_handles() {
        let mut e = Engine::new();
        e.register(B_SPLINE_INTERPOLATOR_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x48);
        let clone = e.mem.alloc(0x4c);
        for offset in (0x1c..0x48).step_by(4) {
            e.mem.set_u32(this + offset, 0x7f80_0000 + offset);
        }
        e.mem.set_u32(clone + 0x48, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x00a4_fec0, &args![this, clone, 0x900u32]);
        assert_eq!(
            calls(&e, B_SPLINE_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this, clone, 0x900]]
        );
        for offset in (0x1c..0x48).step_by(4) {
            assert_eq!(e.mem.u32(clone + offset), 0x7f80_0000 + offset);
        }
        assert_eq!(e.mem.u32(clone + 0x48), 0x5555);
    }

    #[test]
    fn load_binary_reads_the_transform_and_three_handles() {
        let mut e = Engine::new();
        e.register(B_SPLINE_INTERPOLATOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(QUAT_TRANSFORM_LOAD_BINARY, |_, _| Ret::default());
        const READ: u32 = 0x0300_0000;
        e.register(READ, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[2], a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], a[1] & 0xff);
            4u32.into_ret()
        });
        let input = e.mem.alloc(0xc);
        e.mem.set_u32(input + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, input);
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        e.call(0x00a5_0470, &args![this, stream]);
        assert_eq!(
            calls(&e, QUAT_TRANSFORM_LOAD_BINARY),
            vec![vec![this + 0x1c, stream]]
        );
        let reads: Vec<u32> = calls(&e, READ).iter().map(|c| c[1]).collect();
        assert_eq!(reads, vec![this + 0x3c, this + 0x40, this + 0x44]);
        assert_eq!(e.mem.u32(input + 4), 12);
    }

    fn normalize_constants(e: &mut Engine) {
        e.map(NORMALIZE_CENTER, 8);
        e.mem.set_u64(NORMALIZE_CENTER, 0.959066f64.to_bits());
        e.map(NORMALIZE_SECOND_LIMIT, 8);
        e.mem.set_u64(NORMALIZE_SECOND_LIMIT, 0.915212f64.to_bits());
        e.map(NORMALIZE_THIRD_LIMIT, 4);
        e.set_global(NORMALIZE_THIRD_LIMIT, 0.652_119_7f32.to_bits());
        // Slope and base of the tangent of 1/sqrt at the center.
        let center = 0.959066f64;
        let slope = -0.5 * center.powf(-1.5);
        let base = center.powf(-0.5);
        e.map(NORMALIZE_SLOPE, 8);
        e.set_global(NORMALIZE_SLOPE, (slope as f32).to_bits());
        e.set_global(NORMALIZE_BASE, (base as f32).to_bits());
    }

    fn normalized(scale: f32) -> f64 {
        let mut e = Engine::new();
        normalize_constants(&mut e);
        let q = e.mem.alloc(16);
        for (index, part) in [0.5f32, 0.5, 0.5, 0.5].iter().enumerate() {
            e.mem
                .set_u32(q + 4 * index as u32, (part * scale).to_bits());
        }
        e.call(0x00a4_ff00, &args![q]);
        (0..4)
            .map(|index| f64::from(e.mem.f32(q + 4 * index)).powi(2))
            .sum()
    }

    #[test]
    fn fast_normalize_brings_the_length_near_one() {
        // Lengths near 1 (no step), below the second limit (one step), below the third (two).
        for scale in [1.0f32, 0.95, 0.85, 0.7] {
            let length = normalized(scale);
            assert!((length - 1.0).abs() < 0.02, "{scale} {length}");
        }
    }

    #[test]
    fn fast_normalize_scales_every_component_alike() {
        let mut e = Engine::new();
        normalize_constants(&mut e);
        let q = e.mem.alloc(16);
        for (index, part) in [0.1f32, 0.2, 0.3, 0.9].iter().enumerate() {
            e.mem.set_u32(q + 4 * index as u32, part.to_bits());
        }
        e.call(0x00a4_ff00, &args![q]);
        let w = e.mem.f32(q);
        let z = e.mem.f32(q + 0xc);
        assert!((f64::from(z) / f64::from(w) - 9.0).abs() < 1e-5);
    }
}
