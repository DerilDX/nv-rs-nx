//! `fallout shared/run_005e28b0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The string constructor run on `this + 0x18` (`004037b0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;

// Translated from 005e28b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an object of the `ScriptVariable` layout (Xbox PDB: a
/// `SCRIPT_LOCAL` at `+0`, a `BSStringT` name at `+0x18`): zeroes the data
/// ([`fn_005e28e0`]) and constructs the string at `+0x18` (`004037b0`).
/// Returns `this`. The engine map has no name for it (the decompiler's
/// `SafeSQueue` is a folded library name); the stage item loader
/// `0060f6a0` calls it.
pub fn fn_005e28b0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_005e28e0(e, this);
    e.call(STRING_CONSTRUCT, &args![this.addr() + 0x18]);
    this
}

// Translated from 005e28e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Zeroes a `SCRIPT_LOCAL` (Xbox PDB layout): `uiID` (`+0`), the double
/// `fValue` (`+8`, stored from `FLDZ`, all bits zero) and `bIsInteger`
/// (`+0x10`). Returns `this`.
pub fn fn_005e28e0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u8(base + 0x10, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e28b0, fn_005e28b0(Ptr) -> Ptr),
        entry!(0x005e28e0, fn_005e28e0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeroes_the_data_fields_only() {
        let mut e = Engine::new();
        let local = e.mem.alloc(0x18);
        for offset in (0..0x18).step_by(4) {
            e.mem.set_u32(local + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x005e_28e0, &[local]).u32(), local);
        assert_eq!(e.mem.u32(local), 0);
        // The padding word at +4 and the three bytes after the bool stay.
        assert_eq!(e.mem.u32(local + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(local + 8), 0);
        assert_eq!(e.mem.u32(local + 0xc), 0);
        assert_eq!(e.mem.u32(local + 0x10), 0xffff_ff00);
        assert_eq!(e.mem.u32(local + 0x14), 0xffff_ffff);
    }

    #[test]
    fn constructor_zeroes_the_data_and_builds_the_name() {
        let mut e = Engine::new();
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        let variable = e.mem.alloc(0x20);
        e.mem.set_u32(variable, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_28b0, &[variable]).u32(), variable);
        assert_eq!(e.mem.u32(variable), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(STRING_CONSTRUCT, vec![variable + 0x18])]);
    }
}
