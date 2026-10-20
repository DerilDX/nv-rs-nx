//! `NiAnimation/niextradatacontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiExtraDataController::ms_RTTI` (the class's `NiRTTI` record).
const NI_EXTRA_DATA_CONTROLLER_RTTI: u32 = 0x011f_4218;

// Translated from 00a57290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_extra_data_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_EXTRA_DATA_CONTROLLER_RTTI)
}

/// The base class constructor (`00a38e00`) and destructor (`00a6d290`) the controller's
/// own run; `thiscall`, no stack word.
const BASE_CONSTRUCTOR: u32 = 0x00a3_8e00;
const BASE_DESTRUCTOR: u32 = 0x00a6_d290;
/// The base `ProcessClone` (`00a38f50`; `thiscall`: cloning process).
const BASE_PROCESS_CLONE: u32 = 0x00a3_8f50;
/// The base `PostLinkObject` (`00a89af0`; `thiscall`: stream).
const BASE_POST_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiObjectNET::GetExtraData` (`00a5bdd0`; `thiscall` on the target: address of the
/// name).
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
/// The cloning process's map lookup (`00a53c30`; `thiscall` on the map: key, address of
/// the result word; answers whether the key was found).
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiPointer` assignment (`0066b0d0`; `thiscall` on the pointer: the new object).
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The `NiExtraDataController` virtual table (`0109aa14`).
const EXTRA_DATA_CONTROLLER_VTABLE: u32 = 0x0109_aa14;
/// The pooled empty string's handle (`0109b220`), never counted.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiRefObject::DeleteThis`, virtual slot 4.
const SLOT_DELETE_THIS: u32 = 0x04;

layout! {
    /// `NiExtraDataController` (Xbox PDB), 0x40 bytes.
    pub struct NiExtraDataController: 0x40 {
        /// `m_pkTarget` (Xbox PDB, `NiTimeController`): `NiObjectNET*`.
        0x2C m_pkTarget: u32,
        /// `m_spInterpolator` (Xbox PDB, `NiSingleInterpController`).
        0x34 m_spInterpolator: u32,
        /// `m_kExtraDataName` (Xbox PDB): `NiFixedString` handle.
        0x38 m_kExtraDataName: u32,
        /// `m_spExtraData` (Xbox PDB): `NiExtraData*`.
        0x3C m_spExtraData: u32,
    }
}

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at `+4` and, when it
/// reaches zero, the object's `DeleteThis` (virtual slot 4).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

// Translated from 00a572a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController`'s constructor (unnamed in the engine map; `thiscall`, plain
/// `RET`): the base constructor (`00a38e00`), the class's virtual table, the extra data
/// name set to the empty string's handle (`0109b220`, read, not counted) and no extra data.
/// Answers `this`.
pub fn ni_extra_data_controller_constructor(
    e: &mut Engine,
    this: Ptr<NiExtraDataController>,
) -> Ptr<NiExtraDataController> {
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), EXTRA_DATA_CONTROLLER_VTABLE);
    let empty = e.global::<u32>(EMPTY_FIXED_STRING);
    e.set(this, NiExtraDataController::m_kExtraDataName, empty);
    e.set(this, NiExtraDataController::m_spExtraData, 0);
    this
}

// Translated from 00a57240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): looks the extra data up on the
/// target by name (`NiObjectNET::GetExtraData`, `00a5bdd0`) and assigns it to the
/// controller's extra data pointer (the old one released, the new one counted, when they
/// differ).
pub fn fn_00a57240(e: &mut Engine, this: Ptr<NiExtraDataController>) {
    let target = e.get(this, NiExtraDataController::m_pkTarget);
    let name = this.addr() + 0x38;
    let found = e.call(GET_EXTRA_DATA, &args![target, name]).u32();
    let old = e.get(this, NiExtraDataController::m_spExtraData);
    if old == found {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.set(this, NiExtraDataController::m_spExtraData, found);
    if found != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![found + 4]);
    }
}

// Translated from 00a57400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController::PostLinkObject` (Xbox PDB; `thiscall`, `RET 4`: stream): the
/// base `PostLinkObject` (`00a89af0`), then, with a target, the extra data looked up again
/// ([`fn_00a57240`]).
pub fn ni_extra_data_controller_post_link_object(
    e: &mut Engine,
    this: Ptr<NiExtraDataController>,
    stream: Ptr,
) {
    e.call(BASE_POST_LINK_OBJECT, &args![this, stream]);
    if e.get(this, NiExtraDataController::m_pkTarget) != 0 {
        fn_00a57240(e, this);
    }
}

