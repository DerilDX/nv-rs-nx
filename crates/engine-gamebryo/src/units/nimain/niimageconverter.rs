//! `NiMain/niimageconverter.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{assign_ref_counted, interlocked_increment, ALLOCATE};

/// `011f4a7c`: the pointer `00a76ab0` stores the object it builds in.
const IMAGE_CONVERTER_SLOT: u32 = 0x011f_4a7c;
/// `011f4400`: a counted word `00a76ab0` increments for the object it builds.
const IMAGE_CONVERTER_COUNT: u32 = 0x011f_4400;
/// `00a8ae50` (`thiscall`, engine map unit `ninifimagereader.cpp`): constructs the part of
/// the object at `+0x80`.
const PART_CONSTRUCTOR: u32 = 0x00a8_ae50;

// Translated from 00a76ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `niimageconverter.cpp`; run by the start-up `00a89cc0`,
/// no arguments): allocates 0x780 bytes (`00aa13e0`), sets the vtable `0101dce4` and the
/// word at `+4` to 0, counts `011f4400`, sets the vtable `0109d9a4`, constructs the part
/// at `+0x80` with `00a8ae50` and stores the object, counted, in `011f4a7c`, releasing the
/// object held there before. A failed allocation stores null.
pub fn fn_00a76ab0(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x780u32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.mem.set_u32(block, 0x0101_dce4);
        e.mem.set_u32(block + 4, 0);
        interlocked_increment(e, IMAGE_CONVERTER_COUNT);
        e.mem.set_u32(block, 0x0109_d9a4);
        e.call(PART_CONSTRUCTOR, &args![block + 0x80]);
        block
    };
    assign_ref_counted(e, IMAGE_CONVERTER_SLOT, object);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a7_6ab0, fn_00a76ab0())]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn builds_the_object_counts_the_global_and_constructs_the_part() {
        let run = run_default_instance(0x00a7_6ab0, IMAGE_CONVERTER_SLOT, |e| {
            e.mem.set_u32(IMAGE_CONVERTER_COUNT, 7);
            e.register(PART_CONSTRUCTOR, |e, a| {
                e.mem.set_u32(a[0], 0x5a5a);
                Ret::default()
            });
        });
        let e = &run.engine;
        assert_eq!(e.mem.u32(run.object), 0x0109_d9a4);
        assert_eq!(e.mem.u32(run.object + 0x80), 0x5a5a);
        assert_eq!(e.mem.u32(IMAGE_CONVERTER_COUNT), 8);
        // The object's own count is at +4: stored 0, then counted once by the global.
        assert_eq!(e.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }

    #[test]
    fn a_failed_allocation_stores_null_and_counts_nothing() {
        let run = run_default_instance(0x00a7_6ab0, IMAGE_CONVERTER_SLOT, |e| {
            e.mem.set_u32(IMAGE_CONVERTER_COUNT, 7);
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
        });
        assert_eq!(run.object, 0);
        assert_eq!(run.engine.mem.u32(IMAGE_CONVERTER_COUNT), 7);
    }
}
