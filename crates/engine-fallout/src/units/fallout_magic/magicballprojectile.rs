//! `fallout/magic/magicballprojectile.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both functions here are on the player's movement path: they reach the Havok character
//! proxy of a `bhkCharacterController` (`0080fb00` is what `MoveAsProjectile` runs first).

#[allow(unused_imports)]
use crate::prelude::*;

/// The accessor `004ae750` forwards to for a non-null object (`00620b80`, `this` only; it
/// is a thin entry for `0044ddc0`, listed under `cameracaster.cpp` in the engine map).
const PROXY_OF_OBJECT: u32 = 0x0062_0b80;
/// `hkpCharacterProxy` linear velocity setter (`00cac960`, `this` = the proxy, one argument:
/// the address of a four-float vector).
const PROXY_SET_LINEAR_VELOCITY: u32 = 0x00ca_c960;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004ae750, bhk_character_proxy_operator_p(Ptr) -> u32),
        entry!(0x0080fb00, fn_0080fb00(Ptr, Ptr)),
    ]
}

// Translated from 004ae750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::operatorP` (name from the engine map): null when `this` is null,
/// otherwise what `00620b80` answers for it.
pub fn bhk_character_proxy_operator_p(e: &mut Engine, this: Ptr) -> u32 {
    if this.is_null() {
        0
    } else {
        e.call(PROXY_OF_OBJECT, &args![this]).u32()
    }
}

// Translated from 0080fb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Looks up the proxy of `this` (`004ae750`) and, when there is
/// one, runs the proxy's linear velocity setter (`00cac960`) with `vector`.
pub fn fn_0080fb00(e: &mut Engine, this: Ptr, vector: Ptr) {
    let proxy = bhk_character_proxy_operator_p(e, this);
    if proxy != 0 {
        e.call(PROXY_SET_LINEAR_VELOCITY, &args![proxy, vector]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e.register(PROXY_OF_OBJECT, |_, a| (a[0] + 0x100).into_ret());
        e.register(PROXY_SET_LINEAR_VELOCITY, |_, _| Ret::default());
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn proxy_operator_is_null_for_null_and_forwards_otherwise() {
        let mut e = rig();
        assert_eq!(e.call(0x004a_e750, &args![0u32]).u32(), 0);
        assert!(calls(&e, PROXY_OF_OBJECT).is_empty());
        assert_eq!(e.call(0x004a_e750, &args![0x4000u32]).u32(), 0x4100);
        assert_eq!(calls(&e, PROXY_OF_OBJECT), vec![vec![0x4000]]);
    }

    #[test]
    fn set_velocity_runs_only_when_a_proxy_exists() {
        let mut e = rig();
        e.call(0x0080_fb00, &args![0u32, 0x9000u32]);
        assert!(calls(&e, PROXY_SET_LINEAR_VELOCITY).is_empty());
        e.call(0x0080_fb00, &args![0x4000u32, 0x9000u32]);
        assert_eq!(
            calls(&e, PROXY_SET_LINEAR_VELOCITY),
            vec![vec![0x4100, 0x9000]]
        );
    }
}