// Translated from 00a57150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController::ProcessClone` (Xbox PDB; `thiscall`, `RET 4`: cloning process).
/// The base `ProcessClone` (`00a38f50`), then this controller's clone looked up in the
/// cloning process's map (`00a53c30`; the result goes into the argument's own word, which
/// keeps the cloning process when nothing is found). When this controller and the clone
/// both have a target, the clone of the extra data is looked up the same way and assigned
/// to the clone's extra data pointer (`0066b0d0`).
pub fn ni_extra_data_controller_process_clone(
    e: &mut Engine,
    this: Ptr<NiExtraDataController>,
    cloning: Ptr,
) {
    e.call(BASE_PROCESS_CLONE, &args![this, cloning]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), cloning.addr());
        let map = e.mem.u32(cloning.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        if e.get(this, NiExtraDataController::m_pkTarget) == 0 {
            return;
        }
        let clone = e.mem.u32(slot.addr());
        // NiTimeController::m_pkTarget (Xbox PDB) +0x2C of the clone.
        if e.mem.u32(clone.wrapping_add(0x2c)) == 0 {
            return;
        }
        let extra = e.get(this, NiExtraDataController::m_spExtraData);
        let map = e.mem.u32(cloning.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, extra, slot]);
        let found = e.mem.u32(slot.addr());
        e.call(POINTER_ASSIGN, &args![clone.wrapping_add(0x3c), found]);
    });
}

