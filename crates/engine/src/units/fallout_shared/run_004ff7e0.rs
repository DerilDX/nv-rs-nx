//! `fallout shared/run_004ff7e0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the constructor installs (Ghidra labels `0102583c` `SaveGame`).
const SAVE_GAME_VTABLE: u32 = 0x0102_583c;

// Translated from 004ff7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall`: stores vtable `0102583c` at `this + 0` and
/// returns `this` (EAX still holds it).
pub fn fn_004ff7e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), SAVE_GAME_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004ff7e0, fn_004ff7e0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installs_the_vtable_and_returns_this() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(8));
        assert_eq!(e.call(0x004f_f7e0, &args![this]).ptr::<()>(), this);
        assert_eq!(e.mem.u32(this.addr()), SAVE_GAME_VTABLE);
    }
}
