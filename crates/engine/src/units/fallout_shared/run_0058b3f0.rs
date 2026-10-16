//! `fallout shared/run_0058b3f0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0058b450 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00464f40` of unit tesdatahandler, and returns
/// nothing. Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. `00464f40`'s translated signature `fn_00464f40(Ptr)`
/// returns nothing.
pub fn fn_0058b450(e: &mut Engine, this: Ptr) {
    e.call(0x0046_4f40, &args![this]);
}

// Translated from 0058b420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0058b450` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0058b420(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0058_b450, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_b450, fn_0058b450(Ptr)),
        entry!(0x0058_b420, fn_0058b420(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_0058b450_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0046_4f40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0058_b450, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0046_4f40), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0058b420_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_b450, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_b420, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_b420)
                .collect();
            let mut expected = vec![(0x0058_b450, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
