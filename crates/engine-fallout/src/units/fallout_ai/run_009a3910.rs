//! `fallout/ai/run_009a3910` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009a3910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the next capacity for the word at `+0xc`:
/// doubled up to 0x400, plus 0x400 above it.
pub fn fn_009a3910(e: &mut Engine, this: Ptr) -> u32 {
    let size = e.mem.u32(this.addr() + 0xc);
    if size > 0x400 {
        size.wrapping_add(0x400)
    } else {
        size << 1
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009a_3910, fn_009a3910(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_then_steps_by_0x400() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for (size, next) in [(0u32, 0u32), (0x10, 0x20), (0x400, 0x800), (0x401, 0x801)] {
            e.mem.set_u32(this + 0xc, size);
            assert_eq!(e.call(0x009a_3910, &args![this]).u32(), next);
        }
    }
}
