//! `NiAnimation/nipoint3interpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPoint3Interpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_POINT3_INTERPOLATOR_RTTI: u32 = 0x011f_3f70;

// Translated from 00a49640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_point3_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_POINT3_INTERPOLATOR_RTTI)
}

layout! {
    /// `NiPoint3Interpolator` (Xbox PDB), 0x20 bytes on PC (the same).
    pub struct NiPoint3Interpolator: 0x20 {
        /// `m_fLastTime` (Xbox PDB, from `NiInterpolator`).
        0x08 m_fLastTime: f32,
        /// `m_kPoint3Value` (Xbox PDB): an `NiPoint3`.
        0x0C m_kPoint3Value: Inline<NiPoint3>,
        /// `m_spPoint3Data` (Xbox PDB): `NiPointer<NiPosData>`.
        0x18 m_spPoint3Data: Ptr,
        /// `m_uiLastIdx` (Xbox PDB).
        0x1C m_uiLastIdx: u32,
    }
}

use crate::types::NiPoint3;

/// `NiPoint3Interpolator`'s vtable (RTTI `.?AVNiPoint3Interpolator@@`).
const NI_POINT3_INTERPOLATOR_VTABLE: u32 = 0x0109_881c;
/// The point the constructor stores as "no value" and `Update` tests for
/// (three floats at `011a9424`).
const NO_POINT: u32 = 0x011a_9424;
/// `00aa13e0` (cdecl, byte count): `NiNew`'s allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `operator delete` with a size (`00aa1460`, cdecl: pointer, size).
const NI_DELETE: u32 = 0x00aa_1460;
/// `NiKeyBasedInterpolator::NiKeyBasedInterpolator` (`00a3ca10`, thiscall).
const KEY_BASED_CONSTRUCT: u32 = 0x00a3_ca10;
/// The `NiKeyBasedInterpolator` destructor (`00a3ca30`, thiscall).
const KEY_BASED_DESTRUCT: u32 = 0x00a3_ca30;
/// `NiKeyBasedInterpolator::LoadBinary` (`00aa0ac0`, thiscall, stream).
const KEY_BASED_LOAD_BINARY: u32 = 0x00aa_0ac0;
/// The `NiKeyBasedInterpolator` copy of the members (`00aa0ab0`, thiscall:
/// clone, cloning process).
const KEY_BASED_COPY_MEMBERS: u32 = 0x00aa_0ab0;
/// `00a59be0` (`NiColor::LoadBinary` in the engine map, thiscall, stream):
/// reads three floats.
const POINT_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `NiStream::ResolveLinkID` (`00a64620`, thiscall).
const RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// `NiPosKey::GenInterp` (`00a27490`, cdecl: result, time, keys, key type,
/// key count, last index, key size).
const POS_KEY_GEN_INTERP: u32 = 0x00a2_7490;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// Releases a reference to `object` (non-null): `DeleteThis` (vtable +4)
/// when the count at +4 drops to zero.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

/// Assigns `value` to the `NiPointer` at `slot` when it differs.
fn assign_pointer(e: &mut Engine, slot: u32, value: u32) {
    let old = e.mem.u32(slot);
    if old == value {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(slot, value);
    if value != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![value + 4]);
    }
}

/// The member values a new interpolator gets after the base constructor:
/// the class vtable, the "no value" point, no data, index 0.
fn init_members(e: &mut Engine, object: u32) {
    e.mem.set_u32(object, NI_POINT3_INTERPOLATOR_VTABLE);
    for lane in 0..3 {
        let word = e.mem.u32(NO_POINT + 4 * lane);
        e.mem.set_u32(object + 0xc + 4 * lane, word);
    }
    e.mem.set_u32(object + 0x18, 0);
    e.mem.set_u32(object + 0x1c, 0);
}

/// Whether the point at `point` equals the "no value" point, lane by lane
/// as the exe's `FUCOMPP` tests (NaN never equal).
fn is_no_point(e: &mut Engine, point: u32) -> bool {
    (0..3).all(|lane| {
        let value = x87_load32(e.mem.u32(point + 4 * lane));
        let none = x87_load32(e.mem.u32(NO_POINT + 4 * lane));
        value == none
    })
}

/// Copies the three words of a point.
fn copy_point(e: &mut Engine, dest: u32, source: u32) {
    for lane in 0..3 {
        let word = e.mem.u32(source + 4 * lane);
        e.mem.set_u32(dest + 4 * lane, word);
    }
}

