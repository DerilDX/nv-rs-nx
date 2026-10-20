//! `NiMain/run_00a8fd80` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_13e0;
/// `00a8fce0` (`thiscall`, engine map `NiSingleShaderMaterialLibrary::NiSingleShaderMaterialLibrary`):
/// constructs the object in the block and answers it.
const MATERIAL_LIBRARY_CONSTRUCTOR: u32 = 0x00a8_fce0;
/// `011f5e40`: where `00a8fd80` keeps the object.
const MATERIAL_LIBRARY_SLOT: u32 = 0x011f_5e40;

// Translated from 00a8fd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiMain subsystem; run by the start-up
/// `00a89cc0`, no arguments): allocates 0x14 bytes (`00aa13e0`), constructs the object
/// with `00a8fce0` and stores what it answers in `011f5e40`; a failed allocation stores
/// null.
pub fn fn_00a8fd80(e: &mut Engine) {
    let block = e.call(ALLOCATE_BLOCK, &args![0x14u32]).u32();
    if block == 0 {
        e.mem.set_u32(MATERIAL_LIBRARY_SLOT, 0);
        return;
    }
    let object = e.call(MATERIAL_LIBRARY_CONSTRUCTOR, &args![block]).u32();
    e.mem.set_u32(MATERIAL_LIBRARY_SLOT, object);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_fd80, fn_00a8fd80())]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn stores_the_constructed_object() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        let block = e.mem.alloc(0x14);
        e.register_double(ALLOCATE_BLOCK, move |_, a| {
            assert_eq!(a, &[0x14]);
            block.into_ret()
        });
        e.register_double(MATERIAL_LIBRARY_CONSTRUCTOR, move |_, a| {
            assert_eq!(a, &[block]);
            (block + 0x10).into_ret()
        });
        e.call(0x00a8_fd80, &args![]);
        assert_eq!(e.mem.u32(MATERIAL_LIBRARY_SLOT), block + 0x10);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(MATERIAL_LIBRARY_SLOT, 0x1234);
        e.register(ALLOCATE_BLOCK, |_, _| 0u32.into_ret());
        e.call(0x00a8_fd80, &args![]);
        assert_eq!(e.mem.u32(MATERIAL_LIBRARY_SLOT), 0);
    }
}
