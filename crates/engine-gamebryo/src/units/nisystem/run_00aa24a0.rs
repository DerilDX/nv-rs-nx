//! `NiSystem/run_00aa24a0` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable both functions store (`010a2554`; its RTTI type descriptor reads
/// `.?AVNiSearchPath@@`).
const NI_SEARCH_PATH_VTABLE: u32 = 0x010a_2554;

// Translated from 00aa24d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aa24a0`): the `NiSearchPath` constructor body (the vtable
/// `010a2554` is `NiSearchPath`'s by its RTTI), `thiscall`, no stack words. Stores the vtable,
/// clears `m_uiNextPath` (+0x4, Xbox PDB) and the first bytes of `m_acFilePath` (+0x8) and
/// `m_acReferencePath` (+0x10c); answers `this`.
pub fn fn_00aa24d0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_SEARCH_PATH_VTABLE);
    e.mem.set_u32(base + 0x4, 0);
    e.mem.set_u8(base + 0x8, 0);
    e.mem.set_u8(base + 0x10c, 0);
    this
}

// Translated from 00aa24f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aa24a0`): the `NiSearchPath` destructor body (`thiscall`,
/// no stack words): stores the class's vtable `010a2554` and nothing else.
pub fn fn_00aa24f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_SEARCH_PATH_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_24d0, fn_00aa24d0(Ptr) -> Ptr),
        entry!(0x00aa_24f0, fn_00aa24f0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00aa24d0_sets_the_vtable_and_clears_the_paths() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x210);
        for offset in [0x4u32, 0x8, 0x9, 0x10c, 0x10d] {
            e.mem.set_u8(this + offset, 0x55);
        }
        assert_eq!(e.call(0x00aa_24d0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_SEARCH_PATH_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u8(this + 8), 0);
        assert_eq!(e.mem.u8(this + 9), 0x55);
        assert_eq!(e.mem.u8(this + 0x10c), 0);
        assert_eq!(e.mem.u8(this + 0x10d), 0x55);
    }

    #[test]
    fn fn_00aa24f0_only_stores_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x210);
        e.mem.set_u32(this, 0x1234);
        e.mem.set_u32(this + 4, 7);
        e.call(0x00aa_24f0, &args![this]);
        assert_eq!(e.mem.u32(this), NI_SEARCH_PATH_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 7);
    }
}
