//! `fallout shared/run_004a4810` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Setting` (Xbox PDB) base constructor for a word value (`00404920`).
const SETTING_CONSTRUCT_WORD: u32 = 0x0040_4920;
/// The vtable of the third collection's settings and that collection
/// (`0045d180`).
const PREF_SETTING_VTABLE: u32 = 0x0101_84fc;
const PREF_SETTINGS: u32 = 0x0045_d180;
/// Slot of the collection's vtable that adds a setting.
const COLLECTION_ADD_SLOT: u32 = 0x4;

// Translated from 004a4a40 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor of a word setting of the third collection (`thiscall`,
/// `RET 8`): the word base constructor (`00404920`), the vtable `010184fc`,
/// then adds itself to the collection (`0045d180`, vtable slot +4). Returns
/// `this`. The exception-unwinding frame is not translated.
pub fn fn_004a4a40(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    e.call(SETTING_CONSTRUCT_WORD, &args![this, name, value]);
    e.mem.set_u32(this.addr(), PREF_SETTING_VTABLE);
    let collection = e.call(PREF_SETTINGS, &[]).u32();
    e.vcall(collection, COLLECTION_ADD_SLOT, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004a4a40, fn_004a4a40(Ptr, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_word_setting_adds_itself_to_the_third_collection() {
        let mut e = Engine::new();
        let collection = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000]);
        e.mem.set_u32(collection, 0x0090_0000);
        e.register_double(PREF_SETTINGS, move |_, _| collection.into_ret());
        e.register_double(0x0090_1000, |e, a| {
            assert_eq!(e.mem.u32(a[1]), PREF_SETTING_VTABLE);
            Ret::default()
        });
        let s = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004a4a40, &[s, 0x44, 3]).u32(), s);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (SETTING_CONSTRUCT_WORD, vec![s, 0x44, 3]));
        assert_eq!(log[3], (0x0090_1000, vec![collection, s]));
    }
}
