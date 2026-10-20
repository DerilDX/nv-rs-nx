//! `fallout shared/run_005e4970` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e4ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks `this` through its vtable slot +0xB4 (with the address of a
/// one-byte local) and answers the result moved back by 4 bytes (a base
/// class at +4 cast to the whole object), or null for a null result (the
/// engine map has no name). The byte local, uninitialized in the exe, is
/// zero here.
pub fn fn_005e4ab0(e: &mut Engine, this: Ptr) -> u32 {
    let result = e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]).u32());
    if result != 0 {
        result.wrapping_sub(4)
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005e_4ab0, fn_005e4ab0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_b4_result_is_moved_back_four_bytes() {
        const TARGET: u32 = 0x0300_0000;
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0xb8);
        e.mem.set_u32(vtable + 0xb4, TARGET);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, vtable);
        e.register(TARGET, |_, _| 0x1004u32.into_ret());
        assert_eq!(e.call(0x005e_4ab0, &args![this]).u32(), 0x1000);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(0x005e_4ab0, &args![this]).u32(), 0);
    }
}
