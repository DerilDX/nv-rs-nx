//! `fallout shared/run_0044d8c0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0044ddc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `+8` of `this` (a plain field getter; `0040fae0` follows it from an element to the next one).
pub fn fn_0044ddc0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 8)
}

/// The function `fn_0044dd40` calls on the member (`0044cbf0`).
const CALLEE_0044CBF0: u32 = 0x0044_cbf0;

// Translated from 0044dd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0044cbf0` with `ECX` = `this + 0x4` and passes on nothing (the callee returns no value).
pub fn fn_0044dd40(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0044CBF0, &args![this.byte_add(0x4)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044ddc0, fn_0044ddc0(Ptr) -> u32),
        entry!(0x0044_dd40, fn_0044dd40(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_field_at_offset_8() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x50));
        e.mem.set_u32(object.addr() + 8, 0x1234_5678);
        assert_eq!(e.call(0x0044_ddc0, &args![object]).u32(), 0x1234_5678);
    }

    #[test]
    fn l68d_0044dd40_passes_this_plus_4_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0044CBF0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0044_dd40, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0044CBF0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x4]);
    }
}
