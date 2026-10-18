//! `fallout/magic/absorbeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00804f80` (named `ActiveEffect::GetSaveSize` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const GET_SAVE_SIZE_TARGET_00804F80: u32 = 0x0080_4f80;

// Translated from 00802d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::GetSaveSize` (engine map vtable name; vtable words 010792bc and
/// others): forwards `this` and the `Actor` pointer to the base body `00804f80`
/// (`ActiveEffect::GetSaveSize`; Xbox PDB `?GetSaveSize@ActiveEffect@@UAAGPAVActor@@@Z`
/// returns `G`, an unsigned short). The callee ends with `MOV AX,[EBP-4]`; the wrapper returns
/// that 16-bit size.
pub fn value_modifier_effect_get_save_size(e: &mut Engine, this: Ptr, actor: Ptr) -> u16 {
    e.call(GET_SAVE_SIZE_TARGET_00804F80, &args![this, actor])
        .u16()
}

/// `008050d0` (named `ActiveEffect::SaveGame` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const SAVE_GAME_TARGET_008050D0: u32 = 0x0080_50d0;

// Translated from 00802db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::SaveGame` (engine map vtable name; vtable words 010792c4 and others):
/// forwards `this` and one pointer to the base body `008050d0` (`ActiveEffect::SaveGame`; Xbox
/// PDB declares overloads taking `Actor *` and `BGSSaveFormBuffer *`, both void). Returns
/// nothing.
pub fn value_modifier_effect_save_game(e: &mut Engine, this: Ptr, argument: Ptr) {
    e.call(SAVE_GAME_TARGET_008050D0, &args![this, argument]);
}

/// `00805330` (named `ActiveEffect::LoadGame` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LOAD_GAME_TARGET_00805330: u32 = 0x0080_5330;

// Translated from 00802dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::LoadGame` (engine map vtable name; vtable words 010792cc and others):
/// forwards `this` and one pointer to the base body `00805330` (`ActiveEffect::LoadGame`; Xbox
/// PDB declares overloads taking `Actor *` and `BGSLoadFormBuffer *`, both void). Returns
/// nothing.
pub fn value_modifier_effect_load_game(e: &mut Engine, this: Ptr, argument: Ptr) {
    e.call(LOAD_GAME_TARGET_00805330, &args![this, argument]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_2d90, value_modifier_effect_get_save_size(Ptr, Ptr) -> u16),
        entry!(0x0080_2db0, value_modifier_effect_save_game(Ptr, Ptr)),
        entry!(0x0080_2dd0, value_modifier_effect_load_game(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_value_modifier_effect_get_save_size_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(GET_SAVE_SIZE_TARGET_00804F80, |_, _| {
            0xdead_beefu32.into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0080_2d90, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == GET_SAVE_SIZE_TARGET_00804F80)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u16(), 0xbeef);
    }

    #[test]
    fn forwarder_value_modifier_effect_save_game_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(SAVE_GAME_TARGET_008050D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0080_2db0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == SAVE_GAME_TARGET_008050D0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_value_modifier_effect_load_game_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LOAD_GAME_TARGET_00805330, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0080_2dd0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LOAD_GAME_TARGET_00805330)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}
