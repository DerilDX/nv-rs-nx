//! `NiAnimation/nipathinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPosData::ms_RTTI` (the class's `NiRTTI` record).
const NI_POS_DATA_RTTI: u32 = 0x011f_3f68;

/// `NiPathInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_PATH_INTERPOLATOR_RTTI: u32 = 0x011f_3e48;

// Translated from 00a3dab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_pos_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_POS_DATA_RTTI)
}

// Translated from 00a3dde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_path_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_PATH_INTERPOLATOR_RTTI)
}

// Translated from 00a3dac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a48fb0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x18 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_pos_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a4_8fb0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00a3e980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a3dd40`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x5c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_path_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a3_dd40;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x5cu32]);
    }
    this
}

/// `NiPathInterpolator`'s vtable (written by its constructor and destructor).
const NI_PATH_INTERPOLATOR_VTABLE: u32 = 0x0109_720c;
/// `NiKeyBasedInterpolator`'s constructor (`thiscall`, no arguments) and destructor body
/// (`thiscall`).
const KEY_BASED_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a3_ca10;
const KEY_BASED_INTERPOLATOR_DESTRUCTOR: u32 = 0x00a3_ca30;
/// `NiKeyBasedInterpolator::LoadBinary` (`thiscall`, `(stream)`), its `LinkObject`
/// (`00a95ca0`, `(stream)`; the map's name for that address is a folded one) and its
/// `CopyMembers` (`00aa0ab0`, `(destination, cloning)`).
const KEY_BASED_INTERPOLATOR_LOAD_BINARY: u32 = 0x00aa_0ac0;
const KEY_BASED_INTERPOLATOR_LINK_OBJECT: u32 = 0x00a9_5ca0;
const KEY_BASED_INTERPOLATOR_COPY_MEMBERS: u32 = 0x00aa_0ab0;
/// `NiObject::ProcessClone` (`thiscall`, `(cloning)`).
const NI_OBJECT_PROCESS_CLONE: u32 = 0x00a5_d430;
/// `NiStream::GetObjectFromLinkID` (`thiscall` on the stream, no arguments) and the link-ID
/// read `00a645b0` (`thiscall` on the stream, no arguments).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiPathInterpolator::CalcRefFrame` (`thiscall`, `(first, second, ratio, &matrix)`) and
/// `NiQuaternion::FromRotation` (`thiscall` on the quaternion, `(&matrix)`).
const CALC_REF_FRAME: u32 = 0x00a3_de50;
const QUATERNION_FROM_ROTATION: u32 = 0x00a6_df40;
/// `NiFloatKey::GenInterp` (`cdecl`, `(time, keys, key type, key count, &last index, key
/// size)`, the value in ST0).
const FLOAT_KEY_GEN_INTERP: u32 = 0x00a2_6b40;
/// `NiPosKey`'s per-key-type function tables (indexed by `NiPosData`'s key type): the
/// interpolation (`011f3ca8`, `cdecl (ratio, key, next key, &point)`), the derivative
/// (`011f3c18`, same arguments) and the one `LinkObject` stores as the maximum curvature
/// (`011f3d38`, `cdecl (keys, key count)`, the value in ST0).
const POS_KEY_INTERPOLATIONS: u32 = 0x011f_3ca8;
const POS_KEY_DERIVATIVES: u32 = 0x011f_3c18;
const POS_KEY_CURVATURES: u32 = 0x011f_3d38;
/// The allocator's table allocation and free (`cdecl`, one word), `operator new` (`cdecl`,
/// `(size)`), `memcpy` (`cdecl`), the CRT square root `_CIsqrt` (argument and result in ST0)
/// and `_CIfmod` (the dividend in ST1, the divisor in ST0, both passed here as doubles).
const TABLE_ALLOCATE: u32 = 0x00aa_1070;
const TABLE_FREE: u32 = 0x00aa_10f0;
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const MEMCPY: u32 = 0x00ec_44d0;
const CRT_SQRT: u32 = 0x00ec_6040;
const CI_FMOD: u32 = 0x00ec_913a;
/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiQuaternion::IDENTITY` (four floats), `NiInterpolator::INVALID_POINT3` (three floats),
/// `-1.0f`, `-FLT_MAX` (`NiInterpolator::INVALID_FLOAT`'s bits) and the `double` tolerance
/// `0.0001` of the arc-length search.
const QUATERNION_IDENTITY: u32 = 0x011f_4208;
const INVALID_POINT3: u32 = 0x011a_9424;
const MINUS_ONE: u32 = 0x0101_2054;
const LOWEST_FLOAT: u32 = 0x0101_5f5c;
const ARC_LENGTH_TOLERANCE: u32 = 0x0103_2980;
/// The five Gauss-Legendre abscissas (on [0, 1]) and weights of the arc-length integral.
const GAUSS_ABSCISSAS: u32 = 0x011a_854c;
const GAUSS_WEIGHTS: u32 = 0x011a_8538;

layout! {
    /// `NiPathInterpolator` (Xbox PDB), 0x5C bytes (offsets checked against the PC code).
    pub struct NiPathInterpolator: 0x5C {
        /// `NiInterpolator::m_fLastTime` (Xbox PDB), float bits.
        0x08 m_fInterpolatorLastTime: u32,
        /// `m_uFlags` (Xbox PDB): bit 0 "lengths stale", bit 4 constant speed, bit 5 follow.
        0x0C m_uFlags: u16,
        /// `m_uiLastPathIdx` (Xbox PDB).
        0x10 m_uiLastPathIdx: u32,
        /// `m_uiLastPctIdx` (Xbox PDB).
        0x14 m_uiLastPctIdx: u32,
        /// `m_spPathData` (Xbox PDB): `NiPosData*`.
        0x18 m_spPathData: u32,
        /// `m_spPctData` (Xbox PDB): `NiFloatData*`.
        0x1C m_spPctData: u32,
        /// `m_pfPartialLength` (Xbox PDB): one float per key.
        0x20 m_pfPartialLength: u32,
        /// `m_fTotalLength` (Xbox PDB), float bits.
        0x24 m_fTotalLength: u32,
        /// `m_fMaxBankAngle` (Xbox PDB), float bits.
        0x28 m_fMaxBankAngle: u32,
        /// `m_fSmoothing` (Xbox PDB), float bits.
        0x2C m_fSmoothing: u32,
        /// `m_sFollowAxis` (Xbox PDB).
        0x30 m_sFollowAxis: u16,
        /// `m_fMaxCurvature` (Xbox PDB), float bits.
        0x34 m_fMaxCurvature: u32,
        /// `m_eBankDir` (Xbox PDB).
        0x38 m_eBankDir: u32,
        /// `m_kRotation` (Xbox PDB): `NiQuaternion`, four floats.
        0x3C m_kRotation: u32,
        /// `m_kPosition` (Xbox PDB): `NiPoint3`, three floats.
        0x4C m_kPosition: u32,
        /// `m_fLastTime` (Xbox PDB), float bits.
        0x58 m_fLastTime: u32,
    }
}

/// `NiPosData` (Xbox PDB) fields read here: `m_uiNumKeys` +0x08, `m_pkKeys` +0x0C,
/// `m_eType` +0x10, `m_ucKeySize` +0x14. `NiFloatData` has the same four at the same offsets.
const KEY_DATA_NUM_KEYS: u32 = 0x08;
const KEY_DATA_KEYS: u32 = 0x0C;
const KEY_DATA_TYPE: u32 = 0x10;
const KEY_DATA_KEY_SIZE: u32 = 0x14;

/// Releases a counted object (count at +4): when the decrement reaches 0, its virtual slot
/// +4 (`DeleteThis`) runs.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// The smart-pointer assignment of a counted object to the pointer at `slot`: nothing when
/// it is already there; otherwise the old one is released, `new` stored and counted.
fn assign_counted(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old == new {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(slot, new);
    if new != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
    }
}

/// Reads `size` bytes from the stream's input (`NiStream::m_pkIstr` +0x24C) into
/// `destination`: the input's read function (`NiBinaryStream::m_pfnRead` +0x08) is called
/// as `function(input, destination, size, &count, 1)` with `count` set to `size`, and the
/// byte count it answers is added to the input's position (+0x04).
fn stream_read(e: &mut Engine, stream: Ptr, destination: u32, size: u32) {
    let input = e.mem.u32(stream.addr().wrapping_add(0x24c));
    let function = e.mem.u32(input.wrapping_add(8));
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), size);
        e.call(function, &args![input, destination, size, count, 1u32])
            .u32()
    });
    let position = e.mem.u32(input.wrapping_add(4));
    e.mem
        .set_u32(input.wrapping_add(4), position.wrapping_add(done));
}

/// The path data's key type, key size and keys (all 0 without path data).
fn path_keys(e: &Engine, this: Ptr<NiPathInterpolator>) -> (u32, u32, u32) {
    let data = e.get(this, NiPathInterpolator::m_spPathData);
    if data == 0 {
        (0, 0, 0)
    } else {
        (
            e.mem.u32(data.wrapping_add(KEY_DATA_TYPE)),
            u32::from(e.mem.u8(data.wrapping_add(KEY_DATA_KEY_SIZE))),
            e.mem.u32(data.wrapping_add(KEY_DATA_KEYS)),
        )
    }
}

/// The path data's key count (0 without path data).
fn path_key_count(e: &Engine, this: Ptr<NiPathInterpolator>) -> u32 {
    let data = e.get(this, NiPathInterpolator::m_spPathData);
    if data == 0 {
        0
    } else {
        e.mem.u32(data.wrapping_add(KEY_DATA_NUM_KEYS))
    }
}

/// The low byte of the flags word (the exe tests its bits through a byte read).
fn flags_byte(e: &Engine, this: Ptr<NiPathInterpolator>) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0xc))
}

// Translated from 00a3dd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator`'s destructor body (unnamed in the map): the class's vtable, then
/// the path and percentage data are released and cleared, the partial lengths freed
/// (`00aa10f0`), the two pointers' own destructors run (reading the now cleared words, so
/// they release nothing more), and `NiKeyBasedInterpolator`'s destructor body (`00a3ca30`).
pub fn fn_00a3dd40(e: &mut Engine, this: Ptr<NiPathInterpolator>) {
    e.mem.set_u32(this.addr(), NI_PATH_INTERPOLATOR_VTABLE);
    for field in [
        NiPathInterpolator::m_spPathData,
        NiPathInterpolator::m_spPctData,
    ] {
        let object = e.get(this, field);
        if object != 0 {
            release(e, object);
            e.set(this, field, 0);
        }
    }
    let lengths = e.get(this, NiPathInterpolator::m_pfPartialLength);
    e.call(TABLE_FREE, &args![lengths]);
    for field in [
        NiPathInterpolator::m_spPctData,
        NiPathInterpolator::m_spPathData,
    ] {
        let object = e.get(this, field);
        if object != 0 {
            release(e, object);
        }
    }
    e.call(KEY_BASED_INTERPOLATOR_DESTRUCTOR, &args![this]);
}

