//! `NiParticle/nipsysairfieldmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c2e520`: the jump target of `00c2f4e0`.
const JUMP_TARGET_00C2F4E0: u32 = 0x00c2_e520;

/// `00c28b60`: the jump target of `00c2f4f0`.
const JUMP_TARGET_00C2F4F0: u32 = 0x00c2_8b60;

// Translated from 00c2f4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldModifier::LinkObject` (Xbox PDB): a 5-byte jump to `00c2e520` (engine map: `NiPSysBombModifier::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysAirFieldModifier@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_air_field_modifier_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C2F4E0, &args![this, stream]);
}

// Translated from 00c2f4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldModifier::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00c28b60` (engine map: `NiPSysAgeDeathModifier::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysAirFieldModifier@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_air_field_modifier_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00C2F4F0, &args![this, stream]).bool()
}

// Translated from 00c2f590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27a8.
pub fn ni_p_sys_air_field_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27a8
}

/// `NiPSysModifier::ProcessClone` (`00c317e0`, `thiscall`, one word).
const MODIFIER_PROCESS_CLONE: u32 = 0x00c3_17e0;
/// `00a53c30`: the cloning process's map lookup, `thiscall` on the
/// process's first word, `(key, &value)`; answers whether it found the key.
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiPSysFieldModifier::m_pkFieldObj` (Xbox PDB) +0x18: `NiAVObject*`.
const FIELD_OBJECT: u32 = 0x18;

// Translated from 00c2c110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldModifier::ProcessClone` (engine map): `NiPSysModifier`'s
/// `ProcessClone`, then the clone of `this` from the process's map (the
/// lookup's answer is not checked: the out word starts at 0) and, when the
/// field object is set, its clone from the map (the out word still holding
/// this object's clone) as the clone's field object, or the field object
/// itself when the map has none. The clone is not checked for null.
pub fn ni_p_sys_air_field_modifier_process_clone(e: &mut Engine, this: Ptr, process: Ptr) {
    e.call(MODIFIER_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, out| {
        let map = e.mem.u32(process.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, this, out]);
        let clone = e.mem.u32(out.addr());
        let field = e.mem.u32(this.addr() + FIELD_OBJECT);
        if field == 0 {
            return;
        }
        let map = e.mem.u32(process.addr());
        if e.call(CLONE_MAP_LOOKUP, &args![map, field, out]).bool() {
            let field_clone = e.mem.u32(out.addr());
            e.mem.set_u32(clone + FIELD_OBJECT, field_clone);
        } else {
            let field = e.mem.u32(this.addr() + FIELD_OBJECT);
            e.mem.set_u32(clone + FIELD_OBJECT, field);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c2f590, ni_p_sys_air_field_modifier_get_r_t_t_i() -> u32),
        entry!(
            0x00c2_f4e0,
            ni_p_sys_air_field_modifier_link_object(Ptr, Ptr)
        ),
        entry!(0x00c2_f4f0, ni_p_sys_air_field_modifier_register_streamables(Ptr, Ptr) -> bool),
        entry!(
            0x00c2_c110,
            ni_p_sys_air_field_modifier_process_clone(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c2f4e0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_f4e0;
        const TARGET: u32 = 0x00c2_e520;
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
    fn thunk_00c2f4f0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_f4f0;
        const TARGET: u32 = 0x00c2_8b60;
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
    fn ni_p_sys_air_field_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_f590, &args![]).u32(), 0x0120_27a8);
    }

    /// The clone map double: key 0x5000 (this) maps to the clone, the
    /// field object 0x7100 to 0x7200; other keys are absent.
    fn process_clone_engine() -> (Engine, Ptr, Ptr, u32) {
        let mut e = Engine::new();
        e.register(MODIFIER_PROCESS_CLONE, |_, _| Ret::default());
        let clone = e.mem.alloc(0x68);
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            let value = match a[1] {
                0x5000 => clone,
                0x7100 => 0x7200,
                _ => return Ret::default(),
            };
            e.mem.set_u32(a[2], value);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let this = Ptr::new(e.mem.alloc(0x68));
        let process = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(process.addr(), 0x0066_0000);
        (e, this, process, clone)
    }

    #[test]
    fn process_clone_maps_the_field_object() {
        let (mut e, _, process, clone) = process_clone_engine();
        // `this` must be the mapped key 0x5000: use a fresh block there.
        e.map(0x5000, 0x100);
        let this: Ptr = Ptr::new(0x5000);
        e.mem.set_u32(this.addr() + FIELD_OBJECT, 0x7100);
        e.call_log = Some(vec![]);
        e.call(0x00c2_c110, &args![this, process]);
        assert_eq!(e.mem.u32(clone + FIELD_OBJECT), 0x7200);
        let log = e.call_log.take().unwrap();
        let lookups: Vec<_> = log.iter().filter(|c| c.0 == CLONE_MAP_LOOKUP).collect();
        assert_eq!(lookups.len(), 2);
        assert_eq!(lookups[0].1[..2], [0x0066_0000, 0x5000]);
        assert_eq!(lookups[1].1[..2], [0x0066_0000, 0x7100]);
        let base: Vec<_> = log
            .iter()
            .filter(|c| c.0 == MODIFIER_PROCESS_CLONE)
            .collect();
        assert_eq!(base[0].1, [0x5000, process.addr()]);
    }

    #[test]
    fn process_clone_keeps_an_unmapped_field_object_and_skips_a_null_one() {
        let (mut e, _, process, clone) = process_clone_engine();
        e.map(0x5000, 0x100);
        let this: Ptr = Ptr::new(0x5000);
        e.mem.set_u32(this.addr() + FIELD_OBJECT, 0x7300);
        e.call(0x00c2_c110, &args![this, process]);
        assert_eq!(e.mem.u32(clone + FIELD_OBJECT), 0x7300);

        let (mut e, this, process, _) = process_clone_engine();
        e.call_log = Some(vec![]);
        e.call(0x00c2_c110, &args![this, process]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == CLONE_MAP_LOOKUP).count(), 1);
    }
}
