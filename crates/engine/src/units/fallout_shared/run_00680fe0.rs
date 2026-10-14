//! `fallout shared/run_00680fe0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00407a90` (cdecl: address, a byte passed as a word): swaps the bytes of
/// the 16-bit word at the address.
const SWAP_HALF: u32 = 0x0040_7a90;
/// `00401080` (cdecl: address, a byte passed as a word): swaps the bytes of
/// the word at the address.
const SWAP_WORD: u32 = 0x0040_1080;

// Translated from 00681110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the byte order of a condition item's function data (`ECX`, no
/// stack words; `Data.FunctionData`, Xbox PDB): the 16-bit function number
/// at +0 (`00407a90`), then the parameter words at +4 and +8 (`00401080`),
/// each with a zero byte. The engine map gives it no name.
pub fn fn_00681110(e: &mut Engine, this: Ptr) {
    e.call(SWAP_HALF, &args![this, 0u32]);
    e.call(SWAP_WORD, &args![this.addr() + 4, 0u32]);
    e.call(SWAP_WORD, &args![this.addr() + 8, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0068_1110, fn_00681110(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swaps_the_number_and_both_parameters() {
        let mut e = Engine::new();
        e.register(SWAP_HALF, |_, _| Ret::default());
        e.register(SWAP_WORD, |_, _| Ret::default());
        let data: Ptr = Ptr::new(e.mem.alloc(12));
        e.call_log = Some(vec![]);
        e.call(0x0068_1110, &args![data]);
        let log = e.call_log.take().unwrap();
        let swaps: Vec<_> = log
            .iter()
            .filter(|c| c.0 == SWAP_HALF || c.0 == SWAP_WORD)
            .cloned()
            .collect();
        assert_eq!(
            swaps,
            [
                (SWAP_HALF, vec![data.addr(), 0]),
                (SWAP_WORD, vec![data.addr() + 4, 0]),
                (SWAP_WORD, vec![data.addr() + 8, 0]),
            ]
        );
    }
}
