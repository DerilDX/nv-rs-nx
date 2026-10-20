//! `fallout shared/tesactorbase.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00401170`: thiscall getter, the byte at `this + 4`.
const GET_TYPE_BYTE: u32 = 0x0040_1170;
/// `00461580`: thiscall (`RET 4`), true when `this[4] & mask` is non-zero.
const TEST_FLAGS: u32 = 0x0046_1580;
/// The type byte the function accepts.
const ACCEPTED_TYPE: u32 = 0x2b;
/// The offset, from `this`, of the object whose flags are tested.
const FLAGS_OBJECT_OFFSET: u32 = 0x30;
/// The mask passed to the flag test.
const FLAG_MASK: u32 = 0x20;

// Translated from 005f0c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `tesactorbase.cpp`). `__fastcall`: false
/// unless `00401170(this)` returns `0x2b`; then whether `00461580` on
/// `this + 0x30` with mask `0x20` is true.
pub fn fn_005f0c40(e: &mut Engine, this: Ptr) -> bool {
    if e.call(GET_TYPE_BYTE, &args![this]).u32() != ACCEPTED_TYPE {
        return false;
    }
    let flags_object = this.addr() + FLAGS_OBJECT_OFFSET;
    e.call(TEST_FLAGS, &args![flags_object, FLAG_MASK]).bool()
}

/// The form type byte `TESActorBase::IsImmobile` accepts.
const IMMOBILE_TYPE: u32 = 0x2b;
/// The mask `TESActorBase::IsImmobile` tests.
const IMMOBILE_FLAG_MASK: u32 = 0x0080_0000;
/// The type byte `TESActorBase::GetSex` accepts.
const SEX_TYPE: u32 = 0x2a;
/// The mask `TESActorBase::GetSex` tests.
const SEX_FLAG_MASK: u32 = 1;

// Translated from 005f0c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBase::IsImmobile` (Xbox PDB): `__fastcall`. False unless `00401170(this)` returns
/// `0x2b`; then whether `00461580` on `this + 0x30` with mask `0x800000` is true.
pub fn tes_actor_base_is_immobile(e: &mut Engine, this: Ptr) -> bool {
    if e.call(GET_TYPE_BYTE, &args![this]).u32() != IMMOBILE_TYPE {
        return false;
    }
    let flags_object = this.addr().wrapping_add(FLAGS_OBJECT_OFFSET);
    e.call(TEST_FLAGS, &args![flags_object, IMMOBILE_FLAG_MASK])
        .bool()
}

// Translated from 005f0cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBase::GetSex` (Xbox PDB): `__fastcall`. `-1` unless `00401170(this)` returns
/// `0x2a`; then `1` when `00461580` on `this + 0x30` with mask `1` is true, else `0`.
pub fn tes_actor_base_get_sex(e: &mut Engine, this: Ptr) -> i32 {
    if e.call(GET_TYPE_BYTE, &args![this]).u32() != SEX_TYPE {
        return -1;
    }
    let flags_object = this.addr().wrapping_add(FLAGS_OBJECT_OFFSET);
    i32::from(
        e.call(TEST_FLAGS, &args![flags_object, SEX_FLAG_MASK])
            .bool(),
    )
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_0c80, tes_actor_base_is_immobile(Ptr) -> bool),
        entry!(0x005f_0cc0, tes_actor_base_get_sex(Ptr) -> i32),
        entry!(0x005f0c40, fn_005f0c40(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine(type_byte: u32, flags_result: bool) -> Engine {
        let mut e = Engine::new();
        e.register_double(GET_TYPE_BYTE, move |_, _| Ret {
            eax: type_byte,
            ..Ret::default()
        });
        e.register_double(TEST_FLAGS, move |_, _| Ret {
            eax: flags_result as u32,
            ..Ret::default()
        });
        e
    }

    #[test]
    fn other_types_are_false_without_testing_the_flags() {
        let mut e = engine(0x2a, true);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005f_0c40, &args![Ptr::<()>::new(0x1000)]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(addr, _)| *addr != TEST_FLAGS));
    }

    #[test]
    fn type_0x2b_returns_the_flag_test_on_the_embedded_object() {
        let mut e = engine(0x2b, true);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005f_0c40, &args![Ptr::<()>::new(0x1000)]).bool());
        let log = e.call_log.take().unwrap();
        let test = log.iter().find(|(addr, _)| *addr == TEST_FLAGS).unwrap();
        assert_eq!(test.1, vec![0x1030, 0x20]);
        let mut e = engine(0x2b, false);
        assert!(!e.call(0x005f_0c40, &args![Ptr::<()>::new(0x1000)]).bool());
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig(kind: u32, flag_set: bool) -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register_double(GET_TYPE_BYTE, move |_, _| kind.into_ret());
        e.register_double(TEST_FLAGS, move |_, _| u32::from(flag_set).into_ret());
        e
    }

    #[test]
    fn at_005f0c80_is_immobile_only_for_type_0x2b_with_the_flag() {
        let mut e = rig(0x2b, true);
        assert!(e.call(0x005f_0c80, &args![0x4000u32]).bool());
        assert_eq!(calls(&e, TEST_FLAGS), vec![vec![0x4030, 0x0080_0000]]);
        let mut e = rig(0x2b, false);
        assert!(!e.call(0x005f_0c80, &args![0x4000u32]).bool());
        let mut e = rig(0x2a, true);
        assert!(!e.call(0x005f_0c80, &args![0x4000u32]).bool());
        assert!(calls(&e, TEST_FLAGS).is_empty());
    }

    #[test]
    fn at_005f0cc0_answers_the_sex_for_type_0x2a_and_minus_one_otherwise() {
        let mut e = rig(0x2a, true);
        assert_eq!(e.call(0x005f_0cc0, &args![0x4000u32]).i32(), 1);
        assert_eq!(calls(&e, TEST_FLAGS), vec![vec![0x4030, 1]]);
        let mut e = rig(0x2a, false);
        assert_eq!(e.call(0x005f_0cc0, &args![0x4000u32]).i32(), 0);
        let mut e = rig(0x2b, true);
        assert_eq!(e.call(0x005f_0cc0, &args![0x4000u32]).i32(), -1);
        assert_eq!(e.call(0x005f_0cc0, &args![0x4000u32]).u32(), 0xffff_ffff);
        assert!(calls(&e, TEST_FLAGS).is_empty());
    }
}
