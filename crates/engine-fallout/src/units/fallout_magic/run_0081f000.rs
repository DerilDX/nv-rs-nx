//! `fallout/magic/run_0081f000` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c9c1d0` (engine map: `hkpEntity::activate`; `thiscall`, no stack words).
const ACTIVATE_00C9C1D0: u32 = 0x00c9_c1d0;
/// `009d9f40` (`thiscall`, no stack words): the address of the member at +0xe0.
const MEMBER_00E0: u32 = 0x009d_9f40;

// Translated from 0081f1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two stack words, `RET 8`): runs `00c9c1d0` on
/// `this`, then calls the vtable slot +0x50 of the object `009d9f40` answers (the member at
/// +0xe0) with `first` and `second`.
pub fn fn_0081f1e0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(ACTIVATE_00C9C1D0, &args![this]);
    let member = e.call(MEMBER_00E0, &args![this]).u32();
    e.vcall(member, 0x50, &args![first, second]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0081_f1e0, fn_0081f1e0(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLOT_50: u32 = 0x0e10_0000;

    #[test]
    fn fn_0081f1e0_activates_then_calls_the_member_slot() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x60);
        e.mem.set_u32(vtable + 0x50, SLOT_50);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0xe0, vtable);
        e.register(ACTIVATE_00C9C1D0, |_, _| Ret::default());
        e.register(MEMBER_00E0, |_, a| (a[0] + 0xe0).into_ret());
        e.register(SLOT_50, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0081_f1e0, &args![this, 5u32, 6u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (ACTIVATE_00C9C1D0, vec![this]),
                (MEMBER_00E0, vec![this]),
                (SLOT_50, vec![this + 0xe0, 5, 6]),
            ]
        );
    }
}
