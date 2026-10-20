//! `NiAnimation/nitransforminterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nimainsdm::{interlocked_decrement, interlocked_increment};
use crate::units::nimain::nistream::NiStream;

/// `NiTransformInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRANSFORM_INTERPOLATOR_RTTI: u32 = 0x011f_3e50;

// Translated from 00a3fad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_transform_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRANSFORM_INTERPOLATOR_RTTI)
}

layout! {
    /// `NiTransformInterpolator` (Xbox PDB), 0x48 bytes on PC as on the Xbox: an
    /// `NiKeyBasedInterpolator` (vtable, count, last time), the transform value (an
    /// `NiQuatTransform`: translation, rotation `w x y z`, scale), the data and the last key
    /// indices. Floats are declared as their bit patterns.
    pub struct NiTransformInterpolator: 0x48 {
        /// `NiInterpolator::m_fLastTime` (Xbox PDB).
        0x08 m_fLastTime: u32,
        /// `m_kTransformValue.m_kTranslate` (Xbox PDB), three floats.
        0x0C m_kTranslate: u32,
        /// `m_kTransformValue.m_kRotate` (Xbox PDB), four floats (`m_fW` first).
        0x18 m_kRotate: u32,
        /// `m_kTransformValue.m_kRotate.m_fX` (Xbox PDB).
        0x1C m_fRotateX: u32,
        /// `m_kTransformValue.m_fScale` (Xbox PDB).
        0x28 m_fScale: u32,
        /// `m_spData` (Xbox PDB): `NiPointer<NiTransformData>`.
        0x2C m_spData: u32,
        /// `m_usLastTransIdx` (Xbox PDB).
        0x30 m_usLastTransIdx: u16,
        /// `m_usLastRotIdx` (Xbox PDB).
        0x32 m_usLastRotIdx: u16,
        /// `m_usLastScaleIdx` (Xbox PDB).
        0x34 m_usLastScaleIdx: u16,
        /// `m_origTranslate` (Xbox PDB), three floats.
        0x38 m_origTranslate: u32,
        /// `bPose` (Xbox PDB).
        0x44 bPose: u8,
    }
}

/// `NiTransformInterpolator`'s vtable.
const NI_TRANSFORM_INTERPOLATOR_VTABLE: u32 = 0x0109_7314;
/// The double `-FLT_MAX` an unset translation, rotation or scale holds (compared as such).
const UNSET_DOUBLE: u32 = 0x0102_41b0;
/// The floats a time range starts from: `01097308` (begin) and `-FLT_MAX` (`01015f5c`, end).
const RANGE_BEGIN_FLOAT: u32 = 0x0109_7308;
const RANGE_END_FLOAT: u32 = 0x0101_5f5c;
/// `operator new` (`cdecl`, the size) and the sized delete (`cdecl`, (block, size)).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `00a3fa30` (`thiscall`, the data): the constructor from transform data.
const CONSTRUCT_FROM_DATA: u32 = 0x00a3_fa30;
/// `NiKeyBasedInterpolator`'s constructor and destructor body (`thiscall`, no stack words),
/// its `LoadBinary` (`00aa0ac0`, the stream) and its member copy (`00aa0ab0`, (clone, cloning
/// process)).
const KEY_BASED_CONSTRUCT: u32 = 0x00a3_ca10;
const KEY_BASED_DESTRUCT: u32 = 0x00a3_ca30;
const KEY_BASED_LOAD_BINARY: u32 = 0x00aa_0ac0;
const KEY_BASED_COPY_MEMBERS: u32 = 0x00aa_0ab0;
/// `NiQuatTransform::LoadBinary` (`thiscall` on the transform, the stream).
const QUAT_TRANSFORM_LOAD_BINARY: u32 = 0x00a3_8bb0;
/// `NiStream::ResolveLinkID` (`thiscall`, no stack words).
const STREAM_RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// The three key getters (`thiscall`, (&count, &type, &size); answer the keys):
/// translation `00a3f990`, rotation `0058cb00`, scale `00a3f9e0` (the engine map's names on
/// them, `IsTransformInvalid`, `SetTranslate`, `SetRotate`, do not fit their use here).
const GET_TRANSLATION_KEYS: u32 = 0x00a3_f990;
const GET_ROTATION_KEYS: u32 = 0x0058_cb00;
const GET_SCALE_KEYS: u32 = 0x00a3_f9e0;
/// The key interpolations (`cdecl`): `NiPosKey::GenInterp` and `NiRotKey::GenInterp`
/// ((result, time, keys, type, count, &last index, size), answer the result) and
/// `NiFloatKey::GenInterp` ((time, keys, type, count, &last index, size), answers in `ST0`).
const POS_KEY_GEN_INTERP: u32 = 0x00a2_7490;
const ROT_KEY_GEN_INTERP: u32 = 0x00a2_8740;
const FLOAT_KEY_GEN_INTERP: u32 = 0x00a2_6b40;

