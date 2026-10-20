//! `BSSystem/run_00afd920` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00afdbc0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): whether `address` lies in the
/// block that starts at the word +0xC and holds (word +0x8) 16-byte entries (unsigned
/// compares, the end `start + count * 16` wrapping as the exe's `SHL`/`ADD`).
pub fn fn_00afdbc0(e: &mut Engine, this: u32, address: u32) -> bool {
    let start = e.mem.u32(this + 0xc);
    if address < start {
        return false;
    }
    let end = (e.mem.u32(this + 0x8) << 4).wrapping_add(start);
    address < end
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00af_dbc0, fn_00afdbc0(u32, u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00afdbc0_tests_the_range() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x8, 3);
        e.mem.set_u32(this + 0xc, 0x1000);
        let inside = |e: &mut Engine, address: u32| e.call(0x00af_dbc0, &[this, address]).bool();
        assert!(!inside(&mut e, 0xfff));
        assert!(inside(&mut e, 0x1000));
        assert!(inside(&mut e, 0x102f));
        assert!(!inside(&mut e, 0x1030));
    }
}
