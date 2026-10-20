//! `NiMain/run_00a81520` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{assign_ref_counted, ALLOCATE, OBJECT_BASE_CONSTRUCTOR};

/// `011f5ae8`: the pointer `00a81520` stores the object it builds in.
const SHADE_PROPERTY_SLOT: u32 = 0x011f_5ae8;

// Translated from 00a81520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiMain subsystem; run by
/// the start-up `00a89cc0`, no arguments): builds a 0x20-byte object (allocation
/// `00aa13e0`, base constructor `00a5c200`, vtable `0109e274`, the word at `+0x1c` set to
/// -1, the word at `+0x18` to 1) and stores it, counted, in `011f5ae8`, releasing the
/// object held there before. A failed allocation stores null.
pub fn fn_00a81520(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x20u32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, 0x0109_e274);
        e.mem.set_u32(block + 0x1c, 0xffff_ffff);
        e.mem.set_u16(block + 0x18, 1);
        block
    };
    assign_ref_counted(e, SHADE_PROPERTY_SLOT, object);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_1520, fn_00a81520())]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn builds_the_object_and_replaces_the_held_one() {
        let run = run_default_instance(0x00a8_1520, SHADE_PROPERTY_SLOT, |_| {});
        let e = &run.engine;
        assert_eq!(e.mem.u32(run.object), 0x0109_e274);
        assert_eq!(e.mem.u32(run.object + 0x1c), 0xffff_ffff);
        assert_eq!(e.mem.u16(run.object + 0x18), 1);
        assert_eq!(e.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let run = run_default_instance(0x00a8_1520, SHADE_PROPERTY_SLOT, |e| {
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
        });
        assert_eq!(run.object, 0);
    }
}
