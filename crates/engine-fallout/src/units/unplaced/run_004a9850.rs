//! `(unplaced)/run_004a9850` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the destructor stores (`0101ecec`; RTTI
/// `.?AV?$NiTMap@PAVNiTexture@@PAUTEX_USER_DATA@@@@`).
const TEXTURE_USER_DATA_MAP_VTABLE: u32 = 0x0101_ecec;
/// `00438af0` (unnamed; `thiscall`, no stack words): the map body's clean-up the
/// destructor runs first, and `004a98b0` (unnamed; `thiscall`, no stack words): the base
/// destructor it runs after.
const MAP_CLEAN_UP: u32 = 0x0043_8af0;
const MAP_BASE_DESTRUCT: u32 = 0x004a_98b0;

// Translated from 004a9850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a9850 - the FS:[0] exception frame (unwind state 0 for the base destructor) is not translated
/// `NiTMap<NiTexture*, TEX_USER_DATA*>`'s destructor (by its vtable's RTTI; the engine map
/// does not name it; `thiscall`, no stack words): stores the class's vtable (`0101ecec`),
/// runs `00438af0`, then the base destructor `004a98b0`.
pub fn fn_004a9850(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TEXTURE_USER_DATA_MAP_VTABLE);
    e.call(MAP_CLEAN_UP, &args![this]);
    e.call(MAP_BASE_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004a_9850, fn_004a9850(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructor_sets_the_vtable_then_cleans_up_and_destroys_the_base() {
        let mut e = Engine::new();
        e.register(MAP_CLEAN_UP, |e, a| {
            assert_eq!(e.mem.u32(a[0]), TEXTURE_USER_DATA_MAP_VTABLE);
            Ret::default()
        });
        e.register(MAP_BASE_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x004a_9850, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(MAP_CLEAN_UP, vec![this]), (MAP_BASE_DESTRUCT, vec![this])]
        );
    }
}
