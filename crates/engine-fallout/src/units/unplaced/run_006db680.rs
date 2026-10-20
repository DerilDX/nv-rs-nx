//! `(unplaced)/run_006db680` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable `006db720` sets (RTTI `.?AV?$BSSimpleArray@M$0EAA@@@`, `BSSimpleArray<float,1024>`).
const SIMPLE_ARRAY_OF_FLOATS_VTABLE: u32 = 0x0106_cc64;
/// `008454f0` (`thiscall`, `RET 4`, no unit in the engine map): run with 1.
const ARRAY_008454F0: u32 = 0x0084_54f0;

// Translated from 006db720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its vtable the `BSSimpleArray<float,1024>` destructor body
/// (`ECX` only): sets the vtable `0106cc64` and runs `008454f0` on `this` with 1.
pub fn fn_006db720(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SIMPLE_ARRAY_OF_FLOATS_VTABLE);
    e.call(ARRAY_008454F0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006d_b720, fn_006db720(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_vtable_and_releases_with_one() {
        let mut e = Engine::new();
        e.register(ARRAY_008454F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x10);
        e.call(0x006d_b720, &args![this]);
        assert_eq!(e.mem.u32(this), SIMPLE_ARRAY_OF_FLOATS_VTABLE);
        let calls: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ARRAY_008454F0)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(calls, vec![vec![this, 1]]);
    }
}