/// Releases a counted object (count at `+4`): its slot `+4` runs when the count reaches 0.
fn release(e: &mut Engine, object: u32) {
    if interlocked_decrement(e, object.wrapping_add(4)) == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// The compiler's `NiPointer` assignment of `new` to the word at `slot`.
fn assign_pointer(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old == new {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(slot, new);
    if new != 0 {
        interlocked_increment(e, new.wrapping_add(4));
    }
}

/// Copies the eight words of the transform value (+0xc) to `destination` (`REP MOVSD`).
fn copy_transform(e: &mut Engine, this: Ptr<NiTransformInterpolator>, destination: u32) {
    for word in 0..8 {
        let value = e.mem.u32(this.addr() + 0xc + 4 * word);
        e.mem.set_u32(destination + 4 * word, value);
    }
}

/// Whether the scale, the rotation's `m_fX` and the translation's first float are all the
/// unset double `-FLT_MAX` (`010241b0`), compared in that order.
fn transform_unset(e: &Engine, this: Ptr<NiTransformInterpolator>) -> bool {
    let unset = x87_load64(e.mem.u64(UNSET_DOUBLE));
    x87_load32(e.get(this, NiTransformInterpolator::m_fScale)) == unset
        && x87_load32(e.get(this, NiTransformInterpolator::m_fRotateX)) == unset
        && x87_load32(e.get(this, NiTransformInterpolator::m_kTranslate)) == unset
}

// Translated from 00a3f7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::LoadBinary` (Xbox PDB): `NiKeyBasedInterpolator::LoadBinary`,
/// the transform value (`NiQuatTransform::LoadBinary`), its translation kept as the original
/// translation, the data from the stream's link id (`NiStream::ResolveLinkID`, counted).
/// `bPose` is set when there is no data and the scale, the rotation's `m_fX` or the
/// translation's first float is not `-FLT_MAX` (the double at `010241b0`); cleared otherwise.
pub fn ni_transform_interpolator_load_binary(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    stream: Ptr<NiStream>,
) {
    e.call(KEY_BASED_LOAD_BINARY, &args![this, stream]);
    e.call(
        QUAT_TRANSFORM_LOAD_BINARY,
        &args![this.addr() + 0xc, stream],
    );
    for word in 0..3 {
        let value = e.mem.u32(this.addr() + 0xc + 4 * word);
        e.mem.set_u32(this.addr() + 0x38 + 4 * word, value);
    }
    let data = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.addr() + NiTransformInterpolator::m_spData.off, data);
    let pose = e.get(this, NiTransformInterpolator::m_spData) == 0 && !transform_unset(e, this);
    e.set(this, NiTransformInterpolator::bPose, u8::from(pose));
}

// Translated from 00a3fae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::NiTransformInterpolator` (Xbox PDB; `thiscall`, the
/// `NiQuatTransform` by value, `RET 0x20`): the `NiKeyBasedInterpolator` constructor, the
/// eight words of the transform as the transform value, the vtable `01097314`, no data, no
/// pose and the last key indices 0. Answers `this`.
#[allow(clippy::too_many_arguments)]
pub fn ni_transform_interpolator_construct(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    translate_x: u32,
    translate_y: u32,
    translate_z: u32,
    rotate_w: u32,
    rotate_x: u32,
    rotate_y: u32,
    rotate_z: u32,
    scale: u32,
) -> Ptr<NiTransformInterpolator> {
    e.call(KEY_BASED_CONSTRUCT, &args![this]);
    let words = [
        translate_x,
        translate_y,
        translate_z,
        rotate_w,
        rotate_x,
        rotate_y,
        rotate_z,
        scale,
    ];
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(this.addr() + 0xc + 4 * i as u32, *word);
    }
    e.mem.set_u32(this.addr(), NI_TRANSFORM_INTERPOLATOR_VTABLE);
    e.set(this, NiTransformInterpolator::m_spData, 0);
    e.set(this, NiTransformInterpolator::bPose, 0);
    e.set(this, NiTransformInterpolator::m_usLastTransIdx, 0);
    e.set(this, NiTransformInterpolator::m_usLastRotIdx, 0);
    e.set(this, NiTransformInterpolator::m_usLastScaleIdx, 0);
    this
}

