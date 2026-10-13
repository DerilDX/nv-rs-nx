//! `fallout/ai/combat/combattargetmap.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00853130(this, key, &out)` (unit `bgssaveloadreferencesmap`): the map
/// lookup; returns success in `AL` and writes the value to `out` only on
/// success.
const MAP_LOOKUP: u32 = 0x0085_3130;

// Translated from 009a4250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTargetMap::GetTargetAttackerArray` (engine map name): looks `key`
/// up in the map with `00853130` and returns the value found, or 0 when the
/// lookup fails.
pub fn combat_target_map_get_target_attacker_array(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    e.with_stack(4, |e, found| {
        e.mem.set_u32(found.addr(), 0);
        if e.call(MAP_LOOKUP, &args![this, key, found]).bool() {
            e.mem.u32(found.addr())
        } else {
            0
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x009a4250,
        combat_target_map_get_target_attacker_array(Ptr, u32) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hit_returns_the_value_a_miss_returns_zero() {
        let mut e = Engine::new();
        e.register(MAP_LOOKUP, |e, a| {
            if a[1] == 7 {
                e.mem.set_u32(a[2], 0x1234);
                Ret {
                    eax: 1,
                    ..Ret::default()
                }
            } else {
                // A failed lookup may leave a stale low byte in EAX's upper
                // bits; only AL counts.
                Ret {
                    eax: 0x100,
                    ..Ret::default()
                }
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009a4250, &args![0x5000u32, 7u32]).u32(), 0x1234);
        assert_eq!(e.call(0x009a4250, &args![0x5000u32, 8u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            (log[1].0, log[1].1[..2].to_vec()),
            (MAP_LOOKUP, vec![0x5000, 7])
        );
    }
}
