//! `(unplaced)/run_00619a70` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00619a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit word at `this + 0x20` (returned in `AX`). The engine map
/// gives it no name.
pub fn fn_00619a70(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0x20)
}

// Translated from 00619b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 of the byte at `this + 0x25` is set. The engine map gives
/// it no name.
pub fn fn_00619b20(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x25) & 4 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_9a70, fn_00619a70(Ptr) -> u16),
        entry!(0x0061_9b20, fn_00619b20(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_readers() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0x20, 0x00fb_fffe);
        assert_eq!(e.call(0x0061_9a70, &args![this]).u16(), 0xfffe);
        assert!(!e.call(0x0061_9b20, &args![this]).bool());
        e.mem.set_u8(this + 0x25, 0x04);
        assert!(e.call(0x0061_9b20, &args![this]).bool());
    }
}