/// Widens `[*begin, *end]` by the first and last keys' times (the first float of a key) of
/// `count` keys of `size` bytes at `keys`; each time goes through a `float` local first.
fn widen_range(e: &mut Engine, begin: u32, end: u32, keys: u32, count: u32, size: u32) {
    let first = x87_load32(x87_store32(x87_load32(e.mem.u32(keys))));
    if x87_load32(e.mem.u32(begin)) > first {
        e.mem.set_u32(begin, x87_store32(first));
    }
    let at = count.wrapping_sub(1).wrapping_mul(size).wrapping_add(keys);
    let last = x87_load32(x87_store32(x87_load32(e.mem.u32(at))));
    if x87_load32(e.mem.u32(end)) < last {
        e.mem.set_u32(end, x87_store32(last));
    }
}

// Translated from 00a3fb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::GetActiveTimeRange` (Xbox PDB): the range starts at the float
/// `01097308` (begin) and `-FLT_MAX` (`01015f5c`, end) and is widened by the first and last
/// key of each key list of the data (`NiTransformData`): the translation keys (count +0xa,
/// size byte +0x1d, keys +0x24), the rotation keys (count +0x8, type +0x10, size byte +0x1c,
/// keys +0x20; for type 4, Euler keys, the three axis lists of the Euler key object: counts at
/// +0x14, size bytes at +0x2c, keys at +0x30), the scale keys (count +0xc, size byte +0x1e,
/// keys +0x28). Without data or keys the range is `[0, 0]`.
pub fn ni_transform_interpolator_get_active_time_range(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    begin: Ptr,
    end: Ptr,
) {
    let (begin, end) = (begin.addr(), end.addr());
    let start = x87_store32(x87_load32(e.mem.u32(RANGE_BEGIN_FLOAT)));
    e.mem.set_u32(begin, start);
    let stop = x87_store32(x87_load32(e.mem.u32(RANGE_END_FLOAT)));
    e.mem.set_u32(end, stop);
    let mut found = false;
    let data = e.get(this, NiTransformInterpolator::m_spData);
    if data != 0 {
        let count = u32::from(e.mem.u16(data + 0xa));
        let size = u32::from(e.mem.u8(data + 0x1d));
        let keys = e.mem.u32(data + 0x24);
        if count != 0 {
            widen_range(e, begin, end, keys, count, size);
            found = true;
        }
        let count = u32::from(e.mem.u16(data + 0x8));
        let kind = e.mem.u32(data + 0x10);
        let size = u32::from(e.mem.u8(data + 0x1c));
        let keys = e.mem.u32(data + 0x20);
        if count != 0 {
            if kind == 4 {
                for axis in 0..3 {
                    let axis_count = e.mem.u32(keys + 0x14 + 4 * axis);
                    if axis_count != 0 {
                        let axis_keys = e.mem.u32(keys + 0x30 + 4 * axis);
                        let axis_size = u32::from(e.mem.u8(keys + 0x2c + axis));
                        widen_range(e, begin, end, axis_keys, axis_count, axis_size);
                        found = true;
                    }
                }
            } else {
                widen_range(e, begin, end, keys, count, size);
                found = true;
            }
        }
        let count = u32::from(e.mem.u16(data + 0xc));
        let size = u32::from(e.mem.u8(data + 0x1e));
        let keys = e.mem.u32(data + 0x28);
        if count != 0 {
            widen_range(e, begin, end, keys, count, size);
            return;
        }
    }
    if !found {
        e.mem.set_u32(begin, x87_store32(0.0));
        e.mem.set_u32(end, x87_store32(0.0));
    }
}

