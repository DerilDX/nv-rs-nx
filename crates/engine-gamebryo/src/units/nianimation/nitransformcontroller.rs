//! `NiAnimation/nitransformcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBlendAccumTransformInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_BLEND_ACCUM_TRANSFORM_INTERPOLATOR_RTTI: u32 = 0x011f_371c;

/// `NiTransformController::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRANSFORM_CONTROLLER_RTTI: u32 = 0x011f_36ec;

// Translated from 00a36480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendAccumTransformInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_blend_accum_transform_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BLEND_ACCUM_TRANSFORM_INTERPOLATOR_RTTI)
}

// Translated from 00a36580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_transform_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRANSFORM_CONTROLLER_RTTI)
}

// Translated from 00a36490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendAccumTransformInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a39290`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x58 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_blend_accum_transform_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a3_9290;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x58u32]);
    }
    this
}

/// The imported `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiSingleInterpController::m_spInterpolator` (Xbox PDB +0x34, the same on PC) and
/// `NiTimeController::m_pkTarget` (+0x2C), `m_fLastTime`-like float +0x28 (by the PDB's
/// `NiTimeController` order; the code only stores and reads it).
const INTERPOLATOR: u32 = 0x34;
const TARGET: u32 = 0x2c;
const LAST_SCALED_TIME: u32 = 0x28;
/// The game's `operator new` (`cdecl`, the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The memory manager singleton and its `Allocate` (`thiscall`, the size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The compiler's array constructor iterator (`stdcall`, (array, element size, count,
/// constructor)) and the accumulation item constructor it is given.
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
const ACCUM_ITEM_CONSTRUCT: u32 = 0x00a3_62f0;
/// The base destructor `00a6d290` (unit `NiMain/nitimecontroller.cpp`) `fn_00a360d0`
/// tail-jumps to.
const TIME_CONTROLLER_DESTRUCT: u32 = 0x00a6_d290;
/// `NiQuaternion::FromRotation` (`00a6df40`) and `NiQuaternion::ToRotation` (`004f0180`;
/// both `thiscall` on the quaternion, the matrix address).
const QUATERNION_FROM_ROTATION: u32 = 0x00a6_df40;
const QUATERNION_TO_ROTATION: u32 = 0x004f_0180;
/// `NiTransformInterpolator::NiTransformInterpolator` (`00a3fae0`, a `NiQuatTransform` by
/// value, eight words) and the constructor from a data object (`00a3fa30`, one word).
const TRANSFORM_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_fae0;
const TRANSFORM_INTERPOLATOR_FROM_DATA: u32 = 0x00a3_fa30;
/// `NiSingleInterpController::LoadBinary` (`00a38f80`) and `LinkObject` (`00a35850`, a jump
/// thunk), both one word.
const SINGLE_LOAD_BINARY: u32 = 0x00a3_8f80;
const SINGLE_LINK_OBJECT: u32 = 0x00a3_5850;
/// `00a645b0` (`thiscall` on the stream, no stack words) and
/// `NiStream::GetObjectFromLinkID` (`00a63430`).
const STREAM_READ_LINK: u32 = 0x00a6_45b0;
const STREAM_GET_OBJECT_FROM_LINK: u32 = 0x00a6_3430;
/// The stream's file version word (+0xD8) and the version below which the old data
/// link is read.
const STREAM_VERSION: u32 = 0xd8;
const LINK_VERSION: u32 = 0x0a01_0068;
/// `NiBlendInterpolator::NiBlendInterpolator` (`00a370b0`; (flag, float, count)), the
/// `NiBlendAccumTransformInterpolator` vtable (RTTI) and the other blend interpolator's
/// constructor (`00a40980`, the same three words).
const BLEND_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_70b0;
const BLEND_ACCUM_VTABLE: u32 = 0x0109_6784;
const BLEND_TRANSFORM_CONSTRUCT: u32 = 0x00a4_0980;
/// The constant words copied into a fresh `NiQuatTransform`: the translation
/// (`011a8400`), the rotation (`011f3704`) and the scale float (`01096b8c`, -FLT_MAX in the
/// exe); the double the update compares against (`010241b0`, -FLT_MAX in the exe) and the
/// float stored for a disabled controller (`01096324`).
const INVALID_TRANSLATE: u32 = 0x011a_8400;
const INVALID_ROTATE: u32 = 0x011f_3704;
const INVALID_SCALE: u32 = 0x0109_6b8c;
const INVALID_DOUBLE: u32 = 0x0102_41b0;
const DISABLED_TIME: u32 = 0x0109_6324;
/// `NiTimeController::DontDoUpdate` (`00a36250`; `thiscall`, one float).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// The `NiTransformController` vtable (RTTI), its base constructor (`00a38e00`, by its
/// body the `NiSingleInterpController` constructor; the engine map's name for it is folded
/// code) and the members copy (`00a38ef0`, `thiscall`, (clone, process); folded name too).
const TRANSFORM_CONTROLLER_VTABLE: u32 = 0x0109_687c;
const SINGLE_INTERP_CONSTRUCT: u32 = 0x00a3_8e00;
const COPY_MEMBERS: u32 = 0x00a3_8ef0;

