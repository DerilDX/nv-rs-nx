//! `NiAnimation/nibsplineinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplineInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_B_SPLINE_INTERPOLATOR_RTTI: u32 = 0x011f_4240;

// Translated from 00a57f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_b_spline_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_B_SPLINE_INTERPOLATOR_RTTI)
}

// Translated from 00a581e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a57f80`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x1c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_b_spline_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_7f80;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x1cu32]);
    }
    this
}

/// `NiInterpolator`'s constructor (`00a56ff0`) and destructor (`00a57010`); `thiscall`,
/// no stack word.
const INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a5_6ff0;
const INTERPOLATOR_DESTRUCTOR: u32 = 0x00a5_7010;
/// `NiInterpolator::LoadBinary` (`00a88a40`; `thiscall`: stream).
const INTERPOLATOR_LOAD_BINARY: u32 = 0x00a8_8a40;
/// The base member copy `CopyMembers` runs first (`00aa1000`; `thiscall` on the source:
/// clone, cloning process).
const BASE_COPY_MEMBERS: u32 = 0x00aa_1000;
/// `NiObject::Clone` (`00a5d3f0`; `thiscall`: cloning process), answers the copy.
const OBJECT_CLONE: u32 = 0x00a5_d3f0;
/// `NiStream::ResolveLinkID` (`00a64620`; `thiscall`, no stack word): the next linked
/// object.
const RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// The `NiBSplineInterpolator` virtual table (`0109ae8c`).
const B_SPLINE_INTERPOLATOR_VTABLE: u32 = 0x0109_ae8c;
/// The floats the constructor copies into the start and end times (`0109ae80`,
/// `01015f5c`).
const START_TIME_DEFAULT: u32 = 0x0109_ae80;
const END_TIME_DEFAULT: u32 = 0x0101_5f5c;
/// The doubles `GetActiveTimeRange` compares the times with (`010231b0`, `010241b0`).
const UNSET_START: u32 = 0x0102_31b0;
const UNSET_END: u32 = 0x0102_41b0;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiRefObject::DeleteThis`, virtual slot 4.
const SLOT_DELETE_THIS: u32 = 0x04;

layout! {
    /// `NiBSplineInterpolator` (Xbox PDB), 0x1C bytes.
    pub struct NiBSplineInterpolator: 0x1C {
        /// `m_fLastTime` (Xbox PDB, `NiInterpolator`).
        0x08 m_fLastTime: u32,
        /// `m_fStartTime` (Xbox PDB).
        0x0C m_fStartTime: u32,
        /// `m_fEndTime` (Xbox PDB).
        0x10 m_fEndTime: u32,
        /// `m_spData` (Xbox PDB): `NiBSplineData*`.
        0x14 m_spData: u32,
        /// `m_spBasisData` (Xbox PDB): `NiBSplineBasisData*`.
        0x18 m_spBasisData: u32,
    }
}

/// `InterlockedIncrement` of an object's reference count at `+4`.
fn add_ref(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
}

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at `+4` and, when it
/// reaches zero, the object's `DeleteThis` (virtual slot 4).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// The inline `NiPointer::operator=` for the pointer stored at `slot`.
fn assign_pointer(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old != new {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, new);
        if new != 0 {
            add_ref(e, new);
        }
    }
}

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

// Translated from 00a57f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::NiBSplineInterpolator(NiBSplineData*, NiBSplineBasisData*)`
/// (unnamed in the engine map; `thiscall`, `RET 8`): the `NiInterpolator` constructor
/// (`00a56ff0`), the start and end times from the floats at `0109ae80` and `01015f5c`
/// (through the x87 stack), the class's virtual table, then both pointers stored and
/// counted (when not null). Answers `this`.
pub fn ni_b_spline_interpolator_constructor(
    e: &mut Engine,
    this: Ptr<NiBSplineInterpolator>,
    data: u32,
    basis_data: u32,
) -> Ptr<NiBSplineInterpolator> {
    e.call(INTERPOLATOR_CONSTRUCTOR, &args![this]);
    let start = x87_quiet_bits(e.global::<u32>(START_TIME_DEFAULT));
    e.set(this, NiBSplineInterpolator::m_fStartTime, start);
    let end = x87_quiet_bits(e.global::<u32>(END_TIME_DEFAULT));
    e.set(this, NiBSplineInterpolator::m_fEndTime, end);
    e.mem.set_u32(this.addr(), B_SPLINE_INTERPOLATOR_VTABLE);
    e.set(this, NiBSplineInterpolator::m_spData, data);
    if data != 0 {
        add_ref(e, data);
    }
    e.set(this, NiBSplineInterpolator::m_spBasisData, basis_data);
    if basis_data != 0 {
        add_ref(e, basis_data);
    }
    this
}

