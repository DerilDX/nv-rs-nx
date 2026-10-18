//! `fallout shared/run_005fc220` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005fc680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fc680 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 8 (0x4c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005fc680(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_ba00, &args![base + 0x24]);
    e.mem.set_u32(base, 0x0104_98dc);
    e.mem.set_u32(base + 0x18, 0x0104_98c8);
    e.mem.set_u32(base + 0x24, 0x0104_98b4);
    e.call(0x0096_a2d0, &args![base + 0x3c]);
    e.call(0x005f_c800, &args![this]);
    e.call(0x004f_15a0, &args![this, 0x8]);
    this
}

// Translated from 005fc840 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `005fc860`, which walks a list and deletes its
/// items, and returns nothing. Parameters: one word, `this`; the disassembly of the function
/// ends in a plain `RET` and passes only `ECX` on. `005fc860`'s disassembly is a loop that
/// ends on its exit test without loading a result (Ghidra types it void).
pub fn fn_005fc840(e: &mut Engine, this: Ptr) {
    e.call(0x005f_c860, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_c840, fn_005fc840(Ptr)),
        entry!(0x005f_c680, fn_005fc680(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005fc680_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_c680,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_ba00, &[A::T(0x24)]),
                (0x0096_a2d0, &[A::T(0x3c)]),
                (0x005f_c800, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x8)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_98dc),
                (0x18, 4, 0x0104_98c8),
                (0x24, 4, 0x0104_98b4),
            ],
        });
    }
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
    fn fwd_a03_005fc840_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x005f_c860, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005f_c840, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x005f_c860), vec![vec![0x5000]]);
    }
}
