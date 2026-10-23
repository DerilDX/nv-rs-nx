//! `NiParticle/nipsysmodifierctlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a35850`: the jump target of `00a57630`.
const JUMP_TARGET_00A57630: u32 = 0x00a3_5850;

/// `00a38fe0`: the jump target of `00a57780`.
const JUMP_TARGET_00A57780: u32 = 0x00a3_8fe0;

// Translated from 00a57630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::LinkObject` (Xbox PDB): a 5-byte jump to `00a35850` (engine map: `NiSingleInterpController::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysModifierCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_modifier_ctlr_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A57630, &args![this, stream]);
}

// Translated from 00a57780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a38fe0` (engine map: `NiSingleInterpController::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysModifierCtlr@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_modifier_ctlr_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A57780, &args![this, stream]).bool()
}

// Translated from 00c31290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27c8.
pub fn ni_p_sys_modifier_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27c8
}

use crate::units::nimain::nitimecontroller::NiTimeController;
use crate::units::niparticle::nipsysmodifier::{
    load_name, CLONE_MAP_LOOKUP, INTERLOCKED_DECREMENT, INTERLOCKED_INCREMENT,
};

layout! {
    /// `NiPSysModifierCtlr` (Xbox PDB), 0x40 bytes; the PC code uses the
    /// same offsets. The base's target is `NiTimeController::m_pkTarget`.
    pub struct NiPSysModifierCtlr: 0x40 {
        /// `NiSingleInterpController::m_spInterpolator` (Xbox PDB):
        /// `NiPointer<NiInterpolator>`.
        0x34 m_spInterpolator: u32,
        /// `m_kModifierName` (Xbox PDB): an `NiFixedString` handle.
        0x38 m_kModifierName: u32,
        /// `m_pkModifier` (Xbox PDB): `NiPSysModifier*`.
        0x3C m_pkModifier: u32,
    }
}

/// `00a2f8a0` (unnamed; a 5-byte jump): the base member copy, `thiscall`,
/// `(dest, process)`.
const BASE_COPY_MEMBERS: u32 = 0x00a2_f8a0;
/// `00a5d3f0` (unnamed, `NiMain/niobject.cpp`): the clone of an object for
/// a cloning process (`thiscall` on the object, one word): its entry in the
/// process's map, else a new clone through vtable slot +0x48.
const CLONE_FOR_PROCESS: u32 = 0x00a5_d3f0;
/// `NiSingleInterpController::LoadBinary` (`00a38f80`, one word).
const SINGLE_INTERP_LOAD_BINARY: u32 = 0x00a3_8f80;
/// `NiSingleInterpController::ProcessClone` (`00a38f50`, one word).
const SINGLE_INTERP_PROCESS_CLONE: u32 = 0x00a3_8f50;
/// `00a89af0` (unnamed): the base post-link step, `thiscall`, one word.
const BASE_POST_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiParticleSystem`'s modifier list head (+0xc8, the list's first item;
/// items are `[next, previous, element]`), as the PC code reads it.
const SYSTEM_MODIFIER_LIST_HEAD: u32 = 0xc8;

// Translated from 00a38ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names it `NiPSysModifierCtlr::CopyMembers`, but the body
/// (folded code; it is also reached from other classes' `CopyMembers` and
/// `CreateClone`) copies only the single-interpolator controller's part:
/// the base copy `00a2f8a0` into `dest`, then, when `this` has an
/// interpolator, its clone for the process (`00a5d3f0`) becomes `dest`'s
/// interpolator (`NiPointer` assignment: the old one released and deleted
/// through vtable slot +4 at zero, the new one held). `thiscall`, `RET 8`.
pub fn fn_00a38ef0(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    dest: Ptr<NiPSysModifierCtlr>,
    process: Ptr,
) {
    e.call(BASE_COPY_MEMBERS, &args![this, dest, process]);
    let interpolator = e.get(this, NiPSysModifierCtlr::m_spInterpolator);
    if interpolator == 0 {
        return;
    }
    let clone = e
        .call(CLONE_FOR_PROCESS, &args![interpolator, process])
        .u32();
    let old = e.get(dest, NiPSysModifierCtlr::m_spInterpolator);
    if old == clone {
        return;
    }
    if old != 0 {
        let left = e
            .call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    e.set(dest, NiPSysModifierCtlr::m_spInterpolator, clone);
    if clone != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![clone.wrapping_add(4)]);
    }
}

// Translated from 00a571a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::LoadBinary` (Xbox PDB): the single-interpolator
/// controller's `LoadBinary`, then the modifier name (a C string before
/// file version 20.1.0.1, a fixed string from it).
pub fn ni_p_sys_modifier_ctlr_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    stream: Ptr,
) {
    e.call(SINGLE_INTERP_LOAD_BINARY, &args![this, stream]);
    load_name(
        e,
        stream.addr(),
        this.addr() + NiPSysModifierCtlr::m_kModifierName.off,
    );
}

