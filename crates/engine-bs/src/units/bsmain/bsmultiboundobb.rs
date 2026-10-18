//! `BSMain/bsmultiboundobb.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c38000` (named `BSMultiBoundSphere::LinkObject` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C38000: u32 = 0x00c3_8000;

// Translated from 00c36430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::LinkObject` (engine map vtable name, vtable word 0101e9f4): forwards
/// `this` and the stream to `00c38000`, the `LinkObject` body of `BSMultiBoundAABB`'s table
/// (the engine map names that address `BSMultiBoundSphere::LinkObject`). Returns nothing: Xbox
/// PDB `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream`
/// reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn bs_multi_bound_obb_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C38000, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c3_6430,
        bs_multi_bound_obb_link_object(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_bs_multi_bound_obb_link_object_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C38000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c3_6430, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C38000)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}
