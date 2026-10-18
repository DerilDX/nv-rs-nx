//! `fallout shared/run_0067fcb0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0067fcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A small constructor, `__thiscall`, that returns `this`: it sets the bytes
/// at `+0`, `+1`, `+2` and `+3` to `0xff`, `0xff`, `0x00` and `0xff` and the
/// word at `+4` to zero. `TESPackage`'s constructor (`006707c0`) runs it on the
/// member at `+0x38`. It is not itself a form constructor, and the engine map
/// gives it no name, so the type of the member is unconfirmed.
pub fn fn_0067fcb0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u8(base, 0xff);
    e.mem.set_u8(base + 1, 0xff);
    e.mem.set_u8(base + 2, 0x00);
    e.mem.set_u8(base + 3, 0xff);
    e.mem.set_u32(base + 4, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0067_fcb0, fn_0067fcb0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self};

    #[test]
    fn fn_0067fcb0_sets_the_start_values() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0067_fcb0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[],
            stores: &[
                (0x0, 1, 0xff),
                (0x1, 1, 0xff),
                (0x2, 1, 0x0),
                (0x3, 1, 0xff),
                (0x4, 4, 0x0),
            ],
        });
    }
}