// Translated from 00c31340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::ProcessClone` (Xbox PDB): the single-interpolator
/// controller's `ProcessClone`, then the clone of `this` from the process's
/// map (into a stack word cleared first); when both `this` and the clone
/// have a target, the clone of the modifier (looked up into the same word)
/// becomes the clone's modifier.
pub fn ni_p_sys_modifier_ctlr_process_clone(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    process: Ptr,
) {
    e.call(SINGLE_INTERP_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, slot| {
        let map = e.mem.u32(process.addr());
        e.mem.set_u32(slot.addr(), 0);
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        let clone = Ptr::<NiTimeController>::new(e.mem.u32(slot.addr()));
        if e.get(
            this.cast::<NiTimeController>(),
            NiTimeController::m_pkTarget,
        ) == 0
        {
            return;
        }
        if e.get(clone, NiTimeController::m_pkTarget) == 0 {
            return;
        }
        let modifier = e.get(this, NiPSysModifierCtlr::m_pkModifier);
        let map = e.mem.u32(process.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, modifier, slot]);
        let modifier_clone = e.mem.u32(slot.addr());
        e.set(
            clone.cast::<NiPSysModifierCtlr>(),
            NiPSysModifierCtlr::m_pkModifier,
            modifier_clone,
        );
    });
}

// Translated from 00c31400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: finds the controller's modifier by name. Walks
/// the target particle system's modifier list (head at +0xc8); the first
/// non-null modifier whose name (`NiPSysModifier::m_kName`, +8) is the
/// controller's modifier name becomes `m_pkModifier`, else it is cleared.
pub fn fn_00c31400(e: &mut Engine, this: Ptr<NiPSysModifierCtlr>) {
    let target = e.get(
        this.cast::<NiTimeController>(),
        NiTimeController::m_pkTarget,
    );
    let mut item = e.mem.u32(target.wrapping_add(SYSTEM_MODIFIER_LIST_HEAD));
    while item != 0 {
        let modifier = e.mem.u32(item + 8);
        item = e.mem.u32(item);
        if modifier != 0
            && e.mem.u32(modifier + 8) == e.get(this, NiPSysModifierCtlr::m_kModifierName)
        {
            e.set(this, NiPSysModifierCtlr::m_pkModifier, modifier);
            return;
        }
    }
    e.set(this, NiPSysModifierCtlr::m_pkModifier, 0);
}