// Translated from 00a3ddf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path's point between the keys `first` and `second` (unnamed in the map): the path
/// data's key-type interpolation (`011f3ca8`) with `ratio`, into `point`. Without path data
/// the key type, size and keys are 0. Answers `point`.
pub fn fn_00a3ddf0(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    point: Ptr,
    first: u32,
    second: u32,
    ratio: f32,
) -> Ptr {
    let (key_type, size, keys) = path_keys(e, this);
    let key = size.wrapping_mul(first).wrapping_add(keys);
    let next_key = size.wrapping_mul(second).wrapping_add(keys);
    let function = e
        .mem
        .u32(POS_KEY_INTERPOLATIONS.wrapping_add(key_type.wrapping_mul(4)));
    e.call(
        function,
        &args![x87_quiet_bits(ratio.to_bits()), key, next_key, point],
    );
    point
}

// Translated from 00a3dc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::ProcessClone` (Xbox PDB): `NiObject::ProcessClone`, then the path
/// and percentage data's own `ProcessClone` (vtable +0x68) when present.
pub fn ni_path_interpolator_process_clone(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    cloning: Ptr,
) {
    e.call(NI_OBJECT_PROCESS_CLONE, &args![this, cloning]);
    for field in [
        NiPathInterpolator::m_spPathData,
        NiPathInterpolator::m_spPctData,
    ] {
        let object = e.get(this, field);
        if object != 0 {
            e.vcall(object, 0x68, &args![cloning]);
        }
    }
}

// Translated from 00a3e5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path's speed between the keys `first` and `second` at `ratio` (unnamed in the map):
/// the key type's derivative (`011f3c18`) into a local vector, whose length (`x*x + y*y +
/// z*z` rounded to a float, through `_CIsqrt`) is answered as a float.
pub fn fn_00a3e5f0(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    first: u32,
    second: u32,
    ratio: f32,
) -> f32 {
    let (key_type, size, keys) = path_keys(e, this);
    let key = size.wrapping_mul(first).wrapping_add(keys);
    let next_key = size.wrapping_mul(second).wrapping_add(keys);
    let function = e
        .mem
        .u32(POS_KEY_DERIVATIVES.wrapping_add(key_type.wrapping_mul(4)));
    let [x, y, z] = e.with_stack(12, |e, vector| {
        e.call(
            function,
            &args![x87_quiet_bits(ratio.to_bits()), key, next_key, vector],
        );
        [0u32, 4, 8].map(|off| x87_load32(e.mem.u32(vector.addr() + off)))
    });
    let xx = x87_mul(x, x);
    let yy = x87_mul(y, y);
    let sum = x87_add(yy, xx);
    let zz = x87_mul(z, z);
    let squared = x87_store32(x87_add(sum, zz));
    let root = e.call(CRT_SQRT, &args![x87_load32(squared)]).f64();
    f32::from_bits(x87_quiet_bits(x87_store32(root)))
}

// Translated from 00a3e680 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path's length from the key `first` towards `second` up to `ratio` (unnamed in the
/// map): five-point Gauss-Legendre quadrature of the speed (`fn_00a3e5f0`) at the
/// abscissas (`011a854c`) scaled by `ratio`, weighted (`011a8538`) and summed as a float,
/// times `ratio`.
pub fn fn_00a3e680(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    first: u32,
    second: u32,
    ratio: f32,
) -> f32 {
    let ratio_bits = ratio.to_bits();
    let mut sum = x87_store32(0.0);
    for offset in (0..0x14u32).step_by(4) {
        let abscissa = e.global::<u32>(GAUSS_ABSCISSAS + offset);
        let at = x87_store32(x87_mul(x87_load32(abscissa), x87_m32(ratio_bits)));
        let speed = fn_00a3e5f0(e, this, first, second, f32::from_bits(x87_quiet_bits(at)));
        let weight = e.global::<u32>(GAUSS_WEIGHTS + offset);
        let weighted = x87_mul(x87_load32(speed.to_bits()), x87_m32(weight));
        sum = x87_store32(x87_add(weighted, x87_m32(sum)));
    }
    let length = x87_store32(x87_mul(x87_load32(sum), x87_m32(ratio_bits)));
    f32::from_bits(x87_quiet_bits(length))
}

// Translated from 00a3e9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path's total length (unnamed in the map): while `m_fTotalLength` is below 0 the
/// partial lengths are rebuilt (the old table freed, one float per key allocated, the first
/// 0, each next one the previous plus the length of that segment from `fn_00a3e680` with
/// ratio 1) and the last becomes the total. Answers the total. With no keys the loop runs
/// to the count's wrap-around and the total is read from the word before the table, as the
/// exe does.
pub fn fn_00a3e9a0(e: &mut Engine, this: Ptr<NiPathInterpolator>) -> f32 {
    let total = e.get(this, NiPathInterpolator::m_fTotalLength);
    if x87_load32(total) < 0.0 {
        let count = path_key_count(e, this);
        let old = e.get(this, NiPathInterpolator::m_pfPartialLength);
        e.call(TABLE_FREE, &args![old]);
        let bytes = count.wrapping_mul(4);
        let lengths = e.call(TABLE_ALLOCATE, &args![bytes]).u32();
        e.set(this, NiPathInterpolator::m_pfPartialLength, lengths);
        e.mem.set_u32(lengths, x87_store32(0.0));
        let last = count.wrapping_sub(1);
        if last != 0 {
            let mut index = 0u32;
            loop {
                let next = index.wrapping_add(1);
                let segment = fn_00a3e680(e, this, index, next, 1.0);
                let segment = x87_store32(x87_load32(segment.to_bits()));
                let lengths = e.get(this, NiPathInterpolator::m_pfPartialLength);
                let at = lengths.wrapping_add(index.wrapping_mul(4));
                let sum = x87_add(x87_load32(e.mem.u32(at)), x87_m32(segment));
                e.mem.set_u32(at.wrapping_add(4), x87_store32(sum));
                index = next;
                if index >= last {
                    break;
                }
            }
        }
        let lengths = e.get(this, NiPathInterpolator::m_pfPartialLength);
        let last_length = e.mem.u32(lengths.wrapping_add(bytes).wrapping_sub(4));
        e.set(
            this,
            NiPathInterpolator::m_fTotalLength,
            x87_quiet_bits(last_length),
        );
    }
    let total = e.get(this, NiPathInterpolator::m_fTotalLength);
    f32::from_bits(x87_quiet_bits(total))
}

