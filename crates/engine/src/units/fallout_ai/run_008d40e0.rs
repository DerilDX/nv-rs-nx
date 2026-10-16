//! `fallout/ai/run_008d40e0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d4380 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `008d50a0`, which frees the block held at `+0`
/// through `00401030` when it is non-null and zeroes `+0`, and returns nothing. Parameters:
/// one word, `this`; the disassembly of the function ends in a plain `RET` and passes only
/// `ECX` on. `008d50a0`'s disassembly ends on the store `MOV dword ptr [ECX],0x0` with no
/// result (Ghidra types it void).
pub fn fn_008d4380(e: &mut Engine, this: Ptr) {
    e.call(0x008d_50a0, &args![this]);
}

// Translated from 008d4360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor that runs `008d5020` on the object (`ECX`) and answers it. The engine map gives it
/// no name.
pub fn fn_008d4360(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x008d_5020, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_4380, fn_008d4380(Ptr)),
        entry!(0x008d_4360, fn_008d4360(Ptr) -> Ptr),
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
    fn fwd_a03_008d4380_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x008d_50a0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008d_4380, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x008d_50a0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_008d4360_runs_the_base_initialiser() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x008d_4360,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x008d_5020, &[A::T(0)])],
            stores: &[],
        });
    }
}
