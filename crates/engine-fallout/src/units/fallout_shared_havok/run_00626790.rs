//! `fallout shared/havok/run_00626790` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00626790 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__cdecl(unused, count)`: a loop over `0..count` with an
/// empty body; it does nothing.
pub fn fn_00626790(_e: &mut Engine, _unused_0: u32, _count: i32) {}

// Translated from 006267c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__cdecl(destination, source)`: copies two words from
/// `source` to `destination`.
pub fn fn_006267c0(e: &mut Engine, destination: u32, source: u32) {
    for i in 0..2 {
        let word = e.mem.u32(source + i * 4);
        e.mem.set_u32(destination + i * 4, word);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00626790, fn_00626790(u32, i32)),
        entry!(0x006267c0, fn_006267c0(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00626790_has_no_effect() {
        let mut e = Engine::new();
        let block = e.mem.alloc(4);
        e.mem.set_u32(block, 5);
        e.call(0x0062_6790, &args![block, 10i32]);
        assert_eq!(e.mem.u32(block), 5);
    }

    #[test]
    fn fn_006267c0_copies_two_words() {
        let mut e = Engine::new();
        let source = e.mem.alloc(12);
        let destination = e.mem.alloc(12);
        for (i, word) in [1u32, 2, 3].iter().enumerate() {
            e.mem.set_u32(source + 4 * i as u32, *word);
        }
        e.call(0x0062_67c0, &args![destination, source]);
        assert_eq!(e.mem.u32(destination), 1);
        assert_eq!(e.mem.u32(destination + 4), 2);
        assert_eq!(e.mem.u32(destination + 8), 0);
    }
}