// Translated from 00a3ea60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constant-speed segment search (unnamed in the map): the keys `first`/`second` and the
/// ratio between them at which the path has covered the fraction `time` of its total
/// length. Time 0 answers (0, 1, 0); a negative time is folded with `_CIfmod(time, 1) + 1`;
/// time 1 answers (count - 2, count - 1, 1); a time above 1 is folded with
/// `_CIfmod(time, 1)`. The partial lengths give the segment holding `time * total` and a
/// linear first guess (0 and the segment past the end when none holds it), refined by up
/// to 32 Newton steps (segment length `fn_00a3e680` against the distance into the segment,
/// over the speed `fn_00a3e5f0`) until the error is within 0.0001 (`01032980`).
pub fn fn_00a3ea60(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    time: f32,
    first_out: Ptr,
    second_out: Ptr,
    ratio_out: Ptr,
) {
    let count = path_key_count(e, this);
    let mut current = x87_load32(time.to_bits());
    if current == 0.0 {
        e.mem.set_u32(first_out.addr(), 0);
        e.mem.set_u32(second_out.addr(), 1);
        e.mem.set_u32(ratio_out.addr(), x87_store32(0.0));
        return;
    }
    if current < 0.0 {
        let folded = e.call(CI_FMOD, &args![current, 1.0f64]).f64();
        let folded = x87_load32(x87_store32(folded));
        current = x87_load32(x87_store32(x87_add(folded, 1.0)));
    }
    if current == 1.0 {
        e.mem.set_u32(first_out.addr(), count.wrapping_sub(2));
        e.mem.set_u32(second_out.addr(), count.wrapping_sub(1));
        e.mem.set_u32(ratio_out.addr(), x87_store32(1.0));
        return;
    }
    if 1.0 < current {
        let folded = e.call(CI_FMOD, &args![current, 1.0f64]).f64();
        current = x87_load32(x87_store32(folded));
    }
    let total = e.get(this, NiPathInterpolator::m_fTotalLength);
    let distance = x87_store32(x87_mul(current, x87_m32(total)));
    let mut into_segment = x87_store32(0.0);
    let mut ratio = into_segment;
    let mut first = 0u32;
    let mut second = 1u32;
    let last = count.wrapping_sub(1);
    if last != 0 {
        let lengths = e.get(this, NiPathInterpolator::m_pfPartialLength);
        loop {
            let next = e
                .mem
                .u32(lengths.wrapping_add(first.wrapping_mul(4)).wrapping_add(4));
            if x87_load32(next) >= x87_load32(distance) {
                let lengths = e.get(this, NiPathInterpolator::m_pfPartialLength);
                let start = e.mem.u32(lengths.wrapping_add(first.wrapping_mul(4)));
                let end = e.mem.u32(lengths.wrapping_add(second.wrapping_mul(4)));
                into_segment = x87_store32(x87_sub(x87_load32(distance), x87_m32(start)));
                let span = x87_sub(x87_load32(end), x87_m32(start));
                ratio = x87_store32(x87_div(x87_load32(into_segment), span));
                break;
            }
            first = first.wrapping_add(1);
            second = second.wrapping_add(1);
            if first >= last {
                break;
            }
        }
    }
    let tolerance = e.global::<f64>(ARC_LENGTH_TOLERANCE);
    let mut steps = 0u32;
    loop {
        let guess = f32::from_bits(x87_quiet_bits(ratio));
        let length = fn_00a3e680(e, this, first, second, guess);
        let error = x87_store32(x87_sub(x87_load32(length.to_bits()), x87_m32(into_segment)));
        let error = x87_load32(error);
        let magnitude = x87_store32(error.abs());
        if x87_load32(magnitude) <= tolerance {
            break;
        }
        let speed = fn_00a3e5f0(e, this, first, second, guess);
        let step = x87_div(error, x87_load32(speed.to_bits()));
        ratio = x87_store32(x87_sub(x87_load32(ratio), step));
        steps += 1;
        if steps >= 0x20 {
            break;
        }
    }
    e.mem.set_u32(first_out.addr(), first);
    e.mem.set_u32(second_out.addr(), second);
    e.mem.set_u32(ratio_out.addr(), x87_quiet_bits(ratio));
}

// Translated from 00a3ec30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path segment for `time` (unnamed in the map): with constant speed (flag bit 4) the
/// arc-length search `fn_00a3ea60`, its first key kept as `m_uiLastPathIdx`. Otherwise, by
/// the key times (each key's first float): at or before the first key (0, 1, 0); at or
/// after the last (count - 2, count - 1, 1); else from `m_uiLastPathIdx` on (wrapping to 0
/// once) the first key equal to `time` or followed by a later key, kept as
/// `m_uiLastPathIdx`, and the ratio `(time - t[first]) / (t[second] - t[first])` (with
/// `second` 0 when no key matched). Without path data the keys are read at address 0, as
/// the exe does.
pub fn fn_00a3ec30(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    time: f32,
    first_out: Ptr,
    second_out: Ptr,
    ratio_out: Ptr,
) {
    if (flags_byte(e, this) >> 4) & 1 != 0 {
        fn_00a3ea60(e, this, time, first_out, second_out, ratio_out);
        let first = e.mem.u32(first_out.addr());
        e.set(this, NiPathInterpolator::m_uiLastPathIdx, first);
        return;
    }
    let count = path_key_count(e, this);
    let (_, size, keys) = path_keys(e, this);
    let time = x87_load32(time.to_bits());
    let key_time = |e: &Engine, index: u32| e.mem.u32(size.wrapping_mul(index).wrapping_add(keys));
    if x87_load32(e.mem.u32(keys)) >= time {
        e.mem.set_u32(first_out.addr(), 0);
        e.mem.set_u32(second_out.addr(), 1);
        e.mem.set_u32(ratio_out.addr(), x87_store32(0.0));
        return;
    }
    if x87_load32(key_time(e, count.wrapping_sub(1))) <= time {
        e.mem.set_u32(first_out.addr(), count.wrapping_sub(2));
        e.mem.set_u32(second_out.addr(), count.wrapping_sub(1));
        e.mem.set_u32(ratio_out.addr(), x87_store32(1.0));
        return;
    }
    let mut first = e.get(this, NiPathInterpolator::m_uiLastPathIdx);
    let mut second = 0u32;
    let mut first_pass = true;
    let last = count.wrapping_sub(1);
    if first < last {
        loop {
            let at = x87_load32(key_time(e, first));
            let found = if at == time {
                true
            } else if at < time {
                x87_load32(key_time(e, first.wrapping_add(1))) > time
            } else {
                false
            };
            if found {
                second = first.wrapping_add(1);
                e.set(this, NiPathInterpolator::m_uiLastPathIdx, first);
                break;
            }
            if first == count.wrapping_sub(2) && first_pass {
                first = 0;
                first_pass = false;
            } else {
                first = first.wrapping_add(1);
            }
            if first >= last {
                break;
            }
        }
    }
    let start = key_time(e, first);
    let end = key_time(e, second);
    let elapsed = x87_sub(time, x87_m32(start));
    let span = x87_sub(x87_load32(end), x87_m32(start));
    e.mem
        .set_u32(ratio_out.addr(), x87_store32(x87_div(elapsed, span)));
    e.mem.set_u32(first_out.addr(), first);
    e.mem.set_u32(second_out.addr(), second);
}