// Translated from 00a57fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::CopyMembers` (unnamed in the engine map; `thiscall` on the
/// source, `RET 8`: clone, cloning process). The base copy (`00aa1000`), the end and
/// start times (x87 copies), the data pointer assigned to the clone's (counted), and, when
/// the source has basis data, its clone (`NiObject::Clone`, `00a5d3f0`) assigned.
pub fn ni_b_spline_interpolator_copy_members(
    e: &mut Engine,
    this: Ptr<NiBSplineInterpolator>,
    clone: Ptr<NiBSplineInterpolator>,
    cloning: Ptr,
) {
    e.call(BASE_COPY_MEMBERS, &args![this, clone, cloning]);
    let end = x87_quiet_bits(e.get(this, NiBSplineInterpolator::m_fEndTime));
    e.set(clone, NiBSplineInterpolator::m_fEndTime, end);
    let start = x87_quiet_bits(e.get(this, NiBSplineInterpolator::m_fStartTime));
    e.set(clone, NiBSplineInterpolator::m_fStartTime, start);
    let slot = clone.addr() + 0x14;
    let old = e.mem.u32(slot);
    if old != e.get(this, NiBSplineInterpolator::m_spData) {
        if old != 0 {
            release(e, old);
        }
        let data = e.get(this, NiBSplineInterpolator::m_spData);
        e.mem.set_u32(slot, data);
        if data != 0 {
            add_ref(e, data);
        }
    }
    let basis_data = e.get(this, NiBSplineInterpolator::m_spBasisData);
    if basis_data != 0 {
        let copy = e.call(OBJECT_CLONE, &args![basis_data, cloning]).u32();
        assign_pointer(e, clone.addr() + 0x18, copy);
    }
}

// Translated from 00a57f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::~NiBSplineInterpolator` (Xbox PDB; `thiscall`): stores the
/// class's virtual table, releases the basis data and the data (when set), then the
/// `NiInterpolator` destructor (`00a57010`, a jump).
pub fn ni_b_spline_interpolator_destructor(e: &mut Engine, this: Ptr<NiBSplineInterpolator>) {
    e.mem.set_u32(this.addr(), B_SPLINE_INTERPOLATOR_VTABLE);
    let basis_data = e.get(this, NiBSplineInterpolator::m_spBasisData);
    if basis_data != 0 {
        release(e, basis_data);
    }
    let data = e.get(this, NiBSplineInterpolator::m_spData);
    if data != 0 {
        release(e, data);
    }
    e.call(INTERPOLATOR_DESTRUCTOR, &args![this]);
}

// Translated from 00a57d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::GetActiveTimeRange` (Xbox PDB; `thiscall`, `RET 8`: addresses of
/// the begin and end times). When the begin time equals the double at `010231b0` and the
/// end time the one at `010241b0` (the unset range; a NaN is unequal) both become 0;
/// otherwise they become the interpolator's start and end times (x87 copies).
pub fn ni_b_spline_interpolator_get_active_time_range(
    e: &mut Engine,
    this: Ptr<NiBSplineInterpolator>,
    begin: Ptr,
    end: Ptr,
) {
    let unset_start = f64::from_bits(e.mem.u64(UNSET_START));
    if x87_load32(e.mem.u32(begin.addr())) == unset_start {
        let unset_end = f64::from_bits(e.mem.u64(UNSET_END));
        if x87_load32(e.mem.u32(end.addr())) == unset_end {
            e.mem.set_u32(begin.addr(), 0);
            e.mem.set_u32(end.addr(), 0);
            return;
        }
    }
    let start = x87_quiet_bits(e.get(this, NiBSplineInterpolator::m_fStartTime));
    e.mem.set_u32(begin.addr(), start);
    let finish = x87_quiet_bits(e.get(this, NiBSplineInterpolator::m_fEndTime));
    e.mem.set_u32(end.addr(), finish);
}

