//! `fallout/audio/run_008374e0` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The word table `008374e0` stores into (`011dd87c`, indexed by its first
/// argument).
const TABLE_011DD87C: u32 = 0x011d_d87c;
/// The byte `00837500` clears (`011dd85c`).
const FLAG_011DD85C: u32 = 0x011d_d85c;
/// `ImpactMixer::InitFootsteps` (engine map; `00839ab0`, no words).
const IMPACT_MIXER_INIT_FOOTSTEPS: u32 = 0x0083_9ab0;

// Translated from 008374e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words): stores `value` at
/// `011dd87c + 4 * index` (no bounds check, as in the exe).
pub fn fn_008374e0(e: &mut Engine, index: u32, value: u32) {
    let slot = TABLE_011DD87C.wrapping_add(index.wrapping_mul(4));
    e.mem.set_u32(slot, value);
}

// Translated from 00837500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no words): clears the byte at
/// `011dd85c`, then `ImpactMixer::InitFootsteps`.
pub fn fn_00837500(e: &mut Engine) {
    e.mem.set_u8(FLAG_011DD85C, 0);
    e.call(IMPACT_MIXER_INIT_FOOTSTEPS, &[]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0083_74e0, fn_008374e0(u32, u32)),
        entry!(0x0083_7500, fn_00837500()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_008374e0_stores_into_the_table() {
        let mut e = Engine::new();
        e.map(TABLE_011DD87C, 0x40);
        e.call(0x0083_74e0, &args![3u32, 0x1234u32]);
        assert_eq!(e.mem.u32(TABLE_011DD87C + 12), 0x1234);
        assert_eq!(e.mem.u32(TABLE_011DD87C + 8), 0);
    }

    #[test]
    fn fn_00837500_clears_the_flag_and_inits_footsteps() {
        let mut e = Engine::new();
        e.map(FLAG_011DD85C, 4);
        e.mem.set_u8(FLAG_011DD85C, 1);
        e.mem.set_u8(FLAG_011DD85C + 1, 9);
        e.register(IMPACT_MIXER_INIT_FOOTSTEPS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0083_7500, &args![]);
        assert_eq!(e.mem.u8(FLAG_011DD85C), 0);
        assert_eq!(e.mem.u8(FLAG_011DD85C + 1), 9);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(IMPACT_MIXER_INIT_FOOTSTEPS, vec![])));
    }
}
