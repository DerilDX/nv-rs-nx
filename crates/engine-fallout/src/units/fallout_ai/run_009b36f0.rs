//! `fallout/ai/run_009b36f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `009c52f0` (named `Projectile::LoadGame` in the engine map): the base body the wrapper
/// forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LOAD_GAME_TARGET_009C52F0: u32 = 0x009c_52f0;

// Translated from 009b4cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable words 0108c420, 0108eac0 and 0108f6d0): forwards `this`
/// and one pointer to `009c52f0` (named `Projectile::LoadGame`; Xbox PDB
/// `?LoadGame@Projectile@@UAAXPAVBGSLoadFormBuffer@@@Z`, void). Returns nothing.
pub fn fn_009b4cf0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(LOAD_GAME_TARGET_009C52F0, &args![this, buffer]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009b_4cf0, fn_009b4cf0(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_009b4cf0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LOAD_GAME_TARGET_009C52F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009b_4cf0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LOAD_GAME_TARGET_009C52F0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}
