//! `BSShader/run_00baa0e0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
/// This effect's vtable (`010b895c`; no RTTI locator before it).
const VTABLE_010B895C: u32 = 0x010b_895c;

// Translated from 00baa160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00baa0e0`): an image-space effect's constructor,
/// `thiscall`, no stack words: the base constructor `00ba4080`, then the vtable `010b895c`.
/// Answers `this`.
pub fn fn_00baa160(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_010B895C);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ba_a160, fn_00baa160(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00baa160_runs_the_base_and_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |e, a| {
            e.mem.set_u32(a[0], 1);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x58);
        assert_eq!(e.call(0x00ba_a160, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B895C);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(EFFECT_CONSTRUCT_00BA4080, vec![this])]
        );
    }
}
