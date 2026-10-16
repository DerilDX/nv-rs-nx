//! `fallout shared/sky/stars.cpp` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0063f940` (named `SkyObject::Initialize` in the engine map): the base body the wrapper
/// forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const INITIALIZE_TARGET_0063F940: u32 = 0x0063_f940;

// Translated from 0063fd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Stars::Initialize` (Xbox PDB `?Initialize@Stars@@UAAXPAVNiNode@@@Z`, void, vtable word
/// 0104f1e0): forwards `this` and the `NiNode` pointer to the base body `0063f940`
/// (`SkyObject::Initialize`). Returns nothing: the decorated name is void; the callee ends
/// with an exception-frame unlink after a virtual call on its argument.
pub fn stars_initialize(e: &mut Engine, this: Ptr, root: Ptr) {
    e.call(INITIALIZE_TARGET_0063F940, &args![this, root]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0063_fd90, stars_initialize(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_stars_initialize_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(INITIALIZE_TARGET_0063F940, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0063_fd90, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == INITIALIZE_TARGET_0063F940)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}