// Translated from 00a3fdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::Update` (Xbox PDB; `RET 0xc`: time, the interpolation target,
/// which it does not read, and the `NiQuatTransform` to fill): a pose copies the transform
/// value out after taking `time` as its last time. At the last time the value is copied out
/// unchanged. Otherwise the translation, rotation and scale are interpolated from their keys
/// (`NiPosKey`, `NiRotKey`, `NiFloatKey::GenInterp`, each from the last key index, which it
/// updates; skipped for a list without keys) and the value copied out. Answers 0 when the
/// scale, the rotation's `m_fX` and the translation's first float are all `-FLT_MAX` (an
/// interpolation then does not take the time), 1 otherwise.
pub fn ni_transform_interpolator_update(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    time: f32,
    _unused_2: u32,
    value: Ptr,
) -> bool {
    let time_bits = time.to_bits();
    if e.get(this, NiTransformInterpolator::bPose) != 0 {
        let last = x87_store32(x87_load32(time_bits));
        e.set(this, NiTransformInterpolator::m_fLastTime, last);
        copy_transform(e, this, value.addr());
        return true;
    }
    let last = x87_load32(e.get(this, NiTransformInterpolator::m_fLastTime));
    if x87_load32(time_bits) == last {
        copy_transform(e, this, value.addr());
        return !transform_unset(e, this);
    }
    let time_word = x87_store32(x87_load32(time_bits));
    // The locals: last index (+0), count (+4), size (+8), type (+0xc), result (+0x10).
    e.with_stack(0x20, |e, locals| {
        let l = locals.addr();
        let (index, count, size, kind, result) = (l, l + 4, l + 8, l + 0xc, l + 0x10);
        let keys = e
            .call(GET_TRANSLATION_KEYS, &args![this, count, kind, size])
            .u32();
        let n = e.mem.u32(count);
        if n != 0 {
            let last_index = u32::from(e.get(this, NiTransformInterpolator::m_usLastTransIdx));
            e.mem.set_u32(index, last_index);
            let (k, s) = (e.mem.u32(kind), e.mem.u32(size));
            let out = e
                .call(
                    POS_KEY_GEN_INTERP,
                    &args![result, time_word, keys, k, n, index, s],
                )
                .u32();
            for word in 0..3 {
                let v = e.mem.u32(out + 4 * word);
                e.mem.set_u32(this.addr() + 0xc + 4 * word, v);
            }
            let updated = e.mem.u16(index);
            e.set(this, NiTransformInterpolator::m_usLastTransIdx, updated);
        }
        let keys = e
            .call(GET_ROTATION_KEYS, &args![this, count, kind, size])
            .u32();
        let n = e.mem.u32(count);
        if n != 0 {
            let last_index = u32::from(e.get(this, NiTransformInterpolator::m_usLastRotIdx));
            e.mem.set_u32(index, last_index);
            let (k, s) = (e.mem.u32(kind), e.mem.u32(size));
            let out = e
                .call(
                    ROT_KEY_GEN_INTERP,
                    &args![result, time_word, keys, k, n, index, s],
                )
                .u32();
            for word in 0..4 {
                let v = e.mem.u32(out + 4 * word);
                e.mem.set_u32(this.addr() + 0x18 + 4 * word, v);
            }
            let updated = e.mem.u16(index);
            e.set(this, NiTransformInterpolator::m_usLastRotIdx, updated);
        }
        let keys = e
            .call(GET_SCALE_KEYS, &args![this, count, kind, size])
            .u32();
        let n = e.mem.u32(count);
        if n != 0 {
            let last_index = u32::from(e.get(this, NiTransformInterpolator::m_usLastScaleIdx));
            e.mem.set_u32(index, last_index);
            let (k, s) = (e.mem.u32(kind), e.mem.u32(size));
            let scale = e
                .call(
                    FLOAT_KEY_GEN_INTERP,
                    &args![time_word, keys, k, n, index, s],
                )
                .f64();
            e.set(this, NiTransformInterpolator::m_fScale, x87_store32(scale));
            let updated = e.mem.u16(index);
            e.set(this, NiTransformInterpolator::m_usLastScaleIdx, updated);
        }
    });
    copy_transform(e, this, value.addr());
    if transform_unset(e, this) {
        return false;
    }
    let last = x87_store32(x87_load32(time_bits));
    e.set(this, NiTransformInterpolator::m_fLastTime, last);
    true
}

// Translated from 00a40360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitransforminterpolator.cpp`; by its body the member
/// copy): `00aa0ab0` (the key-based interpolator's copy), then the transform value, the
/// original translation, the data (counted) and the last key indices into `clone`.
pub fn fn_00a40360(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    clone: Ptr<NiTransformInterpolator>,
    cloning_process: Ptr,
) {
    e.call(KEY_BASED_COPY_MEMBERS, &args![this, clone, cloning_process]);
    copy_transform(e, this, clone.addr() + 0xc);
    for word in 0..3 {
        let value = e.mem.u32(this.addr() + 0x38 + 4 * word);
        e.mem.set_u32(clone.addr() + 0x38 + 4 * word, value);
    }
    let data = e.get(this, NiTransformInterpolator::m_spData);
    assign_pointer(
        e,
        clone.addr() + NiTransformInterpolator::m_spData.off,
        data,
    );
    for field in [
        NiTransformInterpolator::m_usLastTransIdx,
        NiTransformInterpolator::m_usLastRotIdx,
        NiTransformInterpolator::m_usLastScaleIdx,
    ] {
        let value = e.get(this, field);
        e.set(clone, field, value);
    }
}

