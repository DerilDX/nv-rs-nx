//! `(unplaced)/run_0040e600` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0040e210` (`spellitem.cpp`; thiscall, one word).
const CALLEE_0040E210: u32 = 0x0040_e210;

// Translated from 0040e650 (decompiled, FalloutNV.exe 1.4.0.525)
/// A base-adjusting forwarder (unnamed in the engine map): calls
/// `0040e210(this - 0x18, 0x20)`, the object 0x18 bytes before `this`.
pub fn fn_0040e650(e: &mut Engine, this: Ptr) {
    let object = this.addr().wrapping_sub(0x18);
    e.call(CALLEE_0040E210, &args![object, 0x20u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0040_e650, fn_0040e650(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwards_the_adjusted_object_and_0x20() {
        let mut e = Engine::new();
        e.register(CALLEE_0040E210, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0040_e650, &args![0x0500_0018u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == CALLEE_0040E210).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x0500_0000, 0x20]);
    }
}