/// The inlined `NiPointer` release of `object` (`InterlockedDecrement` of its count at +4;
/// `DeleteThis`, vtable slot +4, at zero).
fn release(e: &mut Engine, object: u32) {
    if object != 0 && e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 0x4, &[]);
    }
}

// Translated from 00a360d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function name; `thiscall`, no stack words; a destructor body):
/// releases `m_spInterpolator` (+0x34), then tail-jumps to the base destructor `00a6d290`.
pub fn fn_00a360d0(e: &mut Engine, this: u32) {
    let interpolator = e.mem.u32(this + INTERPOLATOR);
    release(e, interpolator);
    e.call(TIME_CONTROLLER_DESTRUCT, &args![this]);
}

// Translated from 00a36100 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiTransformController::CreatePoseInterpolator` (Xbox PDB; `thiscall`, one word it does
/// not read, `RET 4`): a `NiTransformInterpolator` (0x48 bytes from `operator new`, null
/// when that fails) made from the target's local transform: its translation (+0x58), its
/// rotation (+0x34) as a quaternion (`NiQuaternion::FromRotation`) and its scale (+0x64,
/// copied through the x87 stack), passed by value.
pub fn ni_transform_controller_create_pose_interpolator(
    e: &mut Engine,
    this: u32,
    _unused_1: u32,
) -> u32 {
    let target = e.mem.u32(this + TARGET);
    e.with_stack(0x10, |e, quaternion| {
        let quaternion = quaternion.addr();
        e.call(QUATERNION_FROM_ROTATION, &args![quaternion, target + 0x34]);
        let scale = x87_quiet_bits(e.mem.u32(target + 0x64));
        let translate = [
            e.mem.u32(target + 0x58),
            e.mem.u32(target + 0x5c),
            e.mem.u32(target + 0x60),
        ];
        let rotate = [
            e.mem.u32(quaternion),
            e.mem.u32(quaternion + 4),
            e.mem.u32(quaternion + 8),
            e.mem.u32(quaternion + 0xc),
        ];
        let block = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
        if block == 0 {
            return 0;
        }
        e.call(
            TRANSFORM_INTERPOLATOR_CONSTRUCT,
            &args![
                block,
                translate[0],
                translate[1],
                translate[2],
                rotate[0],
                rotate[1],
                rotate[2],
                rotate[3],
                scale
            ],
        )
        .u32()
    })
}

// Translated from 00a36190 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiTransformController::LoadBinary` (Xbox PDB; `thiscall`, one word, `RET 4`):
/// `NiSingleInterpController::LoadBinary`, then for a file older than 10.1.0.104 the old
/// data link (`00a645b0` on the stream).
pub fn ni_transform_controller_load_binary(e: &mut Engine, this: u32, stream: u32) {
    e.call(SINGLE_LOAD_BINARY, &args![this, stream]);
    if e.mem.u32(stream + STREAM_VERSION) < LINK_VERSION {
        e.call(STREAM_READ_LINK, &args![stream]);
    }
}

