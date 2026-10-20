//! `(unplaced)/run_004ae6a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkCharacterProxy::operatorP` (`004ae750`, `magicballprojectile.rs`): `thiscall`; null for
/// null, otherwise what `00620b80` answers.
const CHARACTER_PROXY_OPERATOR_P: u32 = 0x004a_e750;
/// `00a29680` (unnamed in the engine map): a single `RET`, called on `this` before and after
/// the field is read (an empty lock and unlock in this build).
const LOCK_STAND_IN: u32 = 0x00a2_9680;
/// `00460140` (unnamed in the engine map): `thiscall`; `this + 0x10`.
const FIELD_AT_10: u32 = 0x0046_0140;

// Translated from 004ae700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `__fastcall`: answers null when `004ae750` finds no proxy for
/// `this`; otherwise runs `00a29680` on `this`, takes what `00460140` answers for the proxy
/// (the proxy's address plus `0x10`) and runs `00a29680` on `this` again.
pub fn fn_004ae700(e: &mut Engine, this: Ptr) -> u32 {
    let mut result = 0;
    let proxy = e.call(CHARACTER_PROXY_OPERATOR_P, &args![this]).u32();
    if proxy != 0 {
        e.call(LOCK_STAND_IN, &args![this]);
        result = e.call(FIELD_AT_10, &args![proxy]).u32();
        e.call(LOCK_STAND_IN, &args![this]);
    }
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004a_e700, fn_004ae700(Ptr) -> u32)]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn rig(proxy: u32) -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register_double(CHARACTER_PROXY_OPERATOR_P, move |_, _| proxy.into_ret());
        e.register(LOCK_STAND_IN, |_, _| Ret::default());
        e.register(FIELD_AT_10, |_, a| (a[0] + 0x10).into_ret());
        e
    }

    #[test]
    fn at_004ae700_answers_null_without_a_proxy() {
        let mut e = rig(0);
        assert_eq!(e.call(0x004a_e700, &args![0x4000u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .all(|c| c.0 != LOCK_STAND_IN && c.0 != FIELD_AT_10));
    }

    #[test]
    fn at_004ae700_reads_the_field_between_the_two_lock_calls() {
        let mut e = rig(0x9000);
        assert_eq!(e.call(0x004a_e700, &args![0x4000u32]).u32(), 0x9010);
        let order: Vec<(u32, Vec<u32>)> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != 0x004a_e700)
            .collect();
        assert_eq!(
            order,
            vec![
                (CHARACTER_PROXY_OPERATOR_P, vec![0x4000]),
                (LOCK_STAND_IN, vec![0x4000]),
                (FIELD_AT_10, vec![0x9000]),
                (LOCK_STAND_IN, vec![0x4000]),
            ]
        );
    }
}
