//! `Havok SDK/hkclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d472a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A Havok class record's constructor (`hkClass`, Havok SDK; `thiscall`,
/// `RET 0x34`: thirteen words). Stores the first three words at +0, +4 and
/// +8, skips the fourth, and stores the other nine at +0xC to +0x2C, in
/// order. Returns nothing (`EAX` is `this`, unused by the callers).
pub fn fn_00d472a0(e: &mut Engine, this: Ptr, words: &[u32]) {
    let t = this.addr();
    for (i, &w) in words[..3].iter().enumerate() {
        e.mem.set_u32(t + 4 * i as u32, w);
    }
    for (i, &w) in words[4..13].iter().enumerate() {
        e.mem.set_u32(t + 0xc + 4 * i as u32, w);
    }
}

fn fn_00d472a0_entry(e: &mut Engine, a: &[u32]) -> Ret {
    fn_00d472a0(e, Ptr::new(a[0]), &a[1..14]);
    a[0].into_ret()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![(0x00d472a0, fn_00d472a0_entry as AbiFn)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_twelve_of_the_thirteen_words() {
        let mut e = Engine::new();
        let c = e.mem.alloc(0x30);
        let mut words = vec![c];
        words.extend(1..=13u32);
        e.call(0x00d472a0, &words);
        let got: Vec<u32> = (0..12).map(|i| e.mem.u32(c + 4 * i)).collect();
        assert_eq!(got, [1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    }
}