// Translated from 00a572d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController`'s destructor (unnamed in the engine map; `thiscall`): stores the
/// class's virtual table, releases the extra data, the extra data name (unless it is the
/// empty string) and the interpolator (`+0x34`), then the base destructor (`00a6d290`, a
/// jump).
pub fn ni_extra_data_controller_destructor(e: &mut Engine, this: Ptr<NiExtraDataController>) {
    e.mem.set_u32(this.addr(), EXTRA_DATA_CONTROLLER_VTABLE);
    let extra = e.get(this, NiExtraDataController::m_spExtraData);
    if extra != 0 {
        release(e, extra);
    }
    let name = e.get(this, NiExtraDataController::m_kExtraDataName);
    if name != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
    let interpolator = e.get(this, NiExtraDataController::m_spInterpolator);
    if interpolator != 0 {
        release(e, interpolator);
    }
    e.call(BASE_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a57290, ni_extra_data_controller_get_rtti() -> Ptr),
        entry!(
            0x00a5_72a0,
            ni_extra_data_controller_constructor(
                Ptr<NiExtraDataController>,
            ) -> Ptr<NiExtraDataController>
        ),
        entry!(0x00a5_7240, fn_00a57240(Ptr<NiExtraDataController>)),
        entry!(
            0x00a5_7400,
            ni_extra_data_controller_post_link_object(Ptr<NiExtraDataController>, Ptr)
        ),
        entry!(
            0x00a5_7150,
            ni_extra_data_controller_process_clone(Ptr<NiExtraDataController>, Ptr)
        ),
        entry!(
            0x00a5_72d0,
            ni_extra_data_controller_destructor(Ptr<NiExtraDataController>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_extra_data_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a57290, &args![]).u32(),
            NI_EXTRA_DATA_CONTROLLER_RTTI
        );
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
        e.map(EMPTY_FIXED_STRING, 4);
        e.set_global(EMPTY_FIXED_STRING, 0x0800u32);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        e.call_log = Some(vec![]);
        (e, vtable)
    }

    fn object(e: &mut Engine, vtable: u32, count: u32) -> u32 {
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn constructor_starts_with_the_empty_name() {
        let (mut e, _) = engine();
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x3c, 0x1234);
        assert_eq!(e.call(0x00a5_72a0, &args![this]).u32(), this);
        assert_eq!(calls(&e, BASE_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), EXTRA_DATA_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x38), 0x0800);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert!(calls(&e, INTERLOCKED_INCREMENT).is_empty());
    }

    #[test]
    fn extra_data_lookup_replaces_the_pointer() {
        let (mut e, vtable) = engine();
        let old = object(&mut e, vtable, 1);
        let found = object(&mut e, vtable, 0);
        e.register_double(GET_EXTRA_DATA, move |_, _| found.into_ret());
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x2c, 0x7000);
        e.mem.set_u32(this + 0x3c, old);
        e.call(0x00a5_7240, &args![this]);
        assert_eq!(calls(&e, GET_EXTRA_DATA), vec![vec![0x7000, this + 0x38]]);
        assert_eq!(e.mem.u32(this + 0x3c), found);
        assert_eq!(e.mem.u32(found + 4), 1);
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![old]]);
        // The same object again: nothing is counted.
        e.call(0x00a5_7240, &args![this]);
        assert_eq!(e.mem.u32(found + 4), 1);
    }

    #[test]
    fn post_link_object_looks_up_only_with_a_target() {
        let (mut e, _) = engine();
        e.register(BASE_POST_LINK_OBJECT, |_, _| Ret::default());
        e.register(GET_EXTRA_DATA, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call(0x00a5_7400, &args![this, 0x5000u32]);
        assert!(calls(&e, GET_EXTRA_DATA).is_empty());
        e.mem.set_u32(this + 0x2c, 0x7000);
        e.call(0x00a5_7400, &args![this, 0x5000u32]);
        assert_eq!(calls(&e, GET_EXTRA_DATA).len(), 1);
        assert_eq!(
            calls(&e, BASE_POST_LINK_OBJECT),
            vec![vec![this, 0x5000]; 2]
        );
    }

    #[test]
    fn process_clone_assigns_the_cloned_extra_data() {
        let (mut e, _) = engine();
        e.register(BASE_PROCESS_CLONE, |_, _| Ret::default());
        e.register(POINTER_ASSIGN, |_, _| Ret::default());
        let clone = e.mem.alloc(0x40);
        e.mem.set_u32(clone + 0x2c, 0x7100);
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            let found = if a[1] == 0x6600 { 0x6611 } else { clone };
            e.mem.set_u32(a[2], found);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x2c, 0x7000);
        e.mem.set_u32(this + 0x3c, 0x6600);
        let cloning = e.mem.alloc(4);
        e.mem.set_u32(cloning, 0x9900);
        e.call(0x00a5_7150, &args![this, cloning]);
        assert_eq!(calls(&e, BASE_PROCESS_CLONE), vec![vec![this, cloning]]);
        let lookups = calls(&e, CLONE_MAP_LOOKUP);
        assert_eq!((lookups[0][0], lookups[0][1]), (0x9900, this));
        assert_eq!((lookups[1][0], lookups[1][1]), (0x9900, 0x6600));
        assert_eq!(calls(&e, POINTER_ASSIGN), vec![vec![clone + 0x3c, 0x6611]]);
    }

    #[test]
    fn process_clone_stops_without_targets() {
        let (mut e, _) = engine();
        e.register(BASE_PROCESS_CLONE, |_, _| Ret::default());
        e.register(POINTER_ASSIGN, |_, _| Ret::default());
        let clone = e.mem.alloc(0x40);
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            e.mem.set_u32(a[2], clone);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0x40);
        let cloning = e.mem.alloc(4);
        e.call(0x00a5_7150, &args![this, cloning]);
        assert_eq!(calls(&e, CLONE_MAP_LOOKUP).len(), 1);
        // A target here but none on the clone.
        e.mem.set_u32(this + 0x2c, 0x7000);
        e.call(0x00a5_7150, &args![this, cloning]);
        assert_eq!(calls(&e, CLONE_MAP_LOOKUP).len(), 2);
        assert!(calls(&e, POINTER_ASSIGN).is_empty());
    }

    #[test]
    fn destructor_releases_the_members() {
        let (mut e, vtable) = engine();
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        let extra = object(&mut e, vtable, 1);
        let interpolator = object(&mut e, vtable, 2);
        let name = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x34, interpolator);
        e.mem.set_u32(this + 0x38, name + 8);
        e.mem.set_u32(this + 0x3c, extra);
        e.call(0x00a5_72d0, &args![this]);
        assert_eq!(e.mem.u32(this), EXTRA_DATA_CONTROLLER_VTABLE);
        assert_eq!(
            calls(&e, INTERLOCKED_DECREMENT),
            vec![vec![extra + 4], vec![name], vec![interpolator + 4]]
        );
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![extra]]);
        assert_eq!(calls(&e, BASE_DESTRUCTOR), vec![vec![this]]);
        // The empty name is not released.
        e.mem.set_u32(this + 0x38, 0x0800);
        e.mem.set_u32(this + 0x34, 0);
        e.mem.set_u32(this + 0x3c, 0);
        e.call(0x00a5_72d0, &args![this]);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 3);
    }
}
