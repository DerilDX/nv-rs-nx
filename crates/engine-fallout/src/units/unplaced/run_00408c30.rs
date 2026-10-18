//! `(unplaced)/run_00408c30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// A static byte the null case writes zero to and returns the address of.
const EMPTY_BYTE: u32 = 0x0120_2800;

// Translated from 00408d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the field at `+4` of `this`; for a null `this`, writes 0 to
/// the static byte at `01202800` and returns its address instead.
pub fn fn_00408d60(e: &mut Engine, this: Ptr) -> Ptr {
    if this.is_null() {
        e.mem.set_u8(EMPTY_BYTE, 0);
        Ptr::new(EMPTY_BYTE)
    } else {
        Ptr::new(this.addr() + 4)
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00408d60, fn_00408d60(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_at_the_field_at_4() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x10));
        assert_eq!(e.call(0x0040_8d60, &args![object]).u32(), object.addr() + 4);
    }

    #[test]
    fn null_this_gives_the_zeroed_static_byte() {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        e.mem.set_u8(EMPTY_BYTE, 9);
        assert_eq!(
            e.call(0x0040_8d60, &args![Ptr::<()>::NULL]).u32(),
            EMPTY_BYTE
        );
        assert_eq!(e.mem.u8(EMPTY_BYTE), 0);
    }
}
