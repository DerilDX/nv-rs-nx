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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006ecd40, fn_006ecd40(Ptr, u32))]
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
}
