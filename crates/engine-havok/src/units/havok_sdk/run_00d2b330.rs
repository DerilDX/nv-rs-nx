//! `Havok SDK/run_00d2b330` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d2b330 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, two words `source` and `buffer`; descriptions, not confirmed): calls
/// `00d61c10` on `buffer` with `source` (thiscall), then `00d61840` on `buffer` with
/// `(0x2f, 0x5c, 1)` (`'/'`, `'\\'`: the exe passes these two characters and a flag), and answers
/// the word at `buffer + 0` (the string pointer the buffer holds).
pub fn fn_00d2b330(e: &mut Engine, source: u32, buffer: Ptr) -> u32 {
    e.call(0x00d6_1c10, &args![buffer, source]);
    e.call(0x00d6_1840, &args![buffer, 0x2fu32, 0x5cu32, 1u32]);
    e.mem.u32(buffer.addr())
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d2_b330, fn_00d2b330(u32, Ptr) -> u32)]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d2b330_sets_the_buffer_and_swaps_the_separators() {
        let mut e = Engine::new();
        // The first double gives the buffer its string pointer.
        e.register(0x00d6_1c10, |e, a| {
            e.mem.set_u32(a[0], 0x0123_4560 + a[1]);
            Ret::default()
        });
        e.register(0x00d6_1840, |_, _| Ret::default());
        let buffer = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let answer = e.call(0x00d2_b330, &args![7u32, buffer]).u32();
        assert_eq!(answer, 0x0123_4567);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x00d6_1c10, vec![buffer, 7]),
                (0x00d6_1840, vec![buffer, 0x2f, 0x5c, 1])
            ]
        );
    }
}