// Translated from 00a3ed90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::LoadBinary` (Xbox PDB): `NiKeyBasedInterpolator::LoadBinary`, then
/// from the stream the flags (2 bytes), the bank direction (4 bytes, through a local), the
/// maximum bank angle and smoothing (4 bytes each) and the follow axis (2 bytes), and the
/// stream's link IDs of the two data objects (`00a645b0`, twice).
pub fn ni_path_interpolator_load_binary(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    stream: Ptr,
) {
    e.call(KEY_BASED_INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
    stream_read(e, stream, this.addr() + 0xc, 2);
    let bank = e.with_stack(4, |e, local| {
        stream_read(e, stream, local.addr(), 4);
        e.mem.u32(local.addr())
    });
    e.set(this, NiPathInterpolator::m_eBankDir, bank);
    stream_read(e, stream, this.addr() + 0x28, 4);
    stream_read(e, stream, this.addr() + 0x2c, 4);
    stream_read(e, stream, this.addr() + 0x30, 2);
    e.call(STREAM_READ_LINK_ID, &args![stream]);
    e.call(STREAM_READ_LINK_ID, &args![stream]);
}

// Translated from 00a3ee80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::LinkObject` (Xbox PDB): the base `LinkObject` (`00a95ca0`), the
/// path and percentage data from the stream's next linked objects (counted pointer
/// assignment), the "lengths stale" flag (bit 0) set; with constant speed (bit 4) the total
/// length is recomputed (`fn_00a3e9a0`) and bit 0 cleared. With path data holding keys, the
/// key type's function at `011f3d38` (`(keys, count)`) gives `m_fMaxCurvature`.
pub fn ni_path_interpolator_link_object(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    stream: Ptr,
) {
    e.call(KEY_BASED_INTERPOLATOR_LINK_OBJECT, &args![this, stream]);
    let path = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_counted(e, this.addr() + 0x18, path);
    let percentage = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_counted(e, this.addr() + 0x1c, percentage);
    let flags = e.get(this, NiPathInterpolator::m_uFlags) | 1;
    e.set(this, NiPathInterpolator::m_uFlags, flags);
    if (flags >> 4) & 1 != 0 && flags_byte(e, this) & 1 != 0 {
        let total = fn_00a3e9a0(e, this);
        e.set(this, NiPathInterpolator::m_fTotalLength, total.to_bits());
        let flags = e.get(this, NiPathInterpolator::m_uFlags) & 0xfffe;
        e.set(this, NiPathInterpolator::m_uFlags, flags);
    }
    let data = e.get(this, NiPathInterpolator::m_spPathData);
    if data != 0 {
        let count = e.mem.u32(data.wrapping_add(KEY_DATA_NUM_KEYS));
        let key_type = e.mem.u32(data.wrapping_add(KEY_DATA_TYPE));
        let keys = e.mem.u32(data.wrapping_add(KEY_DATA_KEYS));
        if count != 0 {
            let function = e
                .mem
                .u32(POS_KEY_CURVATURES.wrapping_add(key_type.wrapping_mul(4)));
            let curvature = e.call(function, &args![keys, count]).f64();
            e.set(
                this,
                NiPathInterpolator::m_fMaxCurvature,
                x87_store32(curvature),
            );
        }
    }
}

// Translated from 00a3f060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::NiPathInterpolator` (Xbox PDB), `(path data, percentage data)`:
/// `NiKeyBasedInterpolator`'s constructor, the class's vtable, both data pointers stored
/// and counted, the rotation the identity quaternion (`011f4208`), the position the invalid
/// point (`011a9424`), bank angle, smoothing and curvature 0, total length -1
/// (`01012054`), last time `-FLT_MAX` (`01015f5c`), indices 0, bank direction 1, follow
/// axis 0, no partial lengths, flags 3. Answers `this`.
pub fn ni_path_interpolator_constructor(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    path: Ptr,
    percentage: Ptr,
) -> Ptr<NiPathInterpolator> {
    e.call(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), NI_PATH_INTERPOLATOR_VTABLE);
    for (field, object) in [
        (NiPathInterpolator::m_spPathData, path.addr()),
        (NiPathInterpolator::m_spPctData, percentage.addr()),
    ] {
        e.set(this, field, object);
        if object != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
    }
    for word in 0..4u32 {
        let value = e.global::<u32>(QUATERNION_IDENTITY + 4 * word);
        e.mem.set_u32(this.addr() + 0x3c + 4 * word, value);
    }
    for word in 0..3u32 {
        let value = e.global::<u32>(INVALID_POINT3 + 4 * word);
        e.mem.set_u32(this.addr() + 0x4c + 4 * word, value);
    }
    let zero = x87_store32(0.0);
    e.set(this, NiPathInterpolator::m_fMaxBankAngle, zero);
    e.set(this, NiPathInterpolator::m_fSmoothing, zero);
    e.set(this, NiPathInterpolator::m_fMaxCurvature, zero);
    let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
    e.set(this, NiPathInterpolator::m_fTotalLength, minus_one);
    let lowest = x87_quiet_bits(e.global::<u32>(LOWEST_FLOAT));
    e.set(this, NiPathInterpolator::m_uiLastPathIdx, 0);
    e.set(this, NiPathInterpolator::m_fLastTime, lowest);
    e.set(this, NiPathInterpolator::m_uiLastPctIdx, 0);
    e.set(this, NiPathInterpolator::m_eBankDir, 1);
    e.set(this, NiPathInterpolator::m_sFollowAxis, 0);
    e.set(this, NiPathInterpolator::m_pfPartialLength, 0);
    e.set(this, NiPathInterpolator::m_uFlags, 3);
    this
}

/// `Update`'s copy of the kept position (three words) into the transform, and with flag
/// bit 5 the kept rotation (four words) after it.
fn copy_transform(e: &mut Engine, this: Ptr<NiPathInterpolator>, transform: Ptr) {
    for word in 0..3u32 {
        let value = e.mem.u32(this.addr() + 0x4c + 4 * word);
        e.mem.set_u32(transform.addr() + 4 * word, value);
    }
    if (flags_byte(e, this) >> 5) & 1 != 0 {
        for word in 0..4u32 {
            let value = e.mem.u32(this.addr() + 0x3c + 4 * word);
            e.mem.set_u32(transform.addr() + 0xc + 4 * word, value);
        }
    }
}

// Translated from 00a3f120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::Update` (Xbox PDB), the transform update `(time, interpolator
/// target, &transform)`: at the base's last time (+0x08) the kept position (and with flag
/// bit 5 the kept rotation) is copied out, unless the position is the invalid point, which
/// answers false with the transform's translation, rotation and scale marked invalid
/// (`-FLT_MAX` at +0x00, +0x10, +0x1C). Otherwise, with percentage data holding keys and
/// path data holding keys: the percentage at `time` (`NiFloatKey::GenInterp`), its path
/// segment (`fn_00a3ec30`), with bit 5 the frame's rotation (`CalcRefFrame`,
/// `NiQuaternion::FromRotation` into `m_kRotation`), the point (`fn_00a3ddf0`) kept and
/// copied out with the rotation as before, `m_fLastTime` set; answers true. Without the
/// data it answers false.
pub fn ni_path_interpolator_update(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    time: f32,
    _unused_2: u32,
    transform: Ptr,
) -> bool {
    let at = x87_load32(time.to_bits());
    let last = x87_load32(e.get(this, NiPathInterpolator::m_fInterpolatorLastTime));
    if at == last {
        let invalid = (0..3u32).all(|word| {
            x87_load32(e.mem.u32(INVALID_POINT3 + 4 * word))
                == x87_load32(e.mem.u32(this.addr() + 0x4c + 4 * word))
        });
        if invalid {
            let lowest = x87_quiet_bits(e.global::<u32>(LOWEST_FLOAT));
            for offset in [0u32, 0x10, 0x1c] {
                e.mem.set_u32(transform.addr() + offset, lowest);
            }
            return false;
        }
        copy_transform(e, this, transform);
        return true;
    }
    let percentage = e.get(this, NiPathInterpolator::m_spPctData);
    if percentage == 0 {
        return false;
    }
    let size = u32::from(e.mem.u8(percentage.wrapping_add(KEY_DATA_KEY_SIZE)));
    let count = e.mem.u32(percentage.wrapping_add(KEY_DATA_NUM_KEYS));
    let key_type = e.mem.u32(percentage.wrapping_add(KEY_DATA_TYPE));
    let keys = e.mem.u32(percentage.wrapping_add(KEY_DATA_KEYS));
    if keys == 0 {
        return false;
    }
    let path = e.get(this, NiPathInterpolator::m_spPathData);
    if path == 0 || e.mem.u32(path.wrapping_add(KEY_DATA_KEYS)) == 0 {
        return false;
    }
    e.with_stack(0x40, |e, locals| {
        let ratio_slot = locals;
        let second_slot = locals.byte_add(4);
        let first_slot = locals.byte_add(8);
        let point = locals.byte_add(0xc);
        let matrix = locals.byte_add(0x18);
        let percent = e
            .call(
                FLOAT_KEY_GEN_INTERP,
                &args![
                    x87_quiet_bits(time.to_bits()),
                    keys,
                    key_type,
                    count,
                    this.addr() + 0x14,
                    size
                ],
            )
            .f64();
        let percent = x87_quiet_bits(x87_store32(percent));
        fn_00a3ec30(
            e,
            this,
            f32::from_bits(percent),
            first_slot,
            second_slot,
            ratio_slot,
        );
        let second = e.mem.u32(second_slot.addr());
        let first = e.mem.u32(first_slot.addr());
        if (flags_byte(e, this) >> 5) & 1 != 0 {
            let ratio = x87_quiet_bits(e.mem.u32(ratio_slot.addr()));
            e.call(CALC_REF_FRAME, &args![this, first, second, ratio, matrix]);
            e.call(QUATERNION_FROM_ROTATION, &args![this.addr() + 0x3c, matrix]);
        }
        let ratio = x87_quiet_bits(e.mem.u32(ratio_slot.addr()));
        let result = fn_00a3ddf0(e, this, point, first, second, f32::from_bits(ratio));
        for word in 0..3u32 {
            let value = e.mem.u32(result.addr() + 4 * word);
            e.mem.set_u32(this.addr() + 0x4c + 4 * word, value);
        }
    });
    copy_transform(e, this, transform);
    e.set(
        this,
        NiPathInterpolator::m_fLastTime,
        x87_quiet_bits(time.to_bits()),
    );
    true
}

