//! `fallout/magic/magicshaderhiteffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006b3eb0(this, 0, 0)`: the array-shaped constructor (unit
/// `pathsmootherpov`).
const ARRAY_CONSTRUCTOR: u32 = 0x006b_3eb0;
/// The vtable `00822600` stores.
const VTABLE: u32 = 0x0107_b84c;

// Translated from 00822600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `0107b84c` and runs
/// `006b3eb0(this, 0, 0)` (an empty array: no reserved size, no elements).
/// Returns `this`. The class is not confirmed from the exe.
pub fn fn_00822600(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(ARRAY_CONSTRUCTOR, &args![this, 0u32, 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00822600, fn_00822600(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_vtable_then_builds_an_empty_array() {
        let mut e = Engine::new();
        e.register(ARRAY_CONSTRUCTOR, |e, a| {
            // The vtable is already in place when the constructor runs.
            assert_eq!(e.mem.u32(a[0]), VTABLE);
            Ret::default()
        });
        let this = e.mem.alloc(16);
        e.call_log = Some(vec![]);
        let r = e.call(0x00822600, &args![this]);
        assert_eq!(r.u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ARRAY_CONSTRUCTOR, vec![this, 0, 0]));
    }
}
