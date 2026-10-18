//! `fallout/ai/run_008c51f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008c5e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Start values of the record `ActorPackage::Clear` (`008c5ef0`) resets: clears the words at
/// `+0`, `+4` and `+8`, sets the word at `+0xc` to `0xffffffff`, the float at `+0x10` to the
/// float at `0x01012054` and clears the word at `+0x14`. `ECX` is the object and the answer is
/// the object. The engine map gives it no name.
pub fn fn_008c5e20(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    let value = x87_quiet(e.global::<f32>(0x0101_2054));
    e.mem.set_f32(base + 0x10, value);
    e.mem.set_u32(base + 0xc, 0xffff_ffff);
    e.mem.set_u32(base + 0x14, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008c_5e20, fn_008c5e20(Ptr) -> Ptr)]
}

#[cfg(test)]
mod movement_path_tests {
    use crate::units::crt::ctor_test;

    #[test]
    fn fn_008c5e20_sets_the_start_values() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x008c_5e20,
            size: 0x18,
            globals: &[(0x0101_2054, 0x3f80_0000)],
            returns: &[],
            calls: &[],
            stores: &[
                (0x0, 4, 0),
                (0x4, 4, 0),
                (0x8, 4, 0),
                (0xc, 4, 0xffff_ffff),
                (0x10, 4, 0x3f80_0000),
                (0x14, 4, 0),
            ],
        });
    }
}
