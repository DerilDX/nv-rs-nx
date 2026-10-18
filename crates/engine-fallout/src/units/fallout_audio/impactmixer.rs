//! `fallout/audio/impactmixer.cpp` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `0083b640` (`this` plus one stack word, `RET 4`).
const BASE_CONSTRUCTOR_0083B640: u32 = 0x0083_b640;
/// The vtable stored at `+0` (RTTI class `NiTPointerMap<int,unsigned int>`).
const VTABLE_0083B5E0: u32 = 0x0107_d9c4;

// Translated from 0083b5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `impactmixer.cpp`): runs `0083b640(this, argument)`, then
/// stores the vtable `0107d9c4` (RTTI class `NiTPointerMap<int,unsigned int>`) at `+0`.
/// Returns `this`.
pub fn fn_0083b5e0(e: &mut Engine, this: Ptr, argument: u32) -> Ptr {
    e.call(BASE_CONSTRUCTOR_0083B640, &args![this, argument]);
    e.mem.set_u32(this.addr(), VTABLE_0083B5E0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0083_b5e0, fn_0083b5e0(Ptr, u32) -> Ptr)]
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
        e.register(BASE_CONSTRUCTOR_0083B640, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_b5e0, &args![this, 0x1fu32]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_0083B640),
            vec![vec![this, 0x1f]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_0083B5E0);
    }
}