// Translated from 00a403f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiTransformInterpolator`): allocates 0x48 bytes (`operator new`) and, when
/// that succeeds, runs the constructor from data (`00a3fa30`) with no data on them, answering
/// its result; null otherwise.
pub fn fn_00a403f0(e: &mut Engine) -> Ptr<NiTransformInterpolator> {
    let block = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(CONSTRUCT_FROM_DATA, &args![block, 0u32]).ptr()
}

// Translated from 00a40410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::_scalar_deleting_destructor_` (Xbox PDB): releases the data
/// (when there is one; the word is left as it is), runs the `NiKeyBasedInterpolator`
/// destructor body (`00a3ca30`) and, when bit 0 of `flags` is set, the sized delete of 0x48
/// bytes. Answers `this`.
pub fn ni_transform_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    flags: u32,
) -> Ptr<NiTransformInterpolator> {
    let data = e.get(this, NiTransformInterpolator::m_spData);
    if data != 0 {
        release(e, data);
    }
    e.call(KEY_BASED_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x48u32]);
    }
    this
}

// Translated from 00a40460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::CreateClone` (Xbox PDB): a new interpolator (0x48 bytes,
/// `operator new`, the constructor from data `00a3fa30` with no data; null when the
/// allocation fails) given this one's members (`00a40360`, which runs on null too). Answers
/// the clone.
pub fn ni_transform_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    cloning_process: Ptr,
) -> Ptr<NiTransformInterpolator> {
    let block = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
    let clone = if block != 0 {
        e.call(CONSTRUCT_FROM_DATA, &args![block, 0u32]).u32()
    } else {
        0
    };
    fn_00a40360(e, this, Ptr::new(clone), cloning_process);
    Ptr::new(clone)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3fad0, ni_transform_interpolator_get_rtti() -> Ptr),
        entry!(
            0x00a3_f7b0,
            ni_transform_interpolator_load_binary(Ptr<NiTransformInterpolator>, Ptr<NiStream>)
        ),
        entry!(
            0x00a3_fae0,
            ni_transform_interpolator_construct(
                Ptr<NiTransformInterpolator>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> Ptr<NiTransformInterpolator>
        ),
        entry!(
            0x00a3_fb20,
            ni_transform_interpolator_get_active_time_range(Ptr<NiTransformInterpolator>, Ptr, Ptr)
        ),
        entry!(
            0x00a3_fdb0,
            ni_transform_interpolator_update(Ptr<NiTransformInterpolator>, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a4_0360,
            fn_00a40360(Ptr<NiTransformInterpolator>, Ptr<NiTransformInterpolator>, Ptr)
        ),
        entry!(0x00a4_03f0, fn_00a403f0() -> Ptr<NiTransformInterpolator>),
        entry!(
            0x00a4_0410,
            ni_transform_interpolator_scalar_deleting_destructor(
                Ptr<NiTransformInterpolator>,
                u32,
            )
                -> Ptr<NiTransformInterpolator>
        ),
        entry!(
            0x00a4_0460,
            ni_transform_interpolator_create_clone(
                Ptr<NiTransformInterpolator>,
                Ptr,
            ) -> Ptr<NiTransformInterpolator>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_transform_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a3fad0, &args![]).u32(),
            NI_TRANSFORM_INTERPOLATOR_RTTI
        );
    }
}