// Translated from 00a361c0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiTransformController::LinkObject` (Xbox PDB; `thiscall`, one word, `RET 4`):
/// `NiSingleInterpController::LinkObject`, then for a file older than 10.1.0.104 a
/// `NiTransformInterpolator` (0x48 bytes, `00a3fa30` with the linked data object; null when
/// the allocation fails) becomes `m_spInterpolator` (the old one released, the new one
/// counted up when they differ) and gets its vtable slot +0xBC call.
pub fn ni_transform_controller_link_object(e: &mut Engine, this: u32, stream: u32) {
    e.call(SINGLE_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(stream + STREAM_VERSION) >= LINK_VERSION {
        return;
    }
    let data = e.call(STREAM_GET_OBJECT_FROM_LINK, &args![stream]).u32();
    let block = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
    let interpolator = if block != 0 {
        e.call(TRANSFORM_INTERPOLATOR_FROM_DATA, &args![block, data])
            .u32()
    } else {
        0
    };
    let old = e.mem.u32(this + INTERPOLATOR);
    if old != interpolator {
        release(e, old);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        if interpolator != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![interpolator + 4]);
        }
    }
    let interpolator = e.mem.u32(this + INTERPOLATOR);
    e.vcall(interpolator, 0xbc, &[]);
}

// Translated from 00a363b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (by its vtable the `NiBlendAccumTransformInterpolator` constructor (RTTI);
/// `thiscall`, three words, `RET 0xC`): `NiBlendInterpolator::NiBlendInterpolator(flag,
/// threshold, count)` (the float copied through the x87 stack), the vtable,
/// `m_kAccumulatedTransformValue` (+0x30) the invalid transform (`011a8400`, `011f3704`,
/// the scale `01096b8c`), no `m_pkAccumArray` (+0x50), `m_bReset` (+0x54) false; then, for
/// a non-zero `m_ucArraySize` (+0xD), an array of that many 0x68-byte items from the memory
/// manager (the size all ones on overflow, no count word) built by `00a362f0`
/// (0 when the allocation fails). Answers `this`.
pub fn fn_00a363b0(e: &mut Engine, this: u32, flag: u32, threshold: u32, count: u32) -> u32 {
    e.call(
        BLEND_INTERPOLATOR_CONSTRUCT,
        &args![this, flag, x87_quiet_bits(threshold), count],
    );
    e.mem.set_u32(this, BLEND_ACCUM_VTABLE);
    for i in 0..3 {
        let word = e.mem.u32(INVALID_TRANSLATE + i * 4);
        e.mem.set_u32(this + 0x30 + i * 4, word);
    }
    for i in 0..4 {
        let word = e.mem.u32(INVALID_ROTATE + i * 4);
        e.mem.set_u32(this + 0x3c + i * 4, word);
    }
    let scale = x87_quiet_bits(e.mem.u32(INVALID_SCALE));
    e.mem.set_u32(this + 0x4c, scale);
    let size = e.mem.u8(this + 0xd);
    e.mem.set_u32(this + 0x50, 0);
    e.mem.set_u8(this + 0x54, 0);
    if size == 0 {
        return this;
    }
    let items = u32::from(size);
    let bytes = items.checked_mul(0x68).unwrap_or(u32::MAX);
    let array = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    if array != 0 {
        e.call(
            VECTOR_CONSTRUCT,
            &args![array, 0x68u32, items, ACCUM_ITEM_CONSTRUCT],
        );
    }
    e.mem.set_u32(this + 0x50, array);
    this
}