// Translated from 00a3f2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::CopyMembers` (unnamed in the map), `(destination, cloning)`:
/// `NiKeyBasedInterpolator::CopyMembers`, the flags word, clones (vtable +0x48) of the path
/// and percentage data assigned as counted pointers, flag bits 2, 3, 4, 5, 6 and 1 copied
/// one by one (with bit 4 set and the destination's bit 0 set the total length is
/// recomputed and bit 0 cleared), the bank direction, bank angle, smoothing and follow axis
/// copied, the partial lengths duplicated (`00aa1070` and `memcpy`, one float per path key)
/// when present, and the total length copied.
pub fn fn_00a3f2f0(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    destination: Ptr<NiPathInterpolator>,
    cloning: Ptr,
) {
    e.call(
        KEY_BASED_INTERPOLATOR_COPY_MEMBERS,
        &args![this, destination, cloning],
    );
    let flags = e.get(this, NiPathInterpolator::m_uFlags);
    e.set(destination, NiPathInterpolator::m_uFlags, flags);
    for (offset, field) in [
        (0x18u32, NiPathInterpolator::m_spPathData),
        (0x1c, NiPathInterpolator::m_spPctData),
    ] {
        let object = e.get(this, field);
        if object != 0 {
            let clone = e.vcall(object, 0x48, &args![cloning]).u32();
            assign_counted(e, destination.addr() + offset, clone);
        }
    }
    let copy_bit = |e: &mut Engine, bit: u32| {
        let source = flags_byte(e, this);
        let flags = e.get(destination, NiPathInterpolator::m_uFlags);
        let flags = if (source >> bit) & 1 != 0 {
            flags | (1u16 << bit)
        } else {
            flags & !(1u16 << bit)
        };
        e.set(destination, NiPathInterpolator::m_uFlags, flags);
    };
    copy_bit(e, 2);
    copy_bit(e, 3);
    let bank = e.get(this, NiPathInterpolator::m_eBankDir);
    e.set(destination, NiPathInterpolator::m_eBankDir, bank);
    if (flags_byte(e, this) >> 4) & 1 != 0 {
        let flags = e.get(destination, NiPathInterpolator::m_uFlags) | 0x10;
        e.set(destination, NiPathInterpolator::m_uFlags, flags);
        if flags_byte(e, destination) & 1 != 0 {
            let total = fn_00a3e9a0(e, destination);
            e.set(
                destination,
                NiPathInterpolator::m_fTotalLength,
                total.to_bits(),
            );
            let flags = e.get(destination, NiPathInterpolator::m_uFlags) & 0xfffe;
            e.set(destination, NiPathInterpolator::m_uFlags, flags);
        }
    } else {
        let flags = e.get(destination, NiPathInterpolator::m_uFlags) & 0xffef;
        e.set(destination, NiPathInterpolator::m_uFlags, flags);
    }
    copy_bit(e, 5);
    for field in [
        NiPathInterpolator::m_fMaxBankAngle,
        NiPathInterpolator::m_fSmoothing,
    ] {
        let value = x87_quiet_bits(e.get(this, field));
        e.set(destination, field, value);
    }
    let axis = e.get(this, NiPathInterpolator::m_sFollowAxis);
    e.set(destination, NiPathInterpolator::m_sFollowAxis, axis);
    copy_bit(e, 6);
    copy_bit(e, 1);
    if e.get(this, NiPathInterpolator::m_pfPartialLength) != 0 {
        let bytes = path_key_count(e, this).wrapping_mul(4);
        let copy = e.call(TABLE_ALLOCATE, &args![bytes]).u32();
        e.set(destination, NiPathInterpolator::m_pfPartialLength, copy);
        let lengths = e.get(this, NiPathInterpolator::m_pfPartialLength);
        e.call(MEMCPY, &args![copy, lengths, bytes]);
    }
    let total = x87_quiet_bits(e.get(this, NiPathInterpolator::m_fTotalLength));
    e.set(destination, NiPathInterpolator::m_fTotalLength, total);
}

// Translated from 00a3f520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::CreateClone` (Xbox PDB), `(cloning)`: a 0x5C-byte block
/// (`operator new`), constructed without data (`NiPathInterpolator(0, 0)`), and this
/// interpolator's members copied into it (`fn_00a3f2f0`, also run with a null destination
/// when the allocation fails). Answers the clone (null when the allocation failed).
pub fn ni_path_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    cloning: Ptr,
) -> Ptr<NiPathInterpolator> {
    let block = e.call(OPERATOR_NEW, &args![0x5cu32]).u32();
    let clone = if block != 0 {
        ni_path_interpolator_constructor(e, Ptr::new(block), Ptr::NULL, Ptr::NULL)
    } else {
        Ptr::NULL
    };
    fn_00a3f2f0(e, this, clone, cloning);
    clone
}

