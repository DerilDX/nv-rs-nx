//! `fallout shared/magic/run_00402bc0` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `0048e270` (a `BSStringT` at `+4`, per the Xbox PDB's
/// `TESTexture`).
const BASE_CONSTRUCT: u32 = 0x0048_e270;
/// This class's vtable.
const VTABLE: u32 = 0x0101_1cd4;

// Translated from 00402c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `TESIcon` (the engine map gives it no name; the Xbox
/// PDB's `TESIcon` is a `TESTexture` and `TESQuest` constructs it at +0x24):
/// runs the base constructor `0048e270`, then sets its own vtable. Returns
/// `this`.
pub fn fn_00402c20(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00402c20, fn_00402c20(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_the_base_constructor_then_sets_the_vtable() {
        let mut e = Engine::new();
        // The base constructor sets its own vtable first; the derived one wins.
        e.register(BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xdead);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.mem.alloc(0x10);
        let back = e.call(0x0040_2c20, &args![object]).u32();
        assert_eq!(back, object);
        assert_eq!(e.mem.u32(object), VTABLE);
    }
}
