//! `fallout/interface/run_007021b0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator delete` (`00401030`, `cdecl`: the block).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 00702220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the block the word at `+0` points to (unnamed in the map; `ECX`,
/// no stack words): `operator delete` (`00401030`) on it, the pointer
/// itself left as it is. The change record's buffer teardown (`008456c0`)
/// calls it on the record's `+4`.
pub fn fn_00702220(e: &mut Engine, this: Ptr) {
    let block = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![block]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00702220, fn_00702220(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deletes_the_block_at_plus_0() {
        let mut e = Engine::new();
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, 0x0b00_0000);
        e.call_log = Some(vec![]);
        e.call(0x0070_2220, &args![Ptr::<()>::new(holder)]);
        let log = e.call_log.unwrap();
        let deletes: Vec<_> = log.iter().filter(|c| c.0 == OPERATOR_DELETE).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![0x0b00_0000]);
        assert_eq!(e.mem.u32(holder), 0x0b00_0000);
    }
}
