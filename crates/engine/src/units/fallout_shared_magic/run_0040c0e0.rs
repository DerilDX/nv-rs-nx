//! `fallout shared/magic/run_0040c0e0` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Setting` (Xbox PDB) base constructor for a word value (`00404920`).
const SETTING_CONSTRUCT_WORD: u32 = 0x0040_4920;
/// The game-setting vtable.
const GAME_SETTING_VTABLE: u32 = 0x0101_2114;
/// The game-setting collection (`00404a70`: created on first use).
const GAME_SETTINGS: u32 = 0x0040_4a70;
/// Slot of the collection's vtable that adds a setting.
const COLLECTION_ADD_SLOT: u32 = 0x4;

// Translated from 0040c150 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor of a game setting with a word value (an integer, a
/// string pointer; `thiscall`, `RET 8`): the base constructor (`00404920`)
/// with the name and the value, the game-setting vtable (`01012114`), then
/// adds itself to the game-setting collection (`00404a70`, vtable slot +4).
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn fn_0040c150(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    e.call(SETTING_CONSTRUCT_WORD, &args![this, name, value]);
    e.mem.set_u32(this.addr(), GAME_SETTING_VTABLE);
    let collection = e.call(GAME_SETTINGS, &[]).u32();
    e.vcall(collection, COLLECTION_ADD_SLOT, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0040c150, fn_0040c150(Ptr, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_word_game_setting_adds_itself_to_the_collection() {
        let mut e = Engine::new();
        let collection = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000]);
        e.mem.set_u32(collection, 0x0090_0000);
        e.register_double(GAME_SETTINGS, move |_, _| collection.into_ret());
        e.register_double(0x0090_1000, |e, a| {
            assert_eq!(e.mem.u32(a[1]), GAME_SETTING_VTABLE);
            Ret::default()
        });
        let s = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040c150, &[s, 0x66, 7]).u32(), s);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (SETTING_CONSTRUCT_WORD, vec![s, 0x66, 7]));
        assert_eq!(log[2].0, GAME_SETTINGS);
        assert_eq!(log[3], (0x0090_1000, vec![collection, s]));
        assert_eq!(e.mem.u32(s + 4), 7);
        assert_eq!(e.mem.u32(s + 8), 0x66);
    }
}
