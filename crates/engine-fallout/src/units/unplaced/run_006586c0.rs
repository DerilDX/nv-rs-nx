//! `(unplaced)/run_006586c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00658930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the 16-bit field at `+0xa` of `this` (zero-extended into `EAX`).
pub fn fn_00658930(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0xa)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00658930, fn_00658930(Ptr) -> u16)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_field_at_offset_0xa() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x50));
        e.mem.set_u16(object.addr() + 0xa, 0x1234);
        assert_eq!(e.call(0x0065_8930, &args![object]).u16(), 0x1234);
    }
}