// Translated from 00a58080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`: stream).
/// `NiInterpolator::LoadBinary` (`00a88a40`), the start and end times (one 4-byte read
/// each), then the data and the basis data as the next two linked objects
/// (`NiStream::ResolveLinkID`, `00a64620`), each assigned and counted.
pub fn ni_b_spline_interpolator_load_binary(
    e: &mut Engine,
    this: Ptr<NiBSplineInterpolator>,
    stream: Ptr,
) {
    e.call(INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
    read_word(e, stream, this.addr() + 0xc);
    read_word(e, stream, this.addr() + 0x10);
    let data = e.call(RESOLVE_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.addr() + 0x14, data);
    let basis_data = e.call(RESOLVE_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.addr() + 0x18, basis_data);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a57f70, ni_b_spline_interpolator_get_rtti() -> Ptr),
        entry!(0x00a581e0, ni_b_spline_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a5_7f20,
            ni_b_spline_interpolator_constructor(
                Ptr<NiBSplineInterpolator>,
                u32,
                u32,
            ) -> Ptr<NiBSplineInterpolator>
        ),
        entry!(
            0x00a5_7fd0,
            ni_b_spline_interpolator_copy_members(
                Ptr<NiBSplineInterpolator>,
                Ptr<NiBSplineInterpolator>,
                Ptr,
            )
        ),
        entry!(
            0x00a5_7f80,
            ni_b_spline_interpolator_destructor(Ptr<NiBSplineInterpolator>)
        ),
        entry!(
            0x00a5_7d60,
            ni_b_spline_interpolator_get_active_time_range(Ptr<NiBSplineInterpolator>, Ptr, Ptr)
        ),
        entry!(
            0x00a5_8080,
            ni_b_spline_interpolator_load_binary(Ptr<NiBSplineInterpolator>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_b_spline_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a57f70, &args![]).u32(),
            NI_B_SPLINE_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_b_spline_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
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
        assert_eq!(e.call(0x00a581e0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a581e0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x1c]]);
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

    const DELETE_THIS: u32 = 0x0310_0000;

    /// An engine with counting doubles and a virtual table whose `DeleteThis` is recorded.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(DELETE_THIS, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        e.call_log = Some(vec![]);
        (e, vtable)
    }

    fn object(e: &mut Engine, vtable: u32, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn constructor_counts_both_pointers() {
        let (mut e, vtable) = engine();
        e.register(INTERPOLATOR_CONSTRUCTOR, |_, _| Ret::default());
        e.map(START_TIME_DEFAULT, 4);
        e.set_global(START_TIME_DEFAULT, 0x7f80_0001u32);
        e.map(END_TIME_DEFAULT, 4);
        e.set_global(END_TIME_DEFAULT, 0xff7f_ffffu32);
        let data = object(&mut e, vtable, 0);
        let this = e.mem.alloc(0x1c);
        assert_eq!(e.call(0x00a5_7f20, &args![this, data, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), B_SPLINE_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x10), 0xff7f_ffff);
        assert_eq!((e.mem.u32(this + 0x14), e.mem.u32(this + 0x18)), (data, 0));
        assert_eq!(calls(&e, INTERLOCKED_INCREMENT), vec![vec![data + 4]]);
    }

    #[test]
    fn copy_members_shares_the_data_and_clones_the_basis() {
        let (mut e, vtable) = engine();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        let data = object(&mut e, vtable, 1);
        let old = object(&mut e, vtable, 1);
        let basis = object(&mut e, vtable, 1);
        let copy = object(&mut e, vtable, 0);
        e.register_double(OBJECT_CLONE, move |_, _| copy.into_ret());
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0xc, 1.5f32.to_bits());
        e.mem.set_u32(this + 0x10, 2.5f32.to_bits());
        e.mem.set_u32(this + 0x14, data);
        e.mem.set_u32(this + 0x18, basis);
        let clone = e.mem.alloc(0x1c);
        e.mem.set_u32(clone + 0x14, old);
        e.call(0x00a5_7fd0, &args![this, clone, 0x4000u32]);
        assert_eq!(
            calls(&e, BASE_COPY_MEMBERS),
            vec![vec![this, clone, 0x4000]]
        );
        assert_eq!(e.mem.f32(clone + 0xc), 1.5);
        assert_eq!(e.mem.f32(clone + 0x10), 2.5);
        assert_eq!(e.mem.u32(clone + 0x14), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![old]]);
        assert_eq!(calls(&e, OBJECT_CLONE), vec![vec![basis, 0x4000]]);
        assert_eq!(e.mem.u32(clone + 0x18), copy);
        assert_eq!(e.mem.u32(copy + 4), 1);
    }

    #[test]
    fn destructor_releases_both_pointers() {
        let (mut e, vtable) = engine();
        e.register(INTERPOLATOR_DESTRUCTOR, |_, _| Ret::default());
        let data = object(&mut e, vtable, 2);
        let basis = object(&mut e, vtable, 1);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x14, data);
        e.mem.set_u32(this + 0x18, basis);
        e.call(0x00a5_7f80, &args![this]);
        assert_eq!(e.mem.u32(this), B_SPLINE_INTERPOLATOR_VTABLE);
        assert_eq!(
            calls(&e, INTERLOCKED_DECREMENT),
            vec![vec![basis + 4], vec![data + 4]]
        );
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![basis]]);
        assert_eq!(calls(&e, INTERPOLATOR_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn active_time_range_is_zero_only_when_unset() {
        let mut e = Engine::new();
        e.map(UNSET_START, 8);
        e.mem.set_u64(UNSET_START, f64::from(f32::MAX).to_bits());
        e.map(UNSET_END, 8);
        e.mem.set_u64(UNSET_END, f64::from(f32::MIN).to_bits());
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0xc, 1.0f32.to_bits());
        e.mem.set_u32(this + 0x10, 3.0f32.to_bits());
        let range = e.mem.alloc(8);
        e.mem.set_u32(range, f32::MAX.to_bits());
        e.mem.set_u32(range + 4, f32::MIN.to_bits());
        e.call(0x00a5_7d60, &args![this, range, range + 4]);
        assert_eq!((e.mem.u32(range), e.mem.u32(range + 4)), (0, 0));
        e.mem.set_u32(range, f32::MAX.to_bits());
        e.mem.set_u32(range + 4, 0.0f32.to_bits());
        e.call(0x00a5_7d60, &args![this, range, range + 4]);
        assert_eq!((e.mem.f32(range), e.mem.f32(range + 4)), (1.0, 3.0));
        e.mem.set_u32(range, f32::NAN.to_bits());
        e.mem.set_u32(range + 4, f32::MIN.to_bits());
        e.call(0x00a5_7d60, &args![this, range, range + 4]);
        assert_eq!((e.mem.f32(range), e.mem.f32(range + 4)), (1.0, 3.0));
    }

    #[test]
    fn load_binary_reads_the_times_and_links_both_objects() {
        let (mut e, vtable) = engine();
        e.register(INTERPOLATOR_LOAD_BINARY, |_, _| Ret::default());
        let data = object(&mut e, vtable, 0);
        let basis = object(&mut e, vtable, 0);
        let mut linked = vec![basis, data];
        e.register_double(RESOLVE_LINK_ID, move |_, _| {
            linked.pop().unwrap().into_ret()
        });
        const READ: u32 = 0x0300_0000;
        e.register(READ, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], 0x3f80_0000);
            4u32.into_ret()
        });
        let input = e.mem.alloc(0xc);
        e.mem.set_u32(input + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, input);
        let this = e.mem.alloc(0x1c);
        e.call(0x00a5_8080, &args![this, stream]);
        assert_eq!(
            calls(&e, INTERPOLATOR_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(e.mem.u32(input + 4), 8);
        let reads = calls(&e, READ);
        assert_eq!((reads[0][1], reads[1][1]), (this + 0xc, this + 0x10));
        assert_eq!(
            (e.mem.u32(this + 0x14), e.mem.u32(this + 0x18)),
            (data, basis)
        );
        assert_eq!((e.mem.u32(data + 4), e.mem.u32(basis + 4)), (1, 1));
    }
}