// Translated from 00a36620 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiTransformController::CreateBlendInterpolator` (Xbox PDB; `thiscall`, five words,
/// `RET 0x14`; the first is not read): with `accumulate` (a byte) a 0x58-byte
/// `NiBlendAccumTransformInterpolator` (`fn_00a363b0`), else a 0x30-byte blend interpolator
/// built by `00a40980`, each from `(flag, threshold, count)` (the float copied through the
/// x87 stack); null when `operator new` fails.
pub fn ni_transform_controller_create_blend_interpolator(
    e: &mut Engine,
    _this: u32,
    _unused_1: u32,
    flag: u32,
    accumulate: u32,
    threshold: u32,
    count: u32,
) -> u32 {
    let threshold = x87_quiet_bits(threshold);
    if accumulate as u8 != 0 {
        let block = e.call(OPERATOR_NEW, &args![0x58u32]).u32();
        if block == 0 {
            return 0;
        }
        return fn_00a363b0(e, block, flag, threshold, count);
    }
    let block = e.call(OPERATOR_NEW, &args![0x30u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        BLEND_TRANSFORM_CONSTRUCT,
        &args![block, flag, threshold, count],
    )
    .u32()
}

// Translated from 00a36680 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiTransformController::Update` (Xbox PDB; `thiscall`, one word: the update data, whose
/// first float is the time; `RET 4`). A controller whose flag bit 5 (+0x8) is set stores
/// the float at `01096324` at +0x28 and goes on; otherwise, when `DontDoUpdate(time)`
/// answers true, it goes on only with an interpolator whose vtable slot +0xD4 answers true
/// (and with false straight on). With an interpolator: its `Update` (slot +0x8C) with the
/// float +0x28, the target and an invalid `NiQuatTransform`; when that succeeds, each part
/// that is no longer the invalid -FLT_MAX (the double at `010241b0`; `FUCOM`, so a NaN
/// counts as set) is written to the target: the translation (+0x58), the rotation (+0x34,
/// `NiQuaternion::ToRotation`) and the absolute scale (+0x64).
pub fn ni_transform_controller_update(e: &mut Engine, this: u32, update_data: u32) {
    if (e.mem.u8(this + 0x8) >> 5) & 1 != 0 {
        let time = x87_quiet_bits(e.mem.u32(DISABLED_TIME));
        e.mem.set_u32(this + LAST_SCALED_TIME, time);
    } else {
        let time = x87_quiet_bits(e.mem.u32(update_data));
        if e.call(DONT_DO_UPDATE, &args![this, time]).bool() {
            let interpolator = e.mem.u32(this + INTERPOLATOR);
            if interpolator == 0 || !e.vcall(interpolator, 0xd4, &[]).bool() {
                return;
            }
        }
    }
    let interpolator = e.mem.u32(this + INTERPOLATOR);
    if interpolator == 0 {
        return;
    }
    e.with_stack(0x20, |e, value| {
        let value = value.addr();
        let scale = x87_quiet_bits(e.mem.u32(INVALID_SCALE));
        e.mem.set_u32(value + 0x1c, scale);
        let time = x87_quiet_bits(e.mem.u32(this + LAST_SCALED_TIME));
        for i in 0..3 {
            let word = e.mem.u32(INVALID_TRANSLATE + i * 4);
            e.mem.set_u32(value + i * 4, word);
        }
        for i in 0..4 {
            let word = e.mem.u32(INVALID_ROTATE + i * 4);
            e.mem.set_u32(value + 0xc + i * 4, word);
        }
        let target = e.mem.u32(this + TARGET);
        if !e
            .vcall(interpolator, 0x8c, &args![time, target, value])
            .bool()
        {
            return;
        }
        let target = e.mem.u32(this + TARGET);
        let invalid = e.mem.f64(INVALID_DOUBLE);
        // FUCOM; TEST AH,0x44; JNP: only an ordered equal skips.
        if x87_load32(e.mem.u32(value)) != invalid {
            for i in 0..3 {
                let word = e.mem.u32(value + i * 4);
                e.mem.set_u32(target + 0x58 + i * 4, word);
            }
        }
        if x87_load32(e.mem.u32(value + 0x10)) != invalid {
            e.call(QUATERNION_TO_ROTATION, &args![value + 0xc, target + 0x34]);
        }
        let scale = x87_load32(e.mem.u32(value + 0x1c));
        if scale != invalid {
            let absolute = x87_store32(scale.abs());
            e.mem.set_u32(target + 0x64, x87_quiet_bits(absolute));
        }
    });
}

// Translated from 00a367c0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiTransformController::CreateClone` (Xbox PDB; `thiscall`, one word, `RET 4`): a new
/// 0x38-byte controller (`operator new`, `00a38e00`, the `NiTransformController` vtable)
/// given the members (`00a38ef0(clone, process)`, also run with a null clone when the
/// allocation fails). Answers the clone.
pub fn ni_transform_controller_create_clone(e: &mut Engine, this: u32, process: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x38u32]).u32();
    if block != 0 {
        e.call(SINGLE_INTERP_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, TRANSFORM_CONTROLLER_VTABLE);
    }
    e.call(COPY_MEMBERS, &args![this, block, process]);
    block
}