// Translated from 00c31590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::PostLinkObject` (Xbox PDB): the base post-link step
/// (`00a89af0`), then, with a target, the modifier lookup (`00c31400`).
pub fn ni_p_sys_modifier_ctlr_post_link_object(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    stream: Ptr,
) {
    e.call(BASE_POST_LINK_OBJECT, &args![this, stream]);
    if e.get(
        this.cast::<NiTimeController>(),
        NiTimeController::m_pkTarget,
    ) != 0
    {
        fn_00c31400(e, this);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a3_8ef0,
            fn_00a38ef0(Ptr<NiPSysModifierCtlr>, Ptr<NiPSysModifierCtlr>, Ptr)
        ),
        entry!(
            0x00a5_71a0,
            ni_p_sys_modifier_ctlr_load_binary(Ptr<NiPSysModifierCtlr>, Ptr)
        ),
        entry!(
            0x00c3_1340,
            ni_p_sys_modifier_ctlr_process_clone(Ptr<NiPSysModifierCtlr>, Ptr)
        ),
        entry!(0x00c3_1400, fn_00c31400(Ptr<NiPSysModifierCtlr>)),
        entry!(
            0x00c3_1590,
            ni_p_sys_modifier_ctlr_post_link_object(Ptr<NiPSysModifierCtlr>, Ptr)
        ),
        entry!(0x00c31290, ni_p_sys_modifier_ctlr_get_r_t_t_i() -> u32),
        entry!(0x00a5_7630, ni_p_sys_modifier_ctlr_link_object(Ptr, Ptr)),
        entry!(0x00a5_7780, ni_p_sys_modifier_ctlr_register_streamables(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a57630_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a5_7630;
        const TARGET: u32 = 0x00a3_5850;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a57780_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a5_7780;
        const TARGET: u32 = 0x00a3_8fe0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn ni_p_sys_modifier_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_1290, &args![]).u32(), 0x0120_27c8);
    }

    use crate::units::niparticle::nipsysmodifier::{test_stream, LOAD_FIXED_STRING};

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A reference-counted object with a vtable whose slot +4 is
    /// `delete_this`.
    fn ref_object(e: &mut Engine, count: u32, delete_this: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, delete_this);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn copy_members_assigns_the_interpolator_clone() {
        const DELETE: u32 = 0x0f10_0000;
        let mut e = Engine::new();
        test_stream::interlocked(&mut e);
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        let this = e.new_object::<NiPSysModifierCtlr>();
        let dest = e.new_object::<NiPSysModifierCtlr>();
        // No interpolator: only the base copy runs.
        e.call_log = Some(vec![]);
        e.call(0x00a3_8ef0, &args![this, dest, 0x4000u32]);
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![this.addr(), dest.addr(), 0x4000]]
        );
        assert!(calls_to(&e, CLONE_FOR_PROCESS).is_empty());
        let old = ref_object(&mut e, 1, DELETE);
        let clone = ref_object(&mut e, 0, DELETE);
        e.set(this, NiPSysModifierCtlr::m_spInterpolator, 0x7000);
        e.set(dest, NiPSysModifierCtlr::m_spInterpolator, old);
        e.register_double(CLONE_FOR_PROCESS, move |_, _| Ret {
            eax: clone,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a3_8ef0, &args![this, dest, 0x4000u32]);
        assert_eq!(calls_to(&e, CLONE_FOR_PROCESS), vec![vec![0x7000, 0x4000]]);
        assert_eq!(calls_to(&e, DELETE), vec![vec![old]]);
        assert_eq!(e.get(dest, NiPSysModifierCtlr::m_spInterpolator), clone);
        assert_eq!(e.mem.u32(clone + 4), 1);
        // The same clone again: no reference changes.
        e.call_log = Some(vec![]);
        e.call(0x00a3_8ef0, &args![this, dest, 0x4000u32]);
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
    }

    #[test]
    fn load_binary_reads_the_modifier_name() {
        let mut e = Engine::new();
        e.register(SINGLE_INTERP_LOAD_BINARY, |_, _| Ret::default());
        e.register(LOAD_FIXED_STRING, |_, _| Ret::default());
        let stream = test_stream::new(&mut e, 0x1402_0000, &[]);
        let this = e.new_object::<NiPSysModifierCtlr>();
        e.call_log = Some(vec![]);
        e.call(0x00a5_71a0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, SINGLE_INTERP_LOAD_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(
            calls_to(&e, LOAD_FIXED_STRING),
            vec![vec![stream, this.addr() + 0x38]]
        );
    }

    #[test]
    fn process_clone_maps_the_modifier_only_with_targets() {
        for (this_target, clone_target, expected) in [(1u32, 1u32, 0x6100u32), (0, 1, 0), (1, 0, 0)]
        {
            let mut e = Engine::new();
            e.register(SINGLE_INTERP_PROCESS_CLONE, |_, _| Ret::default());
            let this = e.new_object::<NiPSysModifierCtlr>();
            let clone = e.new_object::<NiPSysModifierCtlr>();
            e.mem.set_u32(this.addr() + 0x2c, this_target);
            e.mem.set_u32(clone.addr() + 0x2c, clone_target);
            e.set(this, NiPSysModifierCtlr::m_pkModifier, 0x6000);
            let map = e.mem.alloc(4);
            e.mem.set_u32(map, clone.addr());
            let process = e.mem.alloc(4);
            e.mem.set_u32(process, map);
            e.register(CLONE_MAP_LOOKUP, |e, a| {
                let value = if a[1] == 0x6000 {
                    0x6100
                } else {
                    e.mem.u32(a[0])
                };
                e.mem.set_u32(a[2], value);
                Ret::default()
            });
            e.call(0x00c3_1340, &args![this, process]);
            assert_eq!(e.get(clone, NiPSysModifierCtlr::m_pkModifier), expected);
        }
    }

    #[test]
    fn modifier_lookup_finds_the_named_modifier_or_clears() {
        let mut e = Engine::new();
        let system = e.mem.alloc(0xd0);
        let this = e.new_object::<NiPSysModifierCtlr>();
        e.mem.set_u32(this.addr() + 0x2c, system);
        e.set(this, NiPSysModifierCtlr::m_kModifierName, 0x55);
        let first = e.mem.alloc(0x18);
        e.mem.set_u32(first + 8, 0x44);
        let second = e.mem.alloc(0x18);
        e.mem.set_u32(second + 8, 0x55);
        let items: Vec<u32> = (0..3).map(|_| e.mem.alloc(12)).collect();
        e.mem.set_u32(items[0], items[1]);
        e.mem.set_u32(items[0] + 8, first);
        e.mem.set_u32(items[1], items[2]);
        e.mem.set_u32(items[2] + 8, second);
        e.mem.set_u32(system + 0xc8, items[0]);
        e.call(0x00c3_1400, &args![this]);
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_pkModifier), second);
        e.set(this, NiPSysModifierCtlr::m_kModifierName, 0x66);
        e.call(0x00c3_1400, &args![this]);
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_pkModifier), 0);
    }

    #[test]
    fn post_link_object_looks_up_the_modifier_only_with_a_target() {
        let mut e = Engine::new();
        e.register(BASE_POST_LINK_OBJECT, |_, _| Ret::default());
        let this = e.new_object::<NiPSysModifierCtlr>();
        e.set(this, NiPSysModifierCtlr::m_pkModifier, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1590, &args![this, 0x7000u32]);
        assert_eq!(
            calls_to(&e, BASE_POST_LINK_OBJECT),
            vec![vec![this.addr(), 0x7000]]
        );
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_pkModifier), 0x99);
        let system = e.mem.alloc(0xd0);
        e.mem.set_u32(this.addr() + 0x2c, system);
        e.call(0x00c3_1590, &args![this, 0x7000u32]);
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_pkModifier), 0);
    }
}
