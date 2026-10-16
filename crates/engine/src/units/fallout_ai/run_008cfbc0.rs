//! `fallout/ai/run_008cfbc0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d0270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of the object `00906dc0` builds (unnamed in the engine
/// map; the class is unconfirmed). `ECX` is the object. It stores the virtual
/// table `01085e2c`, runs `008c5e20` on the member at `+4`, clears the word at
/// `+0x2c` and returns `this`.
pub fn fn_008d0270(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0108_5e2c);
    e.call(0x008c_5e20, &args![base + 4]);
    e.mem.set_u32(base + 0x2c, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008d_0270, fn_008d0270(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_008d0270_builds_the_base() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x008d_0270,
            size: 0x30,
            globals: &[],
            returns: &[],
            calls: &[(0x008c_5e20, &[A::T(4)])],
            stores: &[(0x00, 4, 0x0108_5e2c), (0x2c, 4, 0)],
        });
    }
}
