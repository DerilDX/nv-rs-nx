//! `(unplaced)/run_006cf8a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006bc870`: the base part's assignment (`thiscall`, one word).
const BASE_ASSIGN: u32 = 0x006b_c870;

// Translated from 006cf900 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: assignment from `other`: the base part (`006bc870`), then
/// the three words at `+0xc`. Returns `this`.
pub fn fn_006cf900(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(BASE_ASSIGN, &args![this, other]);
    for i in 0..3 {
        let word = e.mem.u32(other.addr() + 0xc + 4 * i);
        e.mem.set_u32(this.addr() + 0xc + 4 * i, word);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006c_f900, fn_006cf900(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assigns_base_then_point() {
        let mut e = Engine::new();
        e.register(BASE_ASSIGN, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        let other = e.mem.alloc(0x18);
        for i in 0..6 {
            e.mem.set_u32(other + 4 * i, 0x100 + i);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c_f900, &args![this, other]).u32(), this);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x103);
        assert_eq!(e.mem.u32(this + 0x14), 0x105);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_ASSIGN, vec![this, other])));
    }
}