// Translated from 00a36810 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; by its body the creation function of
/// `NiTransformController`; no arguments): a new 0x38-byte controller (`operator new`,
/// `00a38e00`, the vtable `0109687c`), or null.
pub fn fn_00a36810(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x38u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(SINGLE_INTERP_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, TRANSFORM_CONTROLLER_VTABLE);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a36480, ni_blend_accum_transform_interpolator_get_rtti() -> Ptr),
        entry!(0x00a36580, ni_transform_controller_get_rtti() -> Ptr),
        entry!(0x00a36490, ni_blend_accum_transform_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a3_60d0, fn_00a360d0(u32)),
        entry!(
            0x00a3_6100,
            ni_transform_controller_create_pose_interpolator(u32, u32) -> u32
        ),
        entry!(0x00a3_6190, ni_transform_controller_load_binary(u32, u32)),
        entry!(0x00a3_61c0, ni_transform_controller_link_object(u32, u32)),
        entry!(0x00a3_63b0, fn_00a363b0(u32, u32, u32, u32) -> u32),
        entry!(
            0x00a3_6620,
            ni_transform_controller_create_blend_interpolator(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00a3_6680, ni_transform_controller_update(u32, u32)),
        entry!(
            0x00a3_67c0,
            ni_transform_controller_create_clone(u32, u32) -> u32
        ),
        entry!(0x00a3_6810, fn_00a36810() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_blend_accum_transform_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a36480, &args![]).u32(),
            NI_BLEND_ACCUM_TRANSFORM_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_transform_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a36580, &args![]).u32(),
            NI_TRANSFORM_CONTROLLER_RTTI
        );
    }

    #[test]
    fn ni_blend_accum_transform_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a3_9290, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a36490, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a36490, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x58]]);
        assert_eq!(calls_to(&e, 0x00a3_9290), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b018_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object whose vtable slots are `0x0097_0000 + slot`, each a double answering 0.
    fn object(e: &mut Engine, size: u32) -> u32 {
        let table = e.mem.alloc(0x100);
        for slot in (0..0x100).step_by(4) {
            e.mem.set_u32(table + slot, 0x0097_0000 + slot);
            e.register(0x0097_0000 + slot, |_, _| Ret::default());
        }
        let this = e.mem.alloc(size);
        e.mem.set_u32(this, table);
        this
    }

    fn counts(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
    }

    #[test]
    fn fn_00a360d0_releases_then_destroys_the_base() {
        let mut e = Engine::new();
        counts(&mut e);
        e.register(TIME_CONTROLLER_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        let interpolator = object(&mut e, 8);
        e.mem.set_u32(interpolator + 4, 1);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.call_log = Some(vec![]);
        e.call(0x00a3_60d0, &[this]);
        assert_eq!(calls_to(&e, 0x0097_0004), vec![vec![interpolator]]);
        assert_eq!(calls_to(&e, TIME_CONTROLLER_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn create_pose_interpolator_passes_the_local_transform() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x38);
        let target = e.mem.alloc(0x70);
        e.mem.set_u32(this + TARGET, target);
        e.mem.set_u32(target + 0x58, 1);
        e.mem.set_u32(target + 0x5c, 2);
        e.mem.set_u32(target + 0x60, 3);
        e.mem.set_u32(target + 0x64, 0x7f80_0001);
        e.register(QUATERNION_FROM_ROTATION, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + i * 4, 10 + i);
            }
            Ret::default()
        });
        e.register(OPERATOR_NEW, |_, _| 0x6000u32.into_ret());
        e.register(TRANSFORM_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a3_6100, &[this, 0]).u32(), 0x6000);
        assert_eq!(
            calls_to(&e, TRANSFORM_INTERPOLATOR_CONSTRUCT),
            vec![vec![0x6000, 1, 2, 3, 10, 11, 12, 13, 0x7fc0_0001]]
        );
        assert_eq!(calls_to(&e, QUATERNION_FROM_ROTATION)[0][1], target + 0x34);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a3_6100, &[this, 0]).u32(), 0);
    }

    #[test]
    fn load_binary_and_link_object_read_the_old_link() {
        let mut e = Engine::new();
        counts(&mut e);
        e.register(SINGLE_LOAD_BINARY, |_, _| Ret::default());
        e.register(SINGLE_LINK_OBJECT, |_, _| Ret::default());
        e.register(STREAM_READ_LINK, |_, _| Ret::default());
        e.register(STREAM_GET_OBJECT_FROM_LINK, |_, _| 0x5500u32.into_ret());
        let interpolator = object(&mut e, 8);
        e.register_double(OPERATOR_NEW, move |_, _| interpolator.into_ret());
        e.register(TRANSFORM_INTERPOLATOR_FROM_DATA, |_, a| a[0].into_ret());
        let stream = e.mem.alloc(0x100);
        let this = e.mem.alloc(0x38);
        let old = object(&mut e, 8);
        e.mem.set_u32(old + 4, 1);
        e.mem.set_u32(this + INTERPOLATOR, old);
        // A current file: nothing more.
        e.mem.set_u32(stream + STREAM_VERSION, LINK_VERSION);
        e.call_log = Some(vec![]);
        e.call(0x00a3_6190, &[this, stream]);
        e.call(0x00a3_61c0, &[this, stream]);
        assert!(calls_to(&e, STREAM_READ_LINK).is_empty());
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        // An old file.
        e.mem.set_u32(stream + STREAM_VERSION, LINK_VERSION - 1);
        e.call_log = Some(vec![]);
        e.call(0x00a3_6190, &[this, stream]);
        assert_eq!(calls_to(&e, STREAM_READ_LINK), vec![vec![stream]]);
        e.call(0x00a3_61c0, &[this, stream]);
        assert_eq!(
            calls_to(&e, TRANSFORM_INTERPOLATOR_FROM_DATA),
            vec![vec![interpolator, 0x5500]]
        );
        assert_eq!(e.mem.u32(this + INTERPOLATOR), interpolator);
        assert_eq!(e.mem.u32(interpolator + 4), 1);
        assert_eq!(calls_to(&e, 0x0097_0004), vec![vec![old]]);
        assert_eq!(calls_to(&e, 0x0097_00bc), vec![vec![interpolator]]);
    }

    #[test]
    fn blend_interpolators_are_made_by_kind() {
        let mut e = Engine::new();
        e.map(0x011a_8000, 0x1000);
        e.map(0x011f_3000, 0x1000);
        e.map(0x0109_6000, 0x1000);
        for i in 0..3 {
            e.mem.set_u32(INVALID_TRANSLATE + i * 4, 0x100 + i);
        }
        for i in 0..4 {
            e.mem.set_u32(INVALID_ROTATE + i * 4, 0x200 + i);
        }
        e.mem.set_u32(INVALID_SCALE, 0xff7f_ffff);
        e.register(BLEND_INTERPOLATOR_CONSTRUCT, |e, a| {
            e.mem.set_u8(a[0] + 0xd, a[3] as u8);
            Ret::default()
        });
        e.register(BLEND_TRANSFORM_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMORY_ALLOCATE, |_, _| 0x7000u32.into_ret());
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let accum = e.call(0x00a3_6620, &[0, 9, 1, 1, 0x7f80_0002, 3]).u32();
        assert_eq!(
            calls_to(&e, BLEND_INTERPOLATOR_CONSTRUCT),
            vec![vec![accum, 1, 0x7fc0_0002, 3]]
        );
        assert_eq!(e.mem.u32(accum), BLEND_ACCUM_VTABLE);
        assert_eq!(e.mem.u32(accum + 0x30), 0x100);
        assert_eq!(e.mem.u32(accum + 0x48), 0x203);
        assert_eq!(e.mem.u32(accum + 0x4c), 0xff7f_ffff);
        assert_eq!(e.mem.u32(accum + 0x50), 0x7000);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 3 * 0x68]]
        );
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![0x7000, 0x68, 3, ACCUM_ITEM_CONSTRUCT]]
        );
        // No items: no array.
        e.call_log = Some(vec![]);
        let empty = e.call(0x00a3_6620, &[0, 9, 1, 1, 0, 0]).u32();
        assert_eq!(e.mem.u32(empty + 0x50), 0);
        assert!(calls_to(&e, MEMORY_ALLOCATE).is_empty());
        // Not accumulating: the other blend interpolator.
        e.call_log = Some(vec![]);
        let other = e.call(0x00a3_6620, &[0, 9, 0, 0, 0, 2]).u32();
        assert_eq!(
            calls_to(&e, BLEND_TRANSFORM_CONSTRUCT),
            vec![vec![other, 0, 0, 2]]
        );
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x30]]);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a3_6620, &[0, 9, 0, 1, 0, 2]).u32(), 0);
    }

    #[test]
    fn update_writes_the_parts_that_were_set() {
        let mut e = Engine::new();
        e.map(0x011a_8000, 0x1000);
        e.map(0x011f_3000, 0x1000);
        e.map(0x0109_6000, 0x1000);
        e.map(0x0102_4000, 0x1000);
        e.mem.set_u32(INVALID_SCALE, 0xff7f_ffff);
        e.mem.set_f32(INVALID_TRANSLATE, -f32::MAX);
        e.mem.set_f32(INVALID_ROTATE + 4, -f32::MAX);
        e.mem.set_f64(INVALID_DOUBLE, -f64::from(f32::MAX));
        e.mem.set_f32(DISABLED_TIME, 4.0);
        let this = e.mem.alloc(0x38);
        let target = e.mem.alloc(0x70);
        e.mem.set_u32(this + TARGET, target);
        let interpolator = object(&mut e, 8);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.register(QUATERNION_TO_ROTATION, |_, _| Ret::default());
        e.register(DONT_DO_UPDATE, |_, _| Ret::default());
        // The interpolator update sets the translation and a negative scale only.
        e.register(0x0097_008c, |e, a| {
            e.mem.set_f32(a[3], 1.0);
            e.mem.set_f32(a[3] + 0x1c, -2.0);
            1u32.into_ret()
        });
        let data = e.mem.alloc(4);
        e.mem.set_f32(data, 3.0);
        e.call_log = Some(vec![]);
        e.call(0x00a3_6680, &[this, data]);
        assert_eq!(e.mem.f32(target + 0x58), 1.0);
        assert_eq!(e.mem.f32(target + 0x64), 2.0);
        assert!(calls_to(&e, QUATERNION_TO_ROTATION).is_empty());
        assert_eq!(
            calls_to(&e, DONT_DO_UPDATE),
            vec![vec![this, 3.0f32.to_bits()]]
        );
        // Disabled (flag bit 5): the time comes from 01096324, no DontDoUpdate.
        e.mem.set_u8(this + 8, 0x20);
        e.register(0x0097_008c, |e, a| {
            e.mem.set_f32(a[3] + 0x10, 0.5);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a3_6680, &[this, data]);
        assert!(calls_to(&e, DONT_DO_UPDATE).is_empty());
        assert_eq!(calls_to(&e, 0x0097_008c)[0][1], 4.0f32.to_bits());
        let rotations = calls_to(&e, QUATERNION_TO_ROTATION);
        assert_eq!(rotations.len(), 1);
        assert_eq!(rotations[0][1], target + 0x34);
        // DontDoUpdate true and slot +0xD4 false: nothing.
        e.mem.set_u8(this + 8, 0);
        e.register(DONT_DO_UPDATE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00a3_6680, &[this, data]);
        assert!(calls_to(&e, 0x0097_008c).is_empty());
        assert_eq!(calls_to(&e, 0x0097_00d4).len(), 1);
    }

    #[test]
    fn create_clone_and_create_object() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SINGLE_INTERP_CONSTRUCT, |_, _| Ret::default());
        e.register(COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a3_67c0, &[0x4000, 0x4100]).u32();
        assert_eq!(e.mem.u32(clone), TRANSFORM_CONTROLLER_VTABLE);
        assert_eq!(
            calls_to(&e, COPY_MEMBERS),
            vec![vec![0x4000, clone, 0x4100]]
        );
        let made = e.call(0x00a3_6810, &[]).u32();
        assert_eq!(e.mem.u32(made), TRANSFORM_CONTROLLER_VTABLE);
        assert_eq!(
            calls_to(&e, SINGLE_INTERP_CONSTRUCT),
            vec![vec![clone], vec![made]]
        );
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a3_67c0, &[0x4000, 0x4100]).u32(), 0);
        assert_eq!(calls_to(&e, COPY_MEMBERS), vec![vec![0x4000, 0, 0x4100]]);
        assert_eq!(e.call(0x00a3_6810, &[]).u32(), 0);
    }
}
