//! `fallout/ai/combat/combatprocedureattackmelee.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006ecd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at `+4` of `this` (`thiscall`, one word). The
/// engine map's name `CombatProcedureAttackMelee::Initialize` belongs to one
/// of the methods folded onto this shared body; `TESQuest::Load` (`0060d104`)
/// uses it on its component at `+0x18` to store the word read from a chunk.
pub fn fn_006ecd40(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(4), value);
}

/// `006ecd40` (named `CombatProcedureAttackMelee::Initialize` in the engine map): the base
/// body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const STORE_TARGET_006ECD40: u32 = 0x006e_cd40;

// Translated from 009cc4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable word 01090c0c of `CombatProcedureAttackMelee`, between
/// `Update` and `Shutdown`): forwards `this` and one word to `006ecd40`, which stores the word
/// at `this+4`; that body's translation (`units/fallout_ai/combatprocedureattackmelee.rs`)
/// returns nothing, and so does this wrapper.
pub fn fn_009cc4b0(e: &mut Engine, this: Ptr, value: u32) {
    e.call(STORE_TARGET_006ECD40, &args![this, value]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006ecd40, fn_006ecd40(Ptr, u32)),
        entry!(0x009c_c4b0, fn_009cc4b0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_word_at_4() {
        let mut e = Engine::new();
        let block = e.mem.alloc(12);
        e.mem.write(block, &[0xa5; 12]);
        e.call(0x006e_cd40, &args![block, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(block + 4), 0x1234_5678);
        assert_eq!(e.mem.u32(block), 0xa5a5_a5a5);
        assert_eq!(e.mem.u32(block + 8), 0xa5a5_a5a5);
    }

    #[test]
    fn forwarder_fn_009cc4b0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(STORE_TARGET_006ECD40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009c_c4b0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == STORE_TARGET_006ECD40)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}
