//! `fallout shared/magic/effectitemlist.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00402c90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00402c90 - the exception-unwinding frame (FS:[0]) is not translated.
/// `EffectItemList::EffectItemList` (the Xbox PDB name the engine map gives
/// it). Runs the constructor `00402d00` on the object (it sets a base vtable
/// and constructs a string at `+4`, see `run_00402d00.rs`), `004056a0` on
/// the member at `+0xc`, then stores the vtable pointers `01011d0c` at `+0`
/// and `01011cf4` at `+0xc` (the RTTI name of both is `.?AVMagicItem@@`).
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn effect_item_list_effect_item_list(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d00, &args![this]);
    e.call(0x0040_56a0, &args![this.byte_add(0xc)]);
    e.mem.set_u32(this.addr(), 0x0101_1d0c);
    e.mem.set_u32(this.addr() + 0xc, 0x0101_1cf4);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00402c90, effect_item_list_effect_item_list(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn effect_item_list_effect_item_list_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_2c90,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d00, &[A::T(0x0)]), (0x0040_56a0, &[A::T(0xc)])],
            stores: &[(0x0, 4, 0x0101_1d0c), (0xc, 4, 0x0101_1cf4)],
        });
    }
}
