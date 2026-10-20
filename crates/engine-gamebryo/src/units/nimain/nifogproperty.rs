//! `NiMain/nifogproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{assign_ref_counted, ALLOCATE, OBJECT_BASE_CONSTRUCTOR};

/// `011f5f3c`: the pointer `00a9a090` stores the object it builds in.
const FOG_PROPERTY_SLOT: u32 = 0x011f_5f3c;
/// Three words at `011f4998` that the object takes at `+0x20`.
const FOG_COLOR_SOURCE: u32 = 0x011f_4998;

// Translated from 00a9a090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nifogproperty.cpp`; run by the start-up `00a89cc0`, no
/// arguments): builds a 0x2c-byte object (allocation `00aa13e0`, base constructor
/// `00a5c200`, vtable `010a0e0c`, the word at `+0x18` set to 0, the float at `+0x1c` to
/// 1.0, the three floats at `+0x20` first to 0.0 and then replaced by the three words at
/// `011f4998`) and stores it, counted, in `011f5f3c`, releasing the object held there
/// before. A failed allocation stores null.
pub fn fn_00a9a090(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, 0x010a_0e0c);
        // `FLDZ; FST; FST; FSTP`: 0.0 into the three words at +0x20.
        e.mem.set_u32(block + 0x20, 0.0f32.to_bits());
        e.mem.set_u32(block + 0x24, 0.0f32.to_bits());
        e.mem.set_u32(block + 0x28, 0.0f32.to_bits());
        e.mem.set_u16(block + 0x18, 0);
        e.mem.set_u32(block + 0x1c, 1.0f32.to_bits());
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(FOG_COLOR_SOURCE + offset);
            e.mem.set_u32(block + 0x20 + offset, word);
        }
        block
    };
    assign_ref_counted(e, FOG_PROPERTY_SLOT, object);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_a090, fn_00a9a090())]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn builds_the_object_with_the_three_source_words() {
        let run = run_default_instance(0x00a9_a090, FOG_PROPERTY_SLOT, |e| {
            e.mem.set_u32(FOG_COLOR_SOURCE, 0x1111_1111);
            e.mem.set_u32(FOG_COLOR_SOURCE + 4, 0x2222_2222);
            e.mem.set_u32(FOG_COLOR_SOURCE + 8, 0x3333_3333);
        });
        let e = &run.engine;
        assert_eq!(e.mem.u32(run.object), 0x010a_0e0c);
        assert_eq!(e.mem.u16(run.object + 0x18), 0);
        assert_eq!(e.mem.u32(run.object + 0x1c), 0x3f80_0000);
        assert_eq!(e.mem.u32(run.object + 0x20), 0x1111_1111);
        assert_eq!(e.mem.u32(run.object + 0x24), 0x2222_2222);
        assert_eq!(e.mem.u32(run.object + 0x28), 0x3333_3333);
        assert_eq!(e.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let run = run_default_instance(0x00a9_a090, FOG_PROPERTY_SLOT, |e| {
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
        });
        assert_eq!(run.object, 0);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }
}
