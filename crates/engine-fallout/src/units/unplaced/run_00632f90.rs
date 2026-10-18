//! `(unplaced)/run_00632f90` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The word `Setting` base constructor (`00404920`), this group's setting
/// vtable and its collection (`00632e20`).
const SETTING_CONSTRUCT_WORD: u32 = 0x0040_4920;
const SETTING_VTABLE: u32 = 0x0104_e6e8;
const COLLECTION: u32 = 0x0063_2e20;
/// Slot of a settings collection's vtable that adds a setting.
const COLLECTION_ADD_SLOT: u32 = 0x4;

// Translated from 00633000 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor of a word setting of the `00632e20` collection
/// (`thiscall`, `RET 8`): the word base constructor (`00404920`), the vtable
/// `0104e6e8`, then adds itself to the collection (vtable slot +4). Returns
/// `this`. The exception-unwinding frame is not translated.
pub fn fn_00633000(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    e.call(SETTING_CONSTRUCT_WORD, &args![this, name, value]);
    e.mem.set_u32(this.addr(), SETTING_VTABLE);
    let collection = e.call(COLLECTION, &[]).u32();
    e.vcall(collection, COLLECTION_ADD_SLOT, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00633000, fn_00633000(Ptr, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_word_setting_joins_its_collection() {
        let mut e = Engine::new();
        let c = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000]);
        e.mem.set_u32(c, 0x0090_0000);
        e.register_double(COLLECTION, move |_, _| c.into_ret());
        e.register(0x0090_1000, |_, _| Ret::default());
        let s = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        e.call(0x00633000, &[s, 0x20, 9]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (SETTING_CONSTRUCT_WORD, vec![s, 0x20, 9]));
        assert_eq!(e.mem.u32(s), SETTING_VTABLE);
        assert_eq!(log.last().unwrap(), &(0x0090_1000, vec![c, s]));
    }
}
