//! `fallout shared/distant terrain system/bgsdistanttreeblock.cpp` (Xbox PDB source unit), subsystem `fallout shared/distant terrain system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006f99e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded map-item allocator: allocates twelve bytes without initializing them.
pub fn allocate_item(e: &mut Engine, _this: Ptr) -> Ptr {
    e.call(0x00401000, &args![12u32]).ptr()
}

/// The function `fn_006f9080` calls on the member (`0045cec0`).
const CALLEE_0045CEC0: u32 = 0x0045_cec0;

// Translated from 006f9080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0045cec0` with `ECX` = `this + 0x1c` and passes on nothing (the callee returns no value).
pub fn fn_006f9080(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0045CEC0, &args![this.byte_add(0x1c)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f99e0, allocate_item(Ptr) -> Ptr),
        entry!(0x006f_9080, fn_006f9080(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006f9080_passes_this_plus_1c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0045CEC0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_9080, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0045CEC0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x1c]);
    }
}
