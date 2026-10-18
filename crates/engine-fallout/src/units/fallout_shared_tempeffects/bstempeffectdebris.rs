//! `fallout shared/tempeffects/bstempeffectdebris.cpp` (Xbox PDB source unit), subsystem `fallout shared/tempeffects`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006834a0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const MATCH_TARGET_006834A0: u32 = 0x0068_34a0;

// Translated from 00683480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (only caller `00682ff4`, in `~BSTempEffectDebris`): forwards
/// `this` and one word to `006834a0`, which returns whether `this` is non-null and the word at
/// `this+4` equals the argument (`SETZ DL`, result read from `AL`; 0 for a null `this`). The
/// wrapper returns that byte.
pub fn fn_00683480(e: &mut Engine, this: Ptr, value: u32) -> bool {
    e.call(MATCH_TARGET_006834A0, &args![this, value]).bool()
}

// Translated from 00682ac0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d6a90 without reading the receiver. The map names this body `BSTempEffectDebris::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00682ac0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_6a90
}

/// The base constructor `00683760` (`this` only, plain `RET`).
const BASE_CONSTRUCTOR_00683760: u32 = 0x0068_3760;
/// The vtable stored at `+0` (RTTI class `BSList<BSTempEffectDebris*>`).
const VTABLE_006836F0: u32 = 0x0106_9948;

// Translated from 006836f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bstempeffectdebris.cpp`): runs `00683760` on `this`, then
/// stores the vtable `01069948` (RTTI class `BSList<BSTempEffectDebris*>`) at `+0`. Returns
/// `this`.
pub fn fn_006836f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR_00683760, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_006836F0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0068_2ac0, fn_00682ac0(Ptr) -> u32),
        entry!(0x0068_3480, fn_00683480(Ptr, u32) -> bool),
        entry!(0x0068_36f0, fn_006836f0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00683480_forwards_this_and_the_argument_and_returns_the_byte() {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(MATCH_TARGET_006834A0, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0068_3480, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == MATCH_TARGET_006834A0)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00682ac0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0068_2ac0, &args![0x1000u32]).u32(), 0x011d_6a90);
        assert_eq!(e.call(0x0068_2ac0, &args![0u32]).u32(), 0x011d_6a90);
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn runs_the_base_constructor_then_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR_00683760, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_36f0, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_00683760),
            vec![vec![this]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_006836F0);
    }
}