// Translated from 00a3f4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPathInterpolator` factory (Ghidra made no function here; unnamed in the map):
/// a 0x5C-byte block (`operator new`) constructed without
/// data, or null when the allocation fails.
pub fn fn_00a3f4b0(e: &mut Engine) -> Ptr<NiPathInterpolator> {
    let block = e.call(OPERATOR_NEW, &args![0x5cu32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    ni_path_interpolator_constructor(e, Ptr::new(block), Ptr::NULL, Ptr::NULL)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3dab0, ni_pos_data_get_rtti() -> Ptr),
        entry!(0x00a3dde0, ni_path_interpolator_get_rtti() -> Ptr),
        entry!(0x00a3dac0, ni_pos_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a3e980, ni_path_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a3dd40, fn_00a3dd40(Ptr<NiPathInterpolator>)),
        entry!(
            0x00a3ddf0,
            fn_00a3ddf0(Ptr<NiPathInterpolator>, Ptr, u32, u32, f32) -> Ptr
        ),
        entry!(
            0x00a3dc80,
            ni_path_interpolator_process_clone(Ptr<NiPathInterpolator>, Ptr)
        ),
        entry!(
            0x00a3e5f0,
            fn_00a3e5f0(Ptr<NiPathInterpolator>, u32, u32, f32) -> f32
        ),
        entry!(
            0x00a3e680,
            fn_00a3e680(Ptr<NiPathInterpolator>, u32, u32, f32) -> f32
        ),
        entry!(0x00a3e9a0, fn_00a3e9a0(Ptr<NiPathInterpolator>) -> f32),
        entry!(
            0x00a3ea60,
            fn_00a3ea60(Ptr<NiPathInterpolator>, f32, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00a3ec30,
            fn_00a3ec30(Ptr<NiPathInterpolator>, f32, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00a3ed90,
            ni_path_interpolator_load_binary(Ptr<NiPathInterpolator>, Ptr)
        ),
        entry!(
            0x00a3ee80,
            ni_path_interpolator_link_object(Ptr<NiPathInterpolator>, Ptr)
        ),
        entry!(
            0x00a3f060,
            ni_path_interpolator_constructor(
                Ptr<NiPathInterpolator>,
                Ptr,
                Ptr,
            ) -> Ptr<NiPathInterpolator>
        ),
        entry!(
            0x00a3f120,
            ni_path_interpolator_update(Ptr<NiPathInterpolator>, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a3f2f0,
            fn_00a3f2f0(Ptr<NiPathInterpolator>, Ptr<NiPathInterpolator>, Ptr)
        ),
        entry!(
            0x00a3f520,
            ni_path_interpolator_create_clone(
                Ptr<NiPathInterpolator>,
                Ptr,
            ) -> Ptr<NiPathInterpolator>
        ),
        entry!(0x00a3f4b0, fn_00a3f4b0() -> Ptr<NiPathInterpolator>),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_pos_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a3dab0, &args![]).u32(), NI_POS_DATA_RTTI);
    }

    #[test]
    fn ni_path_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a3dde0, &args![]).u32(),
            NI_PATH_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_pos_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a4_8fb0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a3dac0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a3dac0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x18]]);
        assert_eq!(calls_to(&e, 0x00a4_8fb0), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_path_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a3_dd40, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a3e980, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a3e980, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x5c]]);
        assert_eq!(calls_to(&e, 0x00a3_dd40), vec![vec![this]; 2]);
    }

    /// The argument words of every logged call to `addr`.
    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine logging calls, with the interlocked imports working on memory, the
    /// constants the constructor and the searches read, and doubles for the allocator.
    fn path_engine() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        e.register(TABLE_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0].max(4)),
            ..Ret::default()
        });
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(CRT_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        e.register(CI_FMOD, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            let y = f64::from_bits(u64::from(a[2]) | u64::from(a[3]) << 32);
            Ret {
                st0: x % y,
                ..Ret::default()
            }
        });
        for page in [
            0x011a_8000u32,
            0x011a_9000,
            0x011f_3000,
            0x011f_4000,
            0x0101_2000,
            0x0101_5000,
            0x0103_2000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global::<f64>(ARC_LENGTH_TOLERANCE, f64::from_bits(0x3f1a_36e2_e000_0000));
        e.set_global::<f32>(MINUS_ONE, -1.0);
        e.set_global::<u32>(LOWEST_FLOAT, 0xff7f_ffff);
        for word in 0..3u32 {
            e.set_global::<u32>(INVALID_POINT3 + 4 * word, 0x7f7f_ffff);
        }
        e.set_global::<f32>(QUATERNION_IDENTITY, 1.0);
        // Gauss abscissas 0.25, 0.5, 0.5, 0.75, 1.0 and weights summing exactly to 1.
        for (i, (x, w)) in [
            (0.25f32, 0.125f32),
            (0.5, 0.25),
            (0.5, 0.25),
            (0.75, 0.25),
            (1.0, 0.125),
        ]
        .into_iter()
        .enumerate()
        {
            e.set_global::<f32>(GAUSS_ABSCISSAS + 4 * i as u32, x);
            e.set_global::<f32>(GAUSS_WEIGHTS + 4 * i as u32, w);
        }
        e
    }

    /// A key-data object (`NiPosData`/`NiFloatData` shape) with `times.len()` keys of
    /// `size` bytes, each key's first float its time.
    fn key_data(e: &mut Engine, key_type: u32, size: u8, times: &[f32]) -> u32 {
        let data = e.mem.alloc(0x18);
        let keys = e.mem.alloc((times.len() as u32 * u32::from(size)).max(4));
        for (i, t) in times.iter().enumerate() {
            e.mem.set_f32(keys + i as u32 * u32::from(size), *t);
        }
        e.mem.set_u32(data + KEY_DATA_NUM_KEYS, times.len() as u32);
        e.mem.set_u32(data + KEY_DATA_KEYS, keys);
        e.mem.set_u32(data + KEY_DATA_TYPE, key_type);
        e.mem.set_u8(data + KEY_DATA_KEY_SIZE, size);
        data
    }

    /// Key type 0's derivative writes `(speed, 0, 0)`: the path's speed is `speed` everywhere.
    fn constant_speed(e: &mut Engine, speed: f32) {
        e.mem.set_u32(POS_KEY_DERIVATIVES, 0x0090_0200);
        e.register_double(0x0090_0200, move |e, a| {
            e.mem.set_f32(a[3], speed);
            e.mem.set_f32(a[3] + 4, 0.0);
            e.mem.set_f32(a[3] + 8, 0.0);
            Ret::default()
        });
    }

    fn interpolator(e: &mut Engine) -> Ptr<NiPathInterpolator> {
        e.new_object::<NiPathInterpolator>()
    }

    #[test]
    fn destructor_body_releases_the_data_and_frees_the_lengths() {
        let mut e = path_engine();
        e.register(KEY_BASED_INTERPOLATOR_DESTRUCTOR, |_, _| Ret::default());
        e.register(0x0090_0004, |_, _| Ret::default());
        e.put_vtable(0x0090_1000, &[0, 0x0090_0004]);
        let this = interpolator(&mut e);
        let path = e.mem.alloc(0x18);
        let percentage = e.mem.alloc(0x18);
        for (object, count) in [(path, 1u32), (percentage, 2)] {
            e.mem.set_u32(object, 0x0090_1000);
            e.mem.set_u32(object + 4, count);
        }
        e.set(this, NiPathInterpolator::m_spPathData, path);
        e.set(this, NiPathInterpolator::m_spPctData, percentage);
        e.set(this, NiPathInterpolator::m_pfPartialLength, 0x1234);
        e.call(0x00a3dd40, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NI_PATH_INTERPOLATOR_VTABLE);
        assert_eq!(e.get(this, NiPathInterpolator::m_spPathData), 0);
        assert_eq!(e.get(this, NiPathInterpolator::m_spPctData), 0);
        assert_eq!(calls_at(&e, 0x0090_0004), vec![vec![path]]);
        assert_eq!(e.mem.u32(percentage + 4), 1);
        assert_eq!(calls_at(&e, TABLE_FREE), vec![vec![0x1234]]);
        assert_eq!(
            calls_at(&e, KEY_BASED_INTERPOLATOR_DESTRUCTOR),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn point_interpolates_with_the_key_types_function() {
        let mut e = path_engine();
        e.map(POS_KEY_INTERPOLATIONS, 0x10);
        e.mem.set_u32(POS_KEY_INTERPOLATIONS + 8, 0x0090_0100);
        e.mem.set_u32(POS_KEY_INTERPOLATIONS, 0x0090_0108);
        e.register(0x0090_0100, |_, _| Ret::default());
        e.register(0x0090_0108, |_, _| Ret::default());
        let this = interpolator(&mut e);
        let data = key_data(&mut e, 2, 0x10, &[0.0, 1.0, 2.0, 3.0]);
        let keys = e.mem.u32(data + KEY_DATA_KEYS);
        e.set(this, NiPathInterpolator::m_spPathData, data);
        let point = 0x7000u32;
        assert_eq!(
            e.call(0x00a3ddf0, &args![this, point, 1u32, 3u32, 0.25f32])
                .u32(),
            point
        );
        assert_eq!(
            calls_at(&e, 0x0090_0100),
            vec![vec![0.25f32.to_bits(), keys + 0x10, keys + 0x30, point]]
        );
        // Without path data: key type, size and keys 0.
        e.set(this, NiPathInterpolator::m_spPathData, 0);
        e.call(0x00a3ddf0, &args![this, point, 1u32, 3u32, 0.5f32]);
        assert_eq!(
            calls_at(&e, 0x0090_0108),
            vec![vec![0.5f32.to_bits(), 0, 0, point]]
        );
    }

    #[test]
    fn process_clone_reaches_both_data_objects() {
        let mut e = path_engine();
        e.register(NI_OBJECT_PROCESS_CLONE, |_, _| Ret::default());
        e.register(0x0090_0068, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x1b];
        slots[0x1a] = 0x0090_0068;
        e.put_vtable(0x0090_1000, &slots);
        let this = interpolator(&mut e);
        let path = e.mem.alloc(0x18);
        e.mem.set_u32(path, 0x0090_1000);
        e.set(this, NiPathInterpolator::m_spPathData, path);
        e.call(0x00a3dc80, &args![this, 0x5555u32]);
        assert_eq!(
            calls_at(&e, NI_OBJECT_PROCESS_CLONE),
            vec![vec![this.addr(), 0x5555]]
        );
        assert_eq!(calls_at(&e, 0x0090_0068), vec![vec![path, 0x5555]]);
        let percentage = e.mem.alloc(0x18);
        e.mem.set_u32(percentage, 0x0090_1000);
        e.set(this, NiPathInterpolator::m_spPctData, percentage);
        e.call(0x00a3dc80, &args![this, 0x5555u32]);
        assert_eq!(calls_at(&e, 0x0090_0068).len(), 3);
    }

    #[test]
    fn speed_is_the_length_of_the_derivative() {
        let mut e = path_engine();
        e.mem.set_u32(POS_KEY_DERIVATIVES, 0x0090_0200);
        e.register(0x0090_0200, |e, a| {
            e.mem.set_f32(a[3], 3.0);
            e.mem.set_f32(a[3] + 4, 4.0);
            e.mem.set_f32(a[3] + 8, 12.0);
            Ret::default()
        });
        let this = interpolator(&mut e);
        let data = key_data(&mut e, 0, 8, &[0.0, 1.0]);
        let keys = e.mem.u32(data + KEY_DATA_KEYS);
        e.set(this, NiPathInterpolator::m_spPathData, data);
        let speed = e.call(0x00a3e5f0, &args![this, 0u32, 1u32, 0.5f32]).f64();
        assert_eq!(speed, 13.0);
        let call = &calls_at(&e, 0x0090_0200)[0];
        assert_eq!(call[..3], [0.5f32.to_bits(), keys, keys + 8]);
        assert_eq!(calls_at(&e, CRT_SQRT), vec![args![169.0f64]]);
    }

    #[test]
    fn segment_length_integrates_the_speed() {
        let mut e = path_engine();
        constant_speed(&mut e, 3.0);
        let this = interpolator(&mut e);
        let data = key_data(&mut e, 0, 8, &[0.0, 1.0]);
        e.set(this, NiPathInterpolator::m_spPathData, data);
        let length = e.call(0x00a3e680, &args![this, 0u32, 1u32, 2.0f32]).f64();
        assert_eq!(length, 6.0);
        let ratios: Vec<u32> = calls_at(&e, 0x0090_0200).iter().map(|c| c[0]).collect();
        let expected: Vec<u32> = [0.5f32, 1.0, 1.0, 1.5, 2.0]
            .iter()
            .map(|r| r.to_bits())
            .collect();
        assert_eq!(ratios, expected);
    }

    #[test]
    fn total_length_is_rebuilt_only_while_negative() {
        let mut e = path_engine();
        constant_speed(&mut e, 1.0);
        let this = interpolator(&mut e);
        let data = key_data(&mut e, 0, 8, &[0.0, 1.0, 2.0]);
        e.set(this, NiPathInterpolator::m_spPathData, data);
        e.set(this, NiPathInterpolator::m_fTotalLength, 5.0f32.to_bits());
        assert_eq!(e.call(0x00a3e9a0, &args![this]).f64(), 5.0);
        assert!(calls_at(&e, TABLE_ALLOCATE).is_empty());
        e.set(
            this,
            NiPathInterpolator::m_fTotalLength,
            (-1.0f32).to_bits(),
        );
        e.set(this, NiPathInterpolator::m_pfPartialLength, 0x4444);
        assert_eq!(e.call(0x00a3e9a0, &args![this]).f64(), 2.0);
        assert_eq!(calls_at(&e, TABLE_FREE), vec![vec![0x4444]]);
        assert_eq!(calls_at(&e, TABLE_ALLOCATE), vec![vec![12]]);
        let lengths = e.get(this, NiPathInterpolator::m_pfPartialLength);
        assert_eq!(
            [0u32, 4, 8].map(|o| e.mem.f32(lengths + o)),
            [0.0, 1.0, 2.0]
        );
        assert_eq!(
            e.get(this, NiPathInterpolator::m_fTotalLength),
            2.0f32.to_bits()
        );
    }

    /// Runs a segment search at `address` and answers (first, second, ratio).
    fn search(
        e: &mut Engine,
        address: u32,
        this: Ptr<NiPathInterpolator>,
        time: f32,
    ) -> (u32, u32, f32) {
        let out = e.mem.alloc(12);
        e.call(address, &args![this, time, out, out + 4, out + 8]);
        (e.mem.u32(out), e.mem.u32(out + 4), e.mem.f32(out + 8))
    }

    /// A constant-speed interpolator over three keys with partial lengths 0, 1, 2.
    fn arc_length_path(e: &mut Engine) -> Ptr<NiPathInterpolator> {
        let this = interpolator(e);
        let data = key_data(e, 0, 8, &[0.0, 1.0, 2.0]);
        e.set(this, NiPathInterpolator::m_spPathData, data);
        let lengths = e.mem.alloc(12);
        for (i, l) in [0.0f32, 1.0, 2.0].into_iter().enumerate() {
            e.mem.set_f32(lengths + 4 * i as u32, l);
        }
        e.set(this, NiPathInterpolator::m_pfPartialLength, lengths);
        e.set(this, NiPathInterpolator::m_fTotalLength, 2.0f32.to_bits());
        this
    }

    #[test]
    fn arc_length_search_folds_the_time_and_refines_the_ratio() {
        let mut e = path_engine();
        constant_speed(&mut e, 1.0);
        let this = arc_length_path(&mut e);
        assert_eq!(search(&mut e, 0x00a3ea60, this, 0.0), (0, 1, 0.0));
        assert_eq!(search(&mut e, 0x00a3ea60, this, 1.0), (1, 2, 1.0));
        assert_eq!(search(&mut e, 0x00a3ea60, this, 0.25), (0, 1, 0.5));
        assert_eq!(search(&mut e, 0x00a3ea60, this, -0.75), (0, 1, 0.5));
        assert_eq!(search(&mut e, 0x00a3ea60, this, 1.25), (0, 1, 0.5));
        assert_eq!(search(&mut e, 0x00a3ea60, this, 0.75), (1, 2, 0.5));
        assert_eq!(calls_at(&e, CI_FMOD).len(), 2);
    }

    #[test]
    fn arc_length_search_takes_newton_steps_until_close() {
        let mut e = path_engine();
        // Speed 2 while the partial lengths say 1 per segment: the first guess 0.5 gives
        // length 1 against 0.5 into the segment; one step (0.5 / 2) gives 0.25.
        constant_speed(&mut e, 2.0);
        let this = arc_length_path(&mut e);
        assert_eq!(search(&mut e, 0x00a3ea60, this, 0.25), (0, 1, 0.25));
    }

    #[test]
    fn key_search_by_times() {
        let mut e = path_engine();
        let this = interpolator(&mut e);
        let data = key_data(&mut e, 0, 0x10, &[0.0, 1.0, 2.0, 3.0]);
        e.set(this, NiPathInterpolator::m_spPathData, data);
        assert_eq!(search(&mut e, 0x00a3ec30, this, -1.0), (0, 1, 0.0));
        assert_eq!(search(&mut e, 0x00a3ec30, this, 3.0), (2, 3, 1.0));
        assert_eq!(search(&mut e, 0x00a3ec30, this, 1.5), (1, 2, 0.5));
        assert_eq!(e.get(this, NiPathInterpolator::m_uiLastPathIdx), 1);
        // From the last segment the search wraps to the start once.
        e.set(this, NiPathInterpolator::m_uiLastPathIdx, 2);
        assert_eq!(search(&mut e, 0x00a3ec30, this, 0.5), (0, 1, 0.5));
        assert_eq!(e.get(this, NiPathInterpolator::m_uiLastPathIdx), 0);
        // A key equal to the time.
        assert_eq!(search(&mut e, 0x00a3ec30, this, 2.0), (2, 3, 0.0));
        // Constant speed: the arc-length search, its first key kept.
        e.set(this, NiPathInterpolator::m_uFlags, 0x10);
        e.set(this, NiPathInterpolator::m_uiLastPathIdx, 7);
        assert_eq!(search(&mut e, 0x00a3ec30, this, 0.0), (0, 1, 0.0));
        assert_eq!(e.get(this, NiPathInterpolator::m_uiLastPathIdx), 0);
    }

    /// A stream whose input's read function fills each read with its 1-based call number.
    fn stream(e: &mut Engine) -> Ptr {
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, 0x0090_0300);
        let mut n = 0u8;
        e.register_double(0x0090_0300, move |e, a| {
            n += 1;
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, n);
            }
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        Ptr::new(stream)
    }

    #[test]
    fn load_binary_reads_the_members() {
        let mut e = path_engine();
        e.register(KEY_BASED_INTERPOLATOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_READ_LINK_ID, |_, _| Ret::default());
        let this = interpolator(&mut e);
        let stream = stream(&mut e);
        e.call(0x00a3ed90, &args![this, stream]);
        assert_eq!(e.get(this, NiPathInterpolator::m_uFlags), 0x0101);
        assert_eq!(e.get(this, NiPathInterpolator::m_eBankDir), 0x0202_0202);
        assert_eq!(
            e.get(this, NiPathInterpolator::m_fMaxBankAngle),
            0x0303_0303
        );
        assert_eq!(e.get(this, NiPathInterpolator::m_fSmoothing), 0x0404_0404);
        assert_eq!(e.get(this, NiPathInterpolator::m_sFollowAxis), 0x0505);
        let input = e.mem.u32(stream.addr() + 0x24c);
        assert_eq!(e.mem.u32(input + 4), 16);
        let sizes: Vec<u32> = calls_at(&e, 0x0090_0300).iter().map(|c| c[2]).collect();
        assert_eq!(sizes, vec![2, 4, 4, 4, 2]);
        assert_eq!(
            calls_at(&e, STREAM_READ_LINK_ID),
            vec![vec![stream.addr()]; 2]
        );
        assert_eq!(
            calls_at(&e, KEY_BASED_INTERPOLATOR_LOAD_BINARY),
            vec![vec![this.addr(), stream.addr()]]
        );
    }

    #[test]
    fn link_object_links_the_data_and_its_curvature() {
        let mut e = path_engine();
        e.register(KEY_BASED_INTERPOLATOR_LINK_OBJECT, |_, _| Ret::default());
        e.register(0x0090_0004, |_, _| Ret::default());
        e.put_vtable(0x0090_1000, &[0, 0x0090_0004]);
        let path = key_data(&mut e, 1, 8, &[0.0, 1.0]);
        let percentage = e.mem.alloc(0x18);
        let mut linked = vec![percentage, path];
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| Ret {
            eax: linked.pop().unwrap_or(0),
            ..Ret::default()
        });
        e.map(POS_KEY_CURVATURES, 8);
        e.mem.set_u32(POS_KEY_CURVATURES + 4, 0x0090_0400);
        e.register(0x0090_0400, |_, _| Ret {
            st0: 0.75,
            ..Ret::default()
        });
        let this = interpolator(&mut e);
        let old = e.mem.alloc(0x18);
        e.mem.set_u32(old, 0x0090_1000);
        e.mem.set_u32(old + 4, 1);
        e.set(this, NiPathInterpolator::m_spPathData, old);
        e.set(this, NiPathInterpolator::m_uFlags, 0x10);
        e.set(this, NiPathInterpolator::m_fTotalLength, 5.0f32.to_bits());
        e.call(0x00a3ee80, &args![this, 0x6000u32]);
        assert_eq!(calls_at(&e, 0x0090_0004), vec![vec![old]]);
        assert_eq!(e.get(this, NiPathInterpolator::m_spPathData), path);
        assert_eq!(e.get(this, NiPathInterpolator::m_spPctData), percentage);
        assert_eq!(e.mem.u32(path + 4), 1);
        assert_eq!(e.mem.u32(percentage + 4), 1);
        assert_eq!(e.get(this, NiPathInterpolator::m_uFlags), 0x10);
        let keys = e.mem.u32(path + KEY_DATA_KEYS);
        assert_eq!(calls_at(&e, 0x0090_0400), vec![vec![keys, 2]]);
        assert_eq!(
            e.get(this, NiPathInterpolator::m_fMaxCurvature),
            0.75f32.to_bits()
        );
        // Without constant speed the stale flag stays set.
        let other = interpolator(&mut e);
        e.call(0x00a3ee80, &args![other, 0x6000u32]);
        assert_eq!(e.get(other, NiPathInterpolator::m_uFlags), 1);
        assert_eq!(calls_at(&e, 0x0090_0400).len(), 1);
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut e = path_engine();
        e.register(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, |_, _| Ret::default());
        let this = interpolator(&mut e);
        let path = e.mem.alloc(0x18);
        assert_eq!(
            e.call(0x00a3f060, &args![this, path, 0u32]).u32(),
            this.addr()
        );
        assert_eq!(e.mem.u32(this.addr()), NI_PATH_INTERPOLATOR_VTABLE);
        assert_eq!(e.get(this, NiPathInterpolator::m_spPathData), path);
        assert_eq!(e.mem.u32(path + 4), 1);
        assert_eq!(e.get(this, NiPathInterpolator::m_spPctData), 0);
        assert_eq!(e.mem.f32(this.addr() + 0x3c), 1.0);
        assert_eq!(e.mem.u32(this.addr() + 0x50), 0x7f7f_ffff);
        assert_eq!(
            e.get(this, NiPathInterpolator::m_fTotalLength),
            (-1.0f32).to_bits()
        );
        assert_eq!(e.get(this, NiPathInterpolator::m_fLastTime), 0xff7f_ffff);
        assert_eq!(e.get(this, NiPathInterpolator::m_eBankDir), 1);
        assert_eq!(e.get(this, NiPathInterpolator::m_uFlags), 3);
        assert_eq!(calls_at(&e, INTERLOCKED_INCREMENT).len(), 1);
    }

    #[test]
    fn update_uses_the_kept_point_at_the_last_time() {
        let mut e = path_engine();
        let this = interpolator(&mut e);
        let transform = e.mem.alloc(0x20);
        e.set(
            this,
            NiPathInterpolator::m_fInterpolatorLastTime,
            2.0f32.to_bits(),
        );
        for word in 0..3u32 {
            e.mem.set_u32(this.addr() + 0x4c + 4 * word, 0x7f7f_ffff);
        }
        assert!(!e
            .call(0x00a3f120, &args![this, 2.0f32, 0u32, transform])
            .bool());
        for offset in [0u32, 0x10, 0x1c] {
            assert_eq!(e.mem.u32(transform + offset), 0xff7f_ffff);
        }
        e.mem.set_f32(this.addr() + 0x50, 4.0);
        e.mem.set_f32(this.addr() + 0x3c, 9.0);
        e.set(this, NiPathInterpolator::m_uFlags, 0x20);
        assert!(e
            .call(0x00a3f120, &args![this, 2.0f32, 0u32, transform])
            .bool());
        assert_eq!(e.mem.f32(transform + 4), 4.0);
        assert_eq!(e.mem.f32(transform + 0xc), 9.0);
    }

    #[test]
    fn update_interpolates_the_path_at_the_percentage() {
        let mut e = path_engine();
        e.register(FLOAT_KEY_GEN_INTERP, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.register(CALC_REF_FRAME, |_, _| Ret::default());
        e.register(QUATERNION_FROM_ROTATION, |_, _| Ret::default());
        e.map(POS_KEY_INTERPOLATIONS, 4);
        e.mem.set_u32(POS_KEY_INTERPOLATIONS, 0x0090_0100);
        e.register(0x0090_0100, |e, a| {
            for (i, v) in [7.0f32, 8.0, 9.0].into_iter().enumerate() {
                e.mem.set_f32(a[3] + 4 * i as u32, v);
            }
            Ret::default()
        });
        let this = interpolator(&mut e);
        let transform = e.mem.alloc(0x20);
        assert!(!e
            .call(0x00a3f120, &args![this, 1.0f32, 0u32, transform])
            .bool());
        let percentage = key_data(&mut e, 3, 8, &[0.0, 1.0]);
        let path = key_data(&mut e, 0, 0x10, &[0.0, 1.0]);
        e.set(this, NiPathInterpolator::m_spPctData, percentage);
        e.set(this, NiPathInterpolator::m_spPathData, path);
        e.set(this, NiPathInterpolator::m_uFlags, 0x20);
        e.mem.set_f32(this.addr() + 0x48, 6.0);
        assert!(e
            .call(0x00a3f120, &args![this, 1.0f32, 0u32, transform])
            .bool());
        let pct_keys = e.mem.u32(percentage + KEY_DATA_KEYS);
        assert_eq!(
            calls_at(&e, FLOAT_KEY_GEN_INTERP),
            vec![vec![
                1.0f32.to_bits(),
                pct_keys,
                3,
                2,
                this.addr() + 0x14,
                8
            ]]
        );
        let frame = &calls_at(&e, CALC_REF_FRAME)[0];
        assert_eq!(frame[..4], [this.addr(), 0, 1, 0.5f32.to_bits()]);
        assert_eq!(
            calls_at(&e, QUATERNION_FROM_ROTATION)[0][0],
            this.addr() + 0x3c
        );
        assert_eq!(
            [0u32, 4, 8].map(|o| e.mem.f32(transform + o)),
            [7.0, 8.0, 9.0]
        );
        assert_eq!(e.mem.f32(this.addr() + 0x54), 9.0);
        assert_eq!(e.mem.f32(transform + 0x18), 6.0);
        assert_eq!(
            e.get(this, NiPathInterpolator::m_fLastTime),
            1.0f32.to_bits()
        );
    }

    #[test]
    fn copy_members_copies_flags_data_and_lengths() {
        let mut e = path_engine();
        e.register(KEY_BASED_INTERPOLATOR_COPY_MEMBERS, |_, _| Ret::default());
        e.register(MEMCPY, |_, _| Ret::default());
        let clone = e.mem.alloc(0x18);
        e.register_double(0x0090_0048, move |_, _| Ret {
            eax: clone,
            ..Ret::default()
        });
        let mut slots = vec![0u32; 0x13];
        slots[0x12] = 0x0090_0048;
        e.put_vtable(0x0090_1000, &slots);
        let this = interpolator(&mut e);
        let destination = interpolator(&mut e);
        let path = key_data(&mut e, 0, 8, &[0.0, 1.0]);
        e.mem.set_u32(path, 0x0090_1000);
        e.set(this, NiPathInterpolator::m_spPathData, path);
        e.set(this, NiPathInterpolator::m_uFlags, 0x7e);
        e.set(this, NiPathInterpolator::m_eBankDir, 2);
        e.set(this, NiPathInterpolator::m_fSmoothing, 0.5f32.to_bits());
        e.set(this, NiPathInterpolator::m_sFollowAxis, 1);
        e.set(this, NiPathInterpolator::m_pfPartialLength, 0x4000);
        e.set(this, NiPathInterpolator::m_fTotalLength, 3.0f32.to_bits());
        e.call(0x00a3f2f0, &args![this, destination, 0x5555u32]);
        assert_eq!(
            calls_at(&e, KEY_BASED_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this.addr(), destination.addr(), 0x5555]]
        );
        assert_eq!(e.get(destination, NiPathInterpolator::m_uFlags), 0x7e);
        assert_eq!(e.get(destination, NiPathInterpolator::m_spPathData), clone);
        assert_eq!(e.mem.u32(clone + 4), 1);
        assert_eq!(e.get(destination, NiPathInterpolator::m_eBankDir), 2);
        assert_eq!(
            e.get(destination, NiPathInterpolator::m_fSmoothing),
            0.5f32.to_bits()
        );
        assert_eq!(e.get(destination, NiPathInterpolator::m_sFollowAxis), 1);
        let copy = e.get(destination, NiPathInterpolator::m_pfPartialLength);
        assert_eq!(calls_at(&e, MEMCPY), vec![vec![copy, 0x4000, 8]]);
        assert_eq!(
            e.get(destination, NiPathInterpolator::m_fTotalLength),
            3.0f32.to_bits()
        );
        // Cleared bits are cleared in the destination; bit 4 with a stale destination
        // recomputes its total length.
        e.set(this, NiPathInterpolator::m_uFlags, 0x10);
        e.set(this, NiPathInterpolator::m_pfPartialLength, 0);
        e.set(
            destination,
            NiPathInterpolator::m_fTotalLength,
            1.5f32.to_bits(),
        );
        e.call(0x00a3f2f0, &args![this, destination, 0x5555u32]);
        assert_eq!(e.get(destination, NiPathInterpolator::m_uFlags), 0x10);
        e.set(this, NiPathInterpolator::m_uFlags, 0x11);
        e.set(
            this,
            NiPathInterpolator::m_fTotalLength,
            (-1.0f32).to_bits(),
        );
        e.call(0x00a3f2f0, &args![this, destination, 0x5555u32]);
        // The recompute runs on the destination (its total 3 from the copy before, kept),
        // clears its bit 0, and the source's total is copied last.
        assert_eq!(e.get(destination, NiPathInterpolator::m_uFlags), 0x10);
        assert_eq!(
            e.get(destination, NiPathInterpolator::m_fTotalLength),
            (-1.0f32).to_bits()
        );
    }

    #[test]
    fn create_clone_and_factory_construct_a_new_interpolator() {
        let mut e = path_engine();
        e.register(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, |_, _| Ret::default());
        e.register(KEY_BASED_INTERPOLATOR_COPY_MEMBERS, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        let this = interpolator(&mut e);
        let clone = e.call(0x00a3f520, &args![this, 0x5555u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), NI_PATH_INTERPOLATOR_VTABLE);
        assert_eq!(calls_at(&e, OPERATOR_NEW), vec![vec![0x5c]]);
        assert_eq!(
            calls_at(&e, KEY_BASED_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x5555]]
        );
        let made = e.call(0x00a3f4b0, &args![]).u32();
        assert_eq!(e.mem.u32(made), NI_PATH_INTERPOLATOR_VTABLE);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a3f4b0, &args![]).u32(), 0);
    }
}