// Translated from 00a49a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator`'s create function (no function in Ghidra; read
/// from the bytes): allocates 0x20 bytes, runs `NiKeyBasedInterpolator`'s
/// constructor and sets the members (vtable, the "no value" point, no
/// data, index 0); null when the allocation fails.
pub fn fn_00a49a30(e: &mut Engine) -> Ptr<NiPoint3Interpolator> {
    let object = e.call(NI_ALLOCATE, &args![0x20u32]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    e.call(KEY_BASED_CONSTRUCT, &args![object]);
    init_members(e, object);
    Ptr::new(object)
}

// Translated from 00a49530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::LoadBinary` (Xbox PDB name in the engine map):
/// the base's `LoadBinary`, the point value (`00a59be0`), then the data
/// pointer from the stream's next link (`NiStream::ResolveLinkID`),
/// assigned as an `NiPointer`.
pub fn ni_point3_interpolator_load_binary(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    stream: Ptr,
) {
    e.call(KEY_BASED_LOAD_BINARY, &args![this, stream]);
    e.call(
        POINT_LOAD_BINARY,
        &args![
            this.byte_add(NiPoint3Interpolator::m_kPoint3Value.off),
            stream
        ],
    );
    let data = e.call(RESOLVE_LINK_ID, &args![stream]).u32();
    assign_pointer(
        e,
        this.addr() + NiPoint3Interpolator::m_spPoint3Data.off,
        data,
    );
}

// Translated from 00a49870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::Update` (Xbox PDB name in the engine map): at the
/// last time, writes the cached point to `value` and answers whether it is a
/// real value (not the "no value" point). Otherwise interpolates the keys
/// of the data (`NiPosKey::GenInterp`, when it has keys) into the cached
/// point; when that is the "no value" point answers false, else writes it
/// to `value`, stores the time as the last time and answers true.
/// `_interp_target` is not read.
pub fn ni_point3_interpolator_update(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    time: f32,
    _interp_target: u32,
    value: Ptr,
) -> bool {
    let t = this.addr();
    let point = t + NiPoint3Interpolator::m_kPoint3Value.off;
    let last_time = x87_load32(e.mem.u32(t + 8));
    let time_bits = x87_quiet_bits(time.to_bits());
    if x87_load32(time.to_bits()) == last_time {
        copy_point(e, value.addr(), point);
        return !is_no_point(e, point);
    }
    let data = e.get(this, NiPoint3Interpolator::m_spPoint3Data);
    if data.addr() != 0 {
        let d = data.addr();
        let count = e.mem.u32(d + 8);
        let key_size = e.mem.u8(d + 0x14) as u32;
        let key_type = e.mem.u32(d + 0x10);
        let keys = e.mem.u32(d + 0xc);
        if count != 0 {
            e.with_stack(0xc, |e, result| {
                let interpolated = e
                    .call(
                        POS_KEY_GEN_INTERP,
                        &args![
                            result,
                            time_bits,
                            keys,
                            key_type,
                            count,
                            t + NiPoint3Interpolator::m_uiLastIdx.off,
                            key_size
                        ],
                    )
                    .u32();
                copy_point(e, point, interpolated);
            });
        }
    }
    if is_no_point(e, point) {
        return false;
    }
    copy_point(e, value.addr(), point);
    e.mem.set_u32(t + 8, time_bits);
    true
}

// Translated from 00a499a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NiPoint3Interpolator::CopyMembers` by its
/// body and callers): the base's copy (`00aa0ab0`), then the point, the
/// data pointer (as an `NiPointer`) and the last index into `clone`.
pub fn fn_00a499a0(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    clone: Ptr<NiPoint3Interpolator>,
    cloning: Ptr,
) {
    let (t, c) = (this.addr(), clone.addr());
    e.call(KEY_BASED_COPY_MEMBERS, &args![this, clone, cloning]);
    copy_point(e, c + 0xc, t + 0xc);
    let data = e.mem.u32(t + 0x18);
    assign_pointer(e, c + 0x18, data);
    let index = e.mem.u32(t + 0x1c);
    e.mem.set_u32(c + 0x1c, index);
}

// Translated from 00a49ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator`'s scalar deleting destructor (Xbox PDB name in the
/// engine map): releases the data pointer, runs the base destructor
/// (`00a3ca30`) and frees the 0x20 bytes when bit 0 of `flags` is set.
/// Returns `this`.
pub fn ni_point3_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    flags: u32,
) -> Ptr<NiPoint3Interpolator> {
    let data = e.get(this, NiPoint3Interpolator::m_spPoint3Data).addr();
    if data != 0 {
        release(e, data);
    }
    e.call(KEY_BASED_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x20u32]);
    }
    this
}

