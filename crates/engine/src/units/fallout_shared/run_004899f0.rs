//! `fallout shared/run_004899f0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The virtual table the constructor stores last (`0101d0d4`), over the one its
/// base constructor stored.
const VTABLE: u32 = 0x0101_d0d4;

// Translated from 00489a60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00489a60 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the component at `+0x114` of `TESCreature` (`005f7230` calls it
/// there). The engine map gives it no name, so the class is unconfirmed. `ECX` is
/// the object. It runs the base constructor `00402d30` (which stores the
/// virtual table `01011d68`), stores the virtual table `0101d0d4`, constructs the
/// two pairs of words at `+4` and `+0xc` (`0096a2d0`: both cleared) and runs the
/// empty `00483710`. Returns `this`. The exception-unwinding frame is not
/// translated.
pub fn fn_00489a60(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(base, VTABLE);
    e.call(0x0096_a2d0, &args![base + 4]);
    e.call(0x0096_a2d0, &args![base + 0xc]);
    e.call(0x0048_3710, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00489a60, fn_00489a60(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00489a60_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0048_9a60,
            size: 0x14,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_2d30, &[A::T(0x0)]),
                (0x0096_a2d0, &[A::T(0x4)]),
                (0x0096_a2d0, &[A::T(0xc)]),
                (0x0048_3710, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0101_d0d4)],
        });
    }
}