#[cfg(test)]
mod interpolator_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};

    const UNSET: u32 = 0xff7f_ffff;

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x0102_4000, 0x1000);
        e.map(0x0101_5000, 0x1000);
        e.map(0x0109_7000, 0x1000);
        e.mem.set_u64(UNSET_DOUBLE, f64::from(f32::MIN).to_bits());
        e.mem.set_u32(RANGE_END_FLOAT, UNSET);
        e.mem.set_u32(RANGE_BEGIN_FLOAT, 0x7f7f_ffff);
        e.put_vtable(0x0300_0000, &[0, 0x0300_0004]);
        e.register(0x0300_0004, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn counted(e: &mut Engine, count: u32) -> u32 {
        let o = e.mem.alloc(0x30);
        e.mem.set_u32(o, 0x0300_0000);
        e.mem.set_u32(o + 4, count);
        o
    }

    fn set_f(e: &mut Engine, at: u32, v: f32) {
        e.mem.set_u32(at, v.to_bits());
    }

    fn unset_transform(e: &mut Engine, this: Ptr<NiTransformInterpolator>) {
        e.set(this, NiTransformInterpolator::m_kTranslate, UNSET);
        e.set(this, NiTransformInterpolator::m_fRotateX, UNSET);
        e.set(this, NiTransformInterpolator::m_fScale, UNSET);
    }

    #[test]
    fn load_binary_resolves_the_data_and_decides_the_pose() {
        let mut e = engine();
        e.register(KEY_BASED_LOAD_BINARY, |_, _| Ret::default());
        e.register(QUAT_TRANSFORM_LOAD_BINARY, |e, a| {
            for word in 0..8 {
                e.mem.set_u32(a[0] + 4 * word, 0x100 + word);
            }
            Ret::default()
        });
        e.register(STREAM_RESOLVE_LINK_ID, |_, _| 0u32.into_ret());
        let this = e.new_object::<NiTransformInterpolator>();
        let stream = e.new_object::<NiStream>();
        e.call(0x00a3_f7b0, &args![this, stream]);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 0x100);
        assert_eq!(e.mem.u32(this.addr() + 0x40), 0x102);
        assert_eq!(e.get(this, NiTransformInterpolator::bPose), 1);
        assert_eq!(
            calls_to(&e, QUAT_TRANSFORM_LOAD_BINARY),
            vec![vec![this.addr() + 0xc, stream.addr()]]
        );
        // With data: no pose, the data counted.
        let data = counted(&mut e, 0);
        e.register_double(STREAM_RESOLVE_LINK_ID, move |_, _| data.into_ret());
        e.call(0x00a3_f7b0, &args![this, stream]);
        assert_eq!(e.get(this, NiTransformInterpolator::m_spData), data);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(e.get(this, NiTransformInterpolator::bPose), 0);
        // No data and everything unset: no pose; the old data released.
        e.register(STREAM_RESOLVE_LINK_ID, |_, _| 0u32.into_ret());
        e.register(QUAT_TRANSFORM_LOAD_BINARY, |e, a| {
            for word in [0u32, 4, 7] {
                e.mem.set_u32(a[0] + 4 * word, UNSET);
            }
            Ret::default()
        });
        e.call(0x00a3_f7b0, &args![this, stream]);
        assert_eq!(e.get(this, NiTransformInterpolator::bPose), 0);
        assert_eq!(calls_to(&e, 0x0300_0004), vec![vec![data]]);
    }

    #[test]
    fn constructor_takes_the_transform_words() {
        let mut e = engine();
        e.register(KEY_BASED_CONSTRUCT, |_, _| Ret::default());
        let this = e.new_object::<NiTransformInterpolator>();
        e.mem.write(this.addr(), &[0xee; 0x48]);
        let r = e
            .call(
                0x00a3_fae0,
                &args![this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32],
            )
            .u32();
        assert_eq!(r, this.addr());
        for word in 0..8 {
            assert_eq!(e.mem.u32(this.addr() + 0xc + 4 * word), word + 1);
        }
        assert_eq!(e.mem.u32(this.addr()), NI_TRANSFORM_INTERPOLATOR_VTABLE);
        assert_eq!(e.get(this, NiTransformInterpolator::m_spData), 0);
        assert_eq!(e.get(this, NiTransformInterpolator::bPose), 0);
        assert_eq!(e.mem.u16(this.addr() + 0x30), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x34), 0xeeee_0000);
    }

    /// Keys of `size` bytes whose times are `times`.
    fn keys(e: &mut Engine, times: &[f32], size: u32) -> u32 {
        let k = e.mem.alloc(size * times.len() as u32);
        for (i, t) in times.iter().enumerate() {
            set_f(e, k + size * i as u32, *t);
        }
        k
    }

    #[test]
    fn active_time_range_spans_every_key_list() {
        let mut e = engine();
        let this = e.new_object::<NiTransformInterpolator>();
        let begin = e.mem.alloc(4);
        let end = e.mem.alloc(4);
        // No data: [0, 0].
        e.call(0x00a3_fb20, &args![this, begin, end]);
        assert_eq!((e.mem.f32(begin), e.mem.f32(end)), (0.0, 0.0));
        let data = e.mem.alloc(0x30);
        e.set(this, NiTransformInterpolator::m_spData, data);
        let translation = keys(&mut e, &[1.0, 2.0, 3.0], 0x10);
        e.mem.set_u16(data + 0xa, 3);
        e.mem.set_u8(data + 0x1d, 0x10);
        e.mem.set_u32(data + 0x24, translation);
        let scale = keys(&mut e, &[0.5, 2.5], 8);
        e.mem.set_u16(data + 0xc, 2);
        e.mem.set_u8(data + 0x1e, 8);
        e.mem.set_u32(data + 0x28, scale);
        e.call(0x00a3_fb20, &args![this, begin, end]);
        assert_eq!((e.mem.f32(begin), e.mem.f32(end)), (0.5, 3.0));
        // Plain rotation keys.
        let rotation = keys(&mut e, &[-1.0, 9.0], 0x14);
        e.mem.set_u16(data + 0x8, 2);
        e.mem.set_u8(data + 0x1c, 0x14);
        e.mem.set_u32(data + 0x20, rotation);
        e.call(0x00a3_fb20, &args![this, begin, end]);
        assert_eq!((e.mem.f32(begin), e.mem.f32(end)), (-1.0, 9.0));
        // Euler keys: the three axis lists (the second one empty).
        let euler = e.mem.alloc(0x40);
        let x = keys(&mut e, &[-4.0, 4.0], 8);
        let z = keys(&mut e, &[0.0, 1.0, 12.0], 8);
        e.mem.set_u32(euler + 0x14, 2);
        e.mem.set_u32(euler + 0x1c, 3);
        e.mem.set_u8(euler + 0x2c, 8);
        e.mem.set_u8(euler + 0x2e, 8);
        e.mem.set_u32(euler + 0x30, x);
        e.mem.set_u32(euler + 0x38, z);
        e.mem.set_u32(data + 0x10, 4);
        e.mem.set_u32(data + 0x20, euler);
        e.call(0x00a3_fb20, &args![this, begin, end]);
        assert_eq!((e.mem.f32(begin), e.mem.f32(end)), (-4.0, 12.0));
        // Only empty lists: [0, 0].
        e.mem.set_u16(data + 0x8, 0);
        e.mem.set_u16(data + 0xa, 0);
        e.mem.set_u16(data + 0xc, 0);
        e.call(0x00a3_fb20, &args![this, begin, end]);
        assert_eq!((e.mem.f32(begin), e.mem.f32(end)), (0.0, 0.0));
    }

    #[test]
    fn update_poses_reuses_and_interpolates() {
        let mut e = engine();
        let this = e.new_object::<NiTransformInterpolator>();
        let out = e.mem.alloc(0x20);
        for word in 0..8 {
            e.mem
                .set_u32(this.addr() + 0xc + 4 * word, 0x3f80_0000 + word);
        }
        // A pose: the value copied out, the time taken.
        e.set(this, NiTransformInterpolator::bPose, 1);
        assert!(e.call(0x00a3_fdb0, &args![this, 2.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.u32(out + 0x1c), 0x3f80_0007);
        assert_eq!(e.mem.f32(this.addr() + 8), 2.0);
        // At the last time: copied out; answers 0 only with everything unset.
        e.set(this, NiTransformInterpolator::bPose, 0);
        assert!(e.call(0x00a3_fdb0, &args![this, 2.0f32, 0u32, out]).bool());
        unset_transform(&mut e, this);
        assert!(!e.call(0x00a3_fdb0, &args![this, 2.0f32, 0u32, out]).bool());
        // A new time: the key lists that have keys are interpolated.
        e.register(GET_TRANSLATION_KEYS, |e, a| {
            e.mem.set_u32(a[1], 2);
            e.mem.set_u32(a[2], 1);
            e.mem.set_u32(a[3], 0x10);
            0x7000u32.into_ret()
        });
        e.register(POS_KEY_GEN_INTERP, |e, a| {
            assert_eq!((a[2], a[3], a[4], a[6]), (0x7000, 1, 2, 0x10));
            assert_eq!(e.mem.u32(a[5]), 5);
            e.mem.set_u32(a[5], 6);
            for word in 0..3 {
                e.mem
                    .set_u32(a[0] + 4 * word, (10.0f32 + word as f32).to_bits());
            }
            a[0].into_ret()
        });
        e.register(GET_ROTATION_KEYS, |e, a| {
            e.mem.set_u32(a[1], 0);
            0u32.into_ret()
        });
        e.register(GET_SCALE_KEYS, |e, a| {
            e.mem.set_u32(a[1], 3);
            e.mem.set_u32(a[2], 2);
            e.mem.set_u32(a[3], 8);
            0x8000u32.into_ret()
        });
        e.register(FLOAT_KEY_GEN_INTERP, |e, a| {
            assert_eq!(f32::from_bits(a[0]), 3.0);
            assert_eq!((a[1], a[2], a[3], a[5]), (0x8000, 2, 3, 8));
            e.mem.set_u32(a[4], 1);
            Ret {
                st0: 1.5,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.set(this, NiTransformInterpolator::m_usLastTransIdx, 5);
        assert!(e.call(0x00a3_fdb0, &args![this, 3.0f32, 0u32, out]).bool());
        assert!(calls_to(&e, ROT_KEY_GEN_INTERP).is_empty());
        assert_eq!(e.mem.f32(this.addr() + 0xc), 10.0);
        assert_eq!(e.mem.f32(this.addr() + 0x14), 12.0);
        assert_eq!(e.get(this, NiTransformInterpolator::m_usLastTransIdx), 6);
        assert_eq!(e.mem.f32(this.addr() + 0x28), 1.5);
        assert_eq!(e.get(this, NiTransformInterpolator::m_usLastScaleIdx), 1);
        assert_eq!(e.mem.f32(out), 10.0);
        assert_eq!(e.mem.f32(this.addr() + 8), 3.0);
        // A new time without keys and everything unset: answers 0, the time not taken.
        e.register(GET_TRANSLATION_KEYS, |e, a| {
            e.mem.set_u32(a[1], 0);
            0u32.into_ret()
        });
        e.register(GET_SCALE_KEYS, |e, a| {
            e.mem.set_u32(a[1], 0);
            0u32.into_ret()
        });
        unset_transform(&mut e, this);
        assert!(!e.call(0x00a3_fdb0, &args![this, 4.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(this.addr() + 8), 3.0);
    }

    #[test]
    fn copy_members_copies_the_value_and_counts_the_data() {
        let mut e = engine();
        e.register(KEY_BASED_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiTransformInterpolator>();
        let clone = e.new_object::<NiTransformInterpolator>();
        for word in 0..14 {
            e.mem.set_u32(this.addr() + 0xc + 4 * word, 0x500 + word);
        }
        let data = counted(&mut e, 1);
        let old = counted(&mut e, 1);
        e.set(this, NiTransformInterpolator::m_spData, data);
        e.set(clone, NiTransformInterpolator::m_spData, old);
        e.set(this, NiTransformInterpolator::m_usLastTransIdx, 1);
        e.set(this, NiTransformInterpolator::m_usLastRotIdx, 2);
        e.set(this, NiTransformInterpolator::m_usLastScaleIdx, 3);
        e.call(0x00a4_0360, &args![this, clone, 0x99u32]);
        assert_eq!(
            calls_to(&e, KEY_BASED_COPY_MEMBERS),
            vec![vec![this.addr(), clone.addr(), 0x99]]
        );
        for word in 0..8 {
            assert_eq!(e.mem.u32(clone.addr() + 0xc + 4 * word), 0x500 + word);
        }
        assert_eq!(e.mem.u32(clone.addr() + 0x38), 0x50b);
        assert_eq!(e.mem.u32(clone.addr() + 0x40), 0x50d);
        assert_eq!(e.get(clone, NiTransformInterpolator::m_spData), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(calls_to(&e, 0x0300_0004), vec![vec![old]]);
        assert_eq!(e.get(clone, NiTransformInterpolator::m_usLastTransIdx), 1);
        assert_eq!(e.get(clone, NiTransformInterpolator::m_usLastRotIdx), 2);
        assert_eq!(e.get(clone, NiTransformInterpolator::m_usLastScaleIdx), 3);
    }

    #[test]
    fn create_object_00a403f0_builds_without_data() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTRUCT_FROM_DATA, |_, a| a[0].into_ret());
        let object = e.call(0x00a4_03f0, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, CONSTRUCT_FROM_DATA), vec![vec![object, 0]]);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x48]]);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_03f0, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, CONSTRUCT_FROM_DATA).len(), 1);
    }

    #[test]
    fn scalar_deleting_destructor_releases_the_data() {
        let mut e = engine();
        e.register(KEY_BASED_DESTRUCT, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = e.new_object::<NiTransformInterpolator>();
        let data = counted(&mut e, 1);
        e.set(this, NiTransformInterpolator::m_spData, data);
        assert_eq!(e.call(0x00a4_0410, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, 0x0300_0004), vec![vec![data]]);
        assert_eq!(calls_to(&e, KEY_BASED_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this.addr(), 0x48]]);
        e.set(this, NiTransformInterpolator::m_spData, 0);
        e.call(0x00a4_0410, &args![this, 0u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
    }

    #[test]
    fn create_clone_constructs_and_copies() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTRUCT_FROM_DATA, |_, a| a[0].into_ret());
        e.register(KEY_BASED_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiTransformInterpolator>();
        e.mem.set_u32(this.addr() + 0x10, 0x1234);
        let clone = e.call(0x00a4_0460, &args![this, 0x77u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone + 0x10), 0x1234);
        assert_eq!(
            calls_to(&e, KEY_BASED_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x77]]
        );
    }
}
