//! `fallout shared/pathfinding/run_00691c60` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0069a690` (`ECX` = the destination's +4, the source's +4; `RET 4`).
const COPY_TAIL: u32 = 0x0069_a690;
/// `0042f5d0` (cdecl: address, size).
const SIZED_0042F5D0: u32 = 0x0042_f5d0;

// Translated from 00691c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy-constructor shape (one word, the source; `RET 4`): the first word
/// is copied, then `0069a690` on `this + 4` with `source + 4`. Returns
/// `this`. The engine map gives it no name.
pub fn fn_00691c60(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let word = e.mem.u32(source.addr());
    e.mem.set_u32(this.addr(), word);
    e.call(COPY_TAIL, &args![this.addr() + 4, source.addr() + 4]);
    this
}

// Translated from 00691d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0042f5d0` with an address and 12 times a count (`thiscall`, `ECX` not
/// read; `RET 8`). The engine map gives it no name.
pub fn fn_00691d20(e: &mut Engine, _unused_0: Ptr, address: u32, count: u32) {
    e.call(SIZED_0042F5D0, &args![address, count.wrapping_mul(0xc)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0069_1c60, fn_00691c60(Ptr, Ptr) -> Ptr),
        entry!(0x0069_1d20, fn_00691d20(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log_of(e: &mut Engine, addr: u32, words: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log = Some(vec![]);
        e.call(addr, words);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect()
    }

    #[test]
    fn at_00691c60_copies_the_head_and_the_tail() {
        let mut e = Engine::new();
        e.register(COPY_TAIL, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0x1234);
        assert_eq!(
            log_of(&mut e, 0x0069_1c60, &[this, source]),
            vec![(COPY_TAIL, vec![this + 4, source + 4])]
        );
        assert_eq!(e.mem.u32(this), 0x1234);
        assert_eq!(e.call(0x0069_1c60, &args![this, source]).u32(), this);
    }

    #[test]
    fn at_00691d20_passes_twelve_bytes_per_element() {
        let mut e = Engine::new();
        e.register(SIZED_0042F5D0, |_, _| Ret::default());
        assert_eq!(
            log_of(&mut e, 0x0069_1d20, &[0, 0x5000, 3]),
            vec![(SIZED_0042F5D0, vec![0x5000, 0x24])]
        );
    }
}
