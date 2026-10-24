//! `BSSystem/run_00b027c0` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable `00b029a0` stores (`010a48e8`).
const VTABLE_010A48E8: u32 = 0x010a_48e8;

// Translated from 00b029a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, no stack words): a destructor body that only stores
/// the vtable `010a48e8` at +0 (EAX is left holding `this`).
pub fn fn_00b029a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_010A48E8);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b0_29a0, fn_00b029a0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod n083_tests {
    use super::*;

    #[test]
    fn stores_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 5);
        assert_eq!(e.call(0x00b0_29a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010A48E8);
        assert_eq!(e.mem.u32(this + 4), 5);
    }
}
