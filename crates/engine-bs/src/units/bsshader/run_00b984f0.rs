//! `BSShader/run_00b984f0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the 0x14-byte list element `00b988c0` constructs (the
/// destructor `00b656b0` sets the same one).
const PERSISTENT_LIST_VTABLE: u32 = 0x010b_7dc0;

// Translated from 00b988c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of one `BSTPersistentList` element of `BSShaderAccumulator`'s
/// rendered-geometry caches (Ghidra made no function here; read from the
/// bytes): sets the vtable and clears the words at +0x04, +0x08 and +0x0C.
/// Returns `this`.
pub fn fn_00b988c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, PERSISTENT_LIST_VTABLE);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b9_88c0, fn_00b988c0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_constructor_sets_the_vtable_and_clears_three_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        for off in (0..0x14).step_by(4) {
            e.mem.set_u32(this + off, 0xdead_beef);
        }
        assert_eq!(e.call(0x00b9_88c0, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), PERSISTENT_LIST_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // +0x10 is not touched.
        assert_eq!(e.mem.u32(this + 0x10), 0xdead_beef);
    }
}