// Translated from 00a49b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3Interpolator::CreateClone` (Xbox PDB name in the engine map): a
/// new interpolator (as `00a49a30` makes it) with the members copied
/// (`00a499a0`). When the allocation fails, the copy still runs with a null
/// clone (as the exe does) and null is returned.
pub fn ni_point3_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr<NiPoint3Interpolator>,
    cloning: Ptr,
) -> Ptr<NiPoint3Interpolator> {
    let object = e.call(NI_ALLOCATE, &args![0x20u32]).u32();
    if object != 0 {
        e.call(KEY_BASED_CONSTRUCT, &args![object]);
        init_members(e, object);
    }
    fn_00a499a0(e, this, Ptr::new(object), cloning);
    Ptr::new(object)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a49640, ni_point3_interpolator_get_rtti() -> Ptr),
        entry!(0x00a4_9a30, fn_00a49a30() -> Ptr<NiPoint3Interpolator>),
        entry!(
            0x00a4_9530,
            ni_point3_interpolator_load_binary(Ptr<NiPoint3Interpolator>, Ptr)
        ),
        entry!(
            0x00a4_9870,
            ni_point3_interpolator_update(Ptr<NiPoint3Interpolator>, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a4_99a0,
            fn_00a499a0(Ptr<NiPoint3Interpolator>, Ptr<NiPoint3Interpolator>, Ptr)
        ),
        entry!(
            0x00a4_9ae0,
            ni_point3_interpolator_scalar_deleting_destructor(
                Ptr<NiPoint3Interpolator>,
                u32,
            )
                -> Ptr<NiPoint3Interpolator>
        ),
        entry!(
            0x00a4_9b30,
            ni_point3_interpolator_create_clone(
                Ptr<NiPoint3Interpolator>,
                Ptr,
            ) -> Ptr<NiPoint3Interpolator>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_point3_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a49640, &args![]).u32(),
            NI_POINT3_INTERPOLATOR_RTTI
        );
    }

    /// Reference counting through the import stand-ins: plain decrement and
    /// increment of the count at the address.
    fn refcount_doubles(e: &mut Engine) {
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
        e.register(0x7000_0004, |_, _| Ret::default());
        e.put_vtable(0x9100_0000, &[0, 0x7000_0004]);
    }

    /// A reference-counted object with `count` references.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, 0x9100_0000);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn no_point(e: &mut Engine) {
        e.map(NO_POINT, 0xc);
        for lane in 0..3 {
            e.mem.set_f32(NO_POINT + 4 * lane, -3.0e38);
        }
    }

    fn deleted(e: &Engine) -> usize {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == 0x7000_0004)
            .count()
    }

    #[test]
    fn fn_00a49a30_creates_an_empty_interpolator() {
        let mut e = Engine::new();
        no_point(&mut e);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(KEY_BASED_CONSTRUCT, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_9a30, &[]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        let object = e.call(0x00a4_9a30, &[]).u32();
        assert_eq!(e.mem.u32(object), NI_POINT3_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.f32(object + 0x10), -3.0e38);
        assert_eq!(e.mem.u32(object + 0x18), 0);
        assert_eq!(e.mem.u32(object + 0x1c), 0);
    }

    #[test]
    fn load_binary_reads_the_point_and_links_the_data() {
        let mut e = Engine::new();
        refcount_doubles(&mut e);
        e.register(KEY_BASED_LOAD_BINARY, |_, _| Ret::default());
        e.register(POINT_LOAD_BINARY, |_, _| Ret::default());
        let data = counted(&mut e, 0);
        e.register_double(RESOLVE_LINK_ID, move |_, _| Ret {
            eax: data,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let old = counted(&mut e, 1);
        e.mem.set_u32(this + 0x18, old);
        e.call_log = Some(vec![]);
        e.call(0x00a4_9530, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(this + 0x18), data);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(deleted(&e), 1, "the old data had its last reference");
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (KEY_BASED_LOAD_BINARY, vec![this, 0x5000]));
        assert_eq!(log[2], (POINT_LOAD_BINARY, vec![this + 0xc, 0x5000]));
        // The same data again changes nothing.
        e.call(0x00a4_9530, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(data + 4), 1);
    }

    #[test]
    fn update_caches_and_interpolates() {
        let mut e = Engine::new();
        no_point(&mut e);
        e.register(POS_KEY_GEN_INTERP, |e, a| {
            let time = f32::from_bits(a[1]);
            for lane in 0..3 {
                e.mem.set_f32(a[0] + 4 * lane, time * (lane + 1) as f32);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x20);
        for lane in 0..3 {
            e.mem.set_f32(this + 0xc + 4 * lane, -3.0e38);
        }
        e.mem.set_f32(this + 8, 1.0);
        let value = e.mem.alloc(0xc);
        let update =
            |e: &mut Engine, time: f32| e.call(0x00a4_9870, &args![this, time, 0u32, value]).bool();
        assert!(!update(&mut e, 1.0), "cached, no value");
        assert_eq!(e.mem.f32(value), -3.0e38);
        assert!(!update(&mut e, 2.0), "no data");
        assert_eq!(e.mem.f32(this + 8), 1.0);

        let data = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0x18, data);
        assert!(!update(&mut e, 2.0), "no keys");
        e.mem.set_u32(data + 8, 3);
        e.mem.set_u32(data + 0xc, 0x4400);
        e.mem.set_u32(data + 0x10, 1);
        e.mem.set_u8(data + 0x14, 0x10);
        e.call_log = Some(vec![]);
        assert!(update(&mut e, 2.0));
        assert_eq!(e.mem.f32(value + 8), 6.0);
        assert_eq!(e.mem.f32(this + 8), 2.0);
        let log = e.call_log.take().unwrap();
        let interp: Vec<_> = log.iter().filter(|c| c.0 == POS_KEY_GEN_INTERP).collect();
        assert_eq!(
            interp[0].1[1..].to_vec(),
            vec![2.0f32.to_bits(), 0x4400, 1, 3, this + 0x1c, 0x10]
        );
        e.mem.set_f32(value + 8, 0.0);
        assert!(update(&mut e, 2.0), "cached value");
        assert_eq!(e.mem.f32(value + 8), 6.0);
    }

    #[test]
    fn fn_00a499a0_copies_the_point_data_and_index() {
        let mut e = Engine::new();
        refcount_doubles(&mut e);
        e.register(KEY_BASED_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let clone = e.mem.alloc(0x20);
        let data = counted(&mut e, 1);
        e.mem.set_u32(this + 0x18, data);
        e.mem.set_f32(this + 0x14, 4.5);
        e.mem.set_u32(this + 0x1c, 7);
        e.call_log = Some(vec![]);
        e.call(0x00a4_99a0, &args![this, clone, 0x6000u32]);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (KEY_BASED_COPY_MEMBERS, vec![this, clone, 0x6000])
        );
        assert_eq!(e.mem.f32(clone + 0x14), 4.5);
        assert_eq!(e.mem.u32(clone + 0x18), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(e.mem.u32(clone + 0x1c), 7);
    }

    #[test]
    fn scalar_deleting_destructor_releases_and_frees_on_bit_0() {
        let mut e = Engine::new();
        refcount_doubles(&mut e);
        e.register(KEY_BASED_DESTRUCT, |_, _| Ret::default());
        e.register(NI_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let data = counted(&mut e, 2);
        e.mem.set_u32(this + 0x18, data);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_9ae0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert!(!e.call_log.take().unwrap().iter().any(|c| c.0 == NI_DELETE));
        e.mem.set_u32(this + 0x18, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_9ae0, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(KEY_BASED_DESTRUCT, vec![this])));
        assert!(log.contains(&(NI_DELETE, vec![this, 0x20])));
    }

    #[test]
    fn create_clone_builds_and_copies() {
        let mut e = Engine::new();
        no_point(&mut e);
        refcount_doubles(&mut e);
        e.register(NI_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(KEY_BASED_CONSTRUCT, |_, _| Ret::default());
        e.register(KEY_BASED_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_f32(this + 0xc, 1.25);
        let clone = e.call(0x00a4_9b30, &args![this, 0x6000u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_POINT3_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.f32(clone + 0xc), 1.25);
        assert_eq!(e.mem.f32(clone + 0x10), 0.0);
    }
}
